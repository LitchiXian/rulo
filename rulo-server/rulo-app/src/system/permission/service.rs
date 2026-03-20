use rulo_common::{
    error::AppError,
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, query, query_as};

use crate::system::permission::model::{
    SysPermission, SysPermissionListDto, SysPermissionSaveDto, SysPermissionUpdateDto,
};

pub async fn save(pool: &PgPool, dto: &SysPermissionSaveDto) -> R<SysPermission> {
    if dto.perm_code.trim().is_empty() {
        return Err(AppError::ServiceError("权限编码不能为空".to_string()));
    }
    if dto.perm_name.trim().is_empty() {
        return Err(AppError::ServiceError("权限名称不能为空".to_string()));
    }
    if dto.perm_type != 1 && dto.perm_type != 2 {
        return Err(AppError::ServiceError("权限类型只能为 1（API权限）或 2（菜单权限）".to_string()));
    }
    let exists: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM sys_permission WHERE perm_code = $1 AND is_deleted = false",
        &dto.perm_code
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    if exists > 0 {
        return Err(AppError::ServiceError(format!("权限码 {} 已存在，请更换", dto.perm_code)));
    }
    let new_perm = SysPermission::new_permission_from_save_dto(&dto);
    query!(
        "insert into sys_permission(
        id, perm_code, perm_name, perm_type,
        is_deleted, create_id, create_time, update_id, update_time, remark
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        new_perm.id,
        &new_perm.perm_code,
        &new_perm.perm_name,
        new_perm.perm_type,
        new_perm.is_deleted,
        new_perm.create_id,
        new_perm.create_time,
        new_perm.update_id,
        new_perm.update_time,
        new_perm.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(new_perm)
}

pub async fn remove(pool: &PgPool, dto: &IdsDto) -> R<()> {
    // perm_type=2（菜单权限）不允许直接删除，只能通过删菜单级联删
    let menu_perm_count: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM sys_permission WHERE id = ANY($1) AND perm_type = 2 AND is_deleted = false",
        &dto.ids
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    if menu_perm_count > 0 {
        return Err(AppError::ServiceError("菜单权限不能直接删除，请通过删除对应菜单来级联删除".to_string()));
    }
    sqlx::query!(
        "UPDATE sys_permission SET is_deleted = true, update_time = now() WHERE id = ANY($1)",
        &dto.ids
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn update(pool: &PgPool, dto: &SysPermissionUpdateDto) -> R<()> {
    if let Some(ref perm_name) = dto.perm_name {
        if perm_name.trim().is_empty() {
            return Err(AppError::ServiceError("权限名称不能为空字符串".to_string()));
        }
    }
    sqlx::query!(
        "UPDATE sys_permission SET
            perm_name = COALESCE($2, perm_name),
            remark = COALESCE($3, remark),
            update_time = now()
        WHERE id = $1 AND is_deleted = false",
        dto.id,
        dto.perm_name.as_deref(),
        dto.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn detail(pool: &PgPool, dto: &IdDto) -> R<SysPermission> {
    let data = query_as!(
        SysPermission,
        "select * from sys_permission where id = $1 AND is_deleted = false",
        dto.id
    )
    .fetch_one(pool)
    .await?;
    success(data)
}

pub async fn list(pool: &PgPool, dto: &SysPermissionListDto) -> R<Vec<SysPermission>> {
    let data = query_as!(
        SysPermission,
        "SELECT * FROM sys_permission WHERE is_deleted = false \
         AND ($1::smallint IS NULL OR perm_type = $1) \
         AND ($2::varchar IS NULL OR perm_code ILIKE '%' || $2 || '%') \
         AND ($3::varchar IS NULL OR perm_name ILIKE '%' || $3 || '%') \
         ORDER BY perm_code ASC, update_time DESC",
        dto.perm_type,
        dto.perm_code.as_deref(),
        dto.perm_name.as_deref()
    )
    .fetch_all(pool)
    .await?;
    success(data)
}
