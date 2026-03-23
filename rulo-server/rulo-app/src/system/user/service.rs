use rulo_common::{
    error::AppError,
    model::{IdDto, IdsDto, PageResult, normalize_page},
    result::{R, success},
};
use sqlx::{PgPool, Postgres, QueryBuilder, query, query_as, query_scalar};

use crate::system::user::model::{
    BindRolesDto, SysUser, SysUserListDto, SysUserSaveDto, SysUserUpdateDto,
};

pub async fn save(pool: &PgPool, dto: &SysUserSaveDto) -> R<SysUser> {
    if dto.nick_name.trim().is_empty() {
        return Err(AppError::ServiceError("昵称不能为空".to_string()));
    }
    if dto.password.trim().is_empty() {
        return Err(AppError::ServiceError("密码不能为空".to_string()));
    }
    let new_user = SysUser::new_user_from_save_dto(&dto);
    query!(
        "insert into sys_user(
        id, user_name, nick_name, password, email,
        is_active, is_deleted, create_id, create_time,
         update_id, update_time, remark
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
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
        new_user.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(new_user)
}

pub async fn remove(pool: &PgPool, dto: &IdsDto) -> R<()> {
    sqlx::query!(
        "UPDATE sys_user SET is_deleted = true, update_time = now() WHERE id = ANY($1)",
        &dto.ids
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn update(pool: &PgPool, dto: &SysUserUpdateDto) -> R<()> {
    if let Some(ref nick_name) = dto.nick_name {
        if nick_name.trim().is_empty() {
            return Err(AppError::ServiceError("昵称不能为空字符串".to_string()));
        }
    }
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
        dto.password.as_deref(),
        dto.email.as_deref(),
        dto.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn update_bind_roles(pool: &PgPool, dto: &BindRolesDto) -> R<()> {
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

pub async fn detail(pool: &PgPool, dto: &IdDto) -> R<SysUser> {
    let data = query_as!(SysUser, "select * from sys_user where id = $1 AND is_deleted = false", dto.id)
        .fetch_one(pool)
        .await?;
    success(data)
}

pub async fn list(pool: &PgPool, dto: &SysUserListDto) -> R<PageResult<SysUser>> {
    let (page_num, page_size) = normalize_page(dto.page_num, dto.page_size);
    let offset = ((page_num - 1) * page_size) as i64;

    let mut count_builder = QueryBuilder::<Postgres>::new(
        "SELECT COUNT(*)::bigint FROM sys_user WHERE is_deleted = false",
    );
    append_user_filters(&mut count_builder, dto);
    let total = count_builder.build_query_scalar::<i64>().fetch_one(pool).await? as u64;

    let mut data_builder = QueryBuilder::<Postgres>::new(
        "SELECT * FROM sys_user WHERE is_deleted = false",
    );
    append_user_filters(&mut data_builder, dto);
    data_builder
        .push(" ORDER BY update_time DESC")
        .push(" LIMIT ")
        .push_bind(page_size as i64)
        .push(" OFFSET ")
        .push_bind(offset);

    let list = data_builder
        .build_query_as::<SysUser>()
        .fetch_all(pool)
        .await?;

    success(PageResult {
        list,
        total,
        page_num,
        page_size,
    })
}

fn append_user_filters(builder: &mut QueryBuilder<Postgres>, dto: &SysUserListDto) {
    if let Some(nick_name) = dto.nick_name.as_deref().filter(|value| !value.trim().is_empty()) {
        builder.push(" AND nick_name ILIKE ").push_bind(format!("%{}%", nick_name.trim()));
    }
    if let Some(email) = dto.email.as_deref().filter(|value| !value.trim().is_empty()) {
        builder.push(" AND email ILIKE ").push_bind(format!("%{}%", email.trim()));
    }
    if let Some(remark) = dto.remark.as_deref().filter(|value| !value.trim().is_empty()) {
        builder.push(" AND remark ILIKE ").push_bind(format!("%{}%", remark.trim()));
    }
    if let Some(create_start_time) = dto.create_start_time {
        builder.push(" AND create_time >= ").push_bind(create_start_time);
    }
    if let Some(create_end_time) = dto.create_end_time {
        builder.push(" AND create_time <= ").push_bind(create_end_time);
    }
}

pub async fn list_bind_roles(pool: &PgPool, user_id: i64) -> R<Vec<i64>> {
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
