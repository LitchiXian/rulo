use rulo_common::{
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, query, query_as};

use crate::system::role::model::{SysRole, SysRoleListDto, SysRoleSaveDto, SysRoleUpdateDto};

pub async fn save_handle(pool: &PgPool, dto: &SysRoleSaveDto) -> R<SysRole> {
    let new_role = SysRole::new_role_from_save_dto(&dto);
    query!(
        "insert into sys_role(
        id, role_name, role_key, is_super, is_active,
        is_deleted, create_id, create_time, update_id, update_time, remark
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        new_role.id,
        &new_role.role_name,
        &new_role.role_key,
        new_role.is_super,
        new_role.is_active,
        new_role.is_deleted,
        new_role.create_id,
        new_role.create_time,
        new_role.update_id,
        new_role.update_time,
        new_role.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(new_role)
}

pub async fn remove_handle(pool: &PgPool, dto: &IdsDto) -> R<()> {
    sqlx::query!("DELETE FROM sys_role WHERE id = ANY($1)", &dto.ids)
        .execute(pool)
        .await?;
    success(())
}

pub async fn update_handle(pool: &PgPool, dto: &SysRoleUpdateDto) -> R<()> {
    sqlx::query!(
        "UPDATE sys_role SET
            role_name = COALESCE($2, role_name),
            role_key = COALESCE($3, role_key),
            is_active = COALESCE($4, is_active),
            remark = COALESCE($5, remark),
            update_time = now()
        WHERE id = $1",
        dto.id,
        dto.role_name.as_deref(),
        dto.role_key.as_deref(),
        dto.is_active,
        dto.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn get_one_handle(pool: &PgPool, dto: &IdDto) -> R<SysRole> {
    let data = query_as!(SysRole, "select * from sys_role where id = $1", dto.id)
        .fetch_one(pool)
        .await?;
    success(data)
}

pub async fn list_handle(pool: &PgPool, _dto: &SysRoleListDto) -> R<Vec<SysRole>> {
    let data = query_as!(SysRole, "select * from sys_role")
        .fetch_all(pool)
        .await?;
    success(data)
}
