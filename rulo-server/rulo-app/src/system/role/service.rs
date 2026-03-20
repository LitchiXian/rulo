use rulo_common::{
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, query, query_as, query_scalar};

use crate::system::role::model::{
    BindMenusDto, BindPermsDto, SysRole, SysRoleListDto, SysRoleSaveDto, SysRoleUpdateDto,
};

pub async fn save(pool: &PgPool, dto: &SysRoleSaveDto) -> R<SysRole> {
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

pub async fn remove(pool: &PgPool, dto: &IdsDto) -> R<()> {
    sqlx::query!("DELETE FROM sys_role WHERE id = ANY($1)", &dto.ids)
        .execute(pool)
        .await?;
    success(())
}

pub async fn update(pool: &PgPool, dto: &SysRoleUpdateDto) -> R<()> {
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

pub async fn update_bind_menus(pool: &PgPool, dto: &BindMenusDto) -> R<()> {
    // 把 menu_ids 转换为 perm_ids（只取有关联权限的菜单）
    let perm_ids: Vec<i64> = query_scalar!(
        "SELECT DISTINCT perm_id FROM sys_menu WHERE id = ANY($1) AND perm_id IS NOT NULL",
        &dto.menu_ids
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .flatten()
    .collect();

    let mut tx = pool.begin().await?;
    // 只删除 perm_type=2（菜单权限）的绑定，不影响 API 权限
    query!(
        "DELETE FROM sys_role_permission WHERE role_id = $1 AND perm_id IN (SELECT id FROM sys_permission WHERE perm_type = 2)",
        dto.role_id
    )
    .execute(&mut *tx)
    .await?;
    for perm_id in &perm_ids {
        query!(
            "INSERT INTO sys_role_permission (role_id, perm_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            dto.role_id,
            perm_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    success(())
}

pub async fn update_bind_perms(pool: &PgPool, dto: &BindPermsDto) -> R<()> {
    let mut tx = pool.begin().await?;
    // 只删除 perm_type=1（API权限）的绑定，不影响菜单权限
    query!(
        "DELETE FROM sys_role_permission WHERE role_id = $1 AND perm_id IN (SELECT id FROM sys_permission WHERE perm_type = 1)",
        dto.role_id
    )
    .execute(&mut *tx)
    .await?;
    for perm_id in &dto.perm_ids {
        query!(
            "INSERT INTO sys_role_permission (role_id, perm_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            dto.role_id,
            perm_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    success(())
}

pub async fn detail(pool: &PgPool, dto: &IdDto) -> R<SysRole> {
    let data = query_as!(SysRole, "select * from sys_role where id = $1", dto.id)
        .fetch_one(pool)
        .await?;
    success(data)
}

pub async fn list(pool: &PgPool, _dto: &SysRoleListDto) -> R<Vec<SysRole>> {
    let data = query_as!(SysRole, "select * from sys_role")
        .fetch_all(pool)
        .await?;
    success(data)
}

pub async fn list_bind_menus(pool: &PgPool, role_id: i64) -> R<Vec<i64>> {
    let ids = query_scalar!(
        "SELECT m.id FROM sys_menu m \
         JOIN sys_permission p ON m.perm_id = p.id \
         JOIN sys_role_permission rp ON rp.perm_id = p.id \
         WHERE rp.role_id = $1 AND p.perm_type = 2",
        role_id
    )
    .fetch_all(pool)
    .await?;
    success(ids)
}

pub async fn list_bind_perms(pool: &PgPool, role_id: i64) -> R<Vec<i64>> {
    let ids = query_scalar!(
        "SELECT rp.perm_id FROM sys_role_permission rp \
         JOIN sys_permission p ON rp.perm_id = p.id \
         WHERE rp.role_id = $1 AND p.perm_type = 1",
        role_id
    )
    .fetch_all(pool)
    .await?;
    success(ids)
}
