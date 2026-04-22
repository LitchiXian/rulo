use common::{
    config::StorageConfig,
    error::AppError,
    model::{IdDto, IdsDto, PageResult, normalize_page},
    result::{R, success},
    util::{password_util, storage_util},
};
use s3::Bucket;
use sqlx::{PgPool, Postgres, QueryBuilder, query, query_as, query_scalar};

use crate::system::user::model::{
    BindRolesDto, SysUser, SysUserListDto, SysUserSaveDto, SysUserUpdateDto,
};

pub async fn save(pool: &PgPool, dto: &SysUserSaveDto) -> R<SysUser> {
    // 检查用户名是否已存在
    let exists: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM sys_user WHERE user_name = $1 AND is_deleted = false",
        &dto.user_name
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    if exists > 0 {
        return Err(AppError::ServiceError("用户名已存在".to_string()));
    }
    // 密码加密
    let hashed_dto = SysUserSaveDto {
        user_name: dto.user_name.clone(),
        nick_name: dto.nick_name.clone(),
        password: password_util::hash_password(&dto.password)?,
        email: dto.email.clone(),
        remark: dto.remark.clone(),
    };
    let new_user = SysUser::new_user_from_save_dto(&hashed_dto);
    query!(
        "insert into sys_user(
        id, user_name, nick_name, password, email,
        is_active, is_deleted, create_id, create_time,
         update_id, update_time, remark, avatar_url
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        new_user.id,
        &new_user.user_name,
        &new_user.nick_name,
        &new_user.password,
        new_user.email.as_deref(),
        new_user.is_active,
        new_user.is_deleted,
        new_user.create_id,
        new_user.create_time,
        new_user.update_id,
        new_user.update_time,
        new_user.remark.as_deref(),
        new_user.avatar_url.as_deref()
    )
    .execute(pool)
    .await?;
    success(new_user)
}

pub async fn remove(pool: &PgPool, dto: &IdsDto, caller_is_super: bool) -> R<()> {
    if !caller_is_super {
        ensure_no_super_admin_target(pool, &dto.ids).await?;
    }
    let result = sqlx::query!(
        "UPDATE sys_user SET is_deleted = true, update_time = now() WHERE id = ANY($1)",
        &dto.ids
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("用户不存在或已删除".to_string()));
    }
    success(())
}

/// 校验目标用户中不包含超级管理员（用于非超管操作者的写入操作保护）
async fn ensure_no_super_admin_target(pool: &PgPool, user_ids: &[i64]) -> Result<(), AppError> {
    if user_ids.is_empty() {
        return Ok(());
    }
    let cnt: i64 = query_scalar!(
        "SELECT COUNT(*) FROM sys_user_role ur \
         JOIN sys_role r ON r.id = ur.role_id \
         WHERE ur.user_id = ANY($1) AND r.is_super = true AND r.is_deleted = false",
        user_ids
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    if cnt > 0 {
        return Err(AppError::Forbidden("无权操作超级管理员用户".to_string()));
    }
    Ok(())
}

pub async fn update(
    pool: &PgPool,
    storage_config: &StorageConfig,
    dto: &SysUserUpdateDto,
    caller_is_super: bool,
) -> R<()> {
    if !caller_is_super {
        ensure_no_super_admin_target(pool, std::slice::from_ref(&dto.id)).await?;
    }
    // avatar_url 特殊处理：None=不修改, Some("")=清空, Some(url)=更新
    let avatar_url_value: Option<Option<String>> = match &dto.avatar_url {
        None => None,                              // 不修改
        Some(url) if url.is_empty() => Some(None), // 清空 → NULL
        Some(url) => Some(Some(storage_util::extract_object_key(storage_config, url))),
    };

    // 密码加密处理
    let hashed_password = match &dto.password {
        Some(pwd) => Some(password_util::hash_password(pwd)?),
        None => None,
    };

    // 对 avatar_url 使用独立的 SQL 分支，避免 COALESCE 无法区分"不传"和"清空"
    let result = match avatar_url_value {
        None => {
            sqlx::query!(
                "UPDATE sys_user SET 
                    nick_name = COALESCE($2, nick_name),
                    password = COALESCE($3, password),
                    email = COALESCE($4, email),
                    remark = COALESCE($5, remark),
                    update_time = now() 
                WHERE id = $1 AND is_deleted = false",
                dto.id,
                dto.nick_name.as_deref(),
                hashed_password.as_deref(),
                dto.email.as_deref(),
                dto.remark.as_deref(),
            )
            .execute(pool)
            .await?
        }
        Some(url) => {
            sqlx::query!(
                "UPDATE sys_user SET 
                    nick_name = COALESCE($2, nick_name),
                    password = COALESCE($3, password),
                    email = COALESCE($4, email),
                    remark = COALESCE($5, remark),
                    avatar_url = $6,
                    update_time = now() 
                WHERE id = $1 AND is_deleted = false",
                dto.id,
                dto.nick_name.as_deref(),
                hashed_password.as_deref(),
                dto.email.as_deref(),
                dto.remark.as_deref(),
                url.as_deref(),
            )
            .execute(pool)
            .await?
        }
    };
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("用户不存在或已删除".to_string()));
    }
    success(())
}

pub async fn update_bind_roles(
    pool: &PgPool,
    dto: &BindRolesDto,
    caller_is_super: bool,
) -> R<()> {
    if !caller_is_super {
        // 1) 不允许操作超管用户
        ensure_no_super_admin_target(pool, std::slice::from_ref(&dto.user_id)).await?;
        // 2) 不允许把超管角色分配给任何人
        if !dto.role_ids.is_empty() {
            let super_cnt: i64 = query_scalar!(
                "SELECT COUNT(*) FROM sys_role WHERE id = ANY($1) AND is_super = true AND is_deleted = false",
                &dto.role_ids
            )
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
            if super_cnt > 0 {
                return Err(AppError::Forbidden("无权分配超级管理员角色".to_string()));
            }
        }
    }
    // 校验目标角色是否存在
    if !dto.role_ids.is_empty() {
        let valid_count: i64 = query_scalar!(
            "SELECT COUNT(*) FROM sys_role WHERE id = ANY($1) AND is_deleted = false",
            &dto.role_ids
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);
        if valid_count != dto.role_ids.len() as i64 {
            return Err(AppError::ServiceError("部分角色不存在或已删除".to_string()));
        }
    }
    let mut tx = pool.begin().await?;
    query!("DELETE FROM sys_user_role WHERE user_id = $1", dto.user_id)
        .execute(&mut *tx)
        .await?;
    for role_id in &dto.role_ids {
        query!(
            "INSERT INTO sys_user_role (user_id, role_id) VALUES ($1, $2)",
            dto.user_id,
            role_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    success(())
}

pub async fn detail(
    pool: &PgPool,
    bucket: &Bucket,
    storage_config: &StorageConfig,
    dto: &IdDto,
    caller_is_super: bool,
) -> R<SysUser> {
    if !caller_is_super {
        ensure_no_super_admin_target(pool, std::slice::from_ref(&dto.id)).await?;
    }
    let mut data = query_as!(
        SysUser,
        "select * from sys_user where id = $1 AND is_deleted = false",
        dto.id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
    data.avatar_url =
        storage_util::resolve_object_url(bucket, storage_config, data.avatar_url.as_deref())
            .await
            .map_err(AppError::Internal)?;
    success(data)
}

pub async fn list(
    pool: &PgPool,
    bucket: &Bucket,
    storage_config: &StorageConfig,
    dto: &SysUserListDto,
    caller_is_super: bool,
) -> R<PageResult<SysUser>> {
    let (page_num, page_size) = normalize_page(dto.page_num, dto.page_size);
    let offset = ((page_num - 1) * page_size) as i64;

    // 非超管操作者：从结果中排除拥有超管角色的用户
    let exclude_super_sql = " AND id NOT IN (SELECT ur.user_id FROM sys_user_role ur JOIN sys_role r ON r.id = ur.role_id WHERE r.is_super = true AND r.is_deleted = false)";

    let mut count_builder = QueryBuilder::<Postgres>::new(
        "SELECT COUNT(*)::bigint FROM sys_user WHERE is_deleted = false",
    );
    if !caller_is_super {
        count_builder.push(exclude_super_sql);
    }
    append_user_filters(&mut count_builder, dto);
    let total = count_builder
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await? as u64;

    let mut data_builder =
        QueryBuilder::<Postgres>::new("SELECT * FROM sys_user WHERE is_deleted = false");
    if !caller_is_super {
        data_builder.push(exclude_super_sql);
    }
    append_user_filters(&mut data_builder, dto);
    data_builder
        .push(" ORDER BY update_time DESC")
        .push(" LIMIT ")
        .push_bind(page_size as i64)
        .push(" OFFSET ")
        .push_bind(offset);

    let mut list = data_builder
        .build_query_as::<SysUser>()
        .fetch_all(pool)
        .await?;

    for user in &mut list {
        user.avatar_url =
            storage_util::resolve_object_url(bucket, storage_config, user.avatar_url.as_deref())
                .await
                .map_err(AppError::Internal)?;
    }

    success(PageResult {
        list,
        total,
        page_num,
        page_size,
    })
}

fn append_user_filters(builder: &mut QueryBuilder<Postgres>, dto: &SysUserListDto) {
    if let Some(nick_name) = dto
        .nick_name
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        builder
            .push(" AND nick_name ILIKE ")
            .push_bind(format!("%{}%", nick_name.trim()));
    }
    if let Some(email) = dto
        .email
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        builder
            .push(" AND email ILIKE ")
            .push_bind(format!("%{}%", email.trim()));
    }
    if let Some(remark) = dto
        .remark
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        builder
            .push(" AND remark ILIKE ")
            .push_bind(format!("%{}%", remark.trim()));
    }
    if let Some(create_start_time) = dto.create_start_time {
        builder
            .push(" AND create_time >= ")
            .push_bind(create_start_time);
    }
    if let Some(create_end_time) = dto.create_end_time {
        builder
            .push(" AND create_time <= ")
            .push_bind(create_end_time);
    }
}

pub async fn list_bind_roles(
    pool: &PgPool,
    user_id: i64,
    caller_is_super: bool,
) -> R<Vec<i64>> {
    if !caller_is_super {
        ensure_no_super_admin_target(pool, std::slice::from_ref(&user_id)).await?;
    }
    let ids = query_scalar!(
        "SELECT ur.role_id FROM sys_user_role ur \
         JOIN sys_role r ON ur.role_id = r.id \
         WHERE ur.user_id = $1 AND r.is_deleted = false",
        user_id
    )
    .fetch_all(pool)
    .await?;
    success(ids)
}
