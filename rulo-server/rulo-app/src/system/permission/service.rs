use rulo_common::{
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, query, query_as};

use crate::system::permission::model::{
    SysPermission, SysPermissionListDto, SysPermissionSaveDto, SysPermissionUpdateDto,
};

pub async fn save(pool: &PgPool, dto: &SysPermissionSaveDto) -> R<SysPermission> {
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
    sqlx::query!("DELETE FROM sys_permission WHERE id = ANY($1)", &dto.ids)
        .execute(pool)
        .await?;
    success(())
}

pub async fn update(pool: &PgPool, dto: &SysPermissionUpdateDto) -> R<()> {
    sqlx::query!(
        "UPDATE sys_permission SET
            perm_name = COALESCE($2, perm_name),
            perm_type = COALESCE($3, perm_type),
            remark = COALESCE($4, remark),
            update_time = now()
        WHERE id = $1",
        dto.id,
        dto.perm_name.as_deref(),
        dto.perm_type,
        dto.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn detail(pool: &PgPool, dto: &IdDto) -> R<SysPermission> {
    let data = query_as!(
        SysPermission,
        "select * from sys_permission where id = $1",
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
         AND ($3::varchar IS NULL OR perm_name ILIKE '%' || $3 || '%')",
        dto.perm_type,
        dto.perm_code.as_deref(),
        dto.perm_name.as_deref()
    )
    .fetch_all(pool)
    .await?;
    success(data)
}
