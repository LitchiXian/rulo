use rulo_common::{
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, query, query_as};

use crate::system::menu::model::{SysMenu, SysMenuListDto, SysMenuSaveDto, SysMenuUpdateDto};

pub async fn save(pool: &PgPool, dto: &SysMenuSaveDto) -> R<SysMenu> {
    let new_menu = SysMenu::new_menu_from_save_dto(&dto);
    query!(
        "insert into sys_menu(
        id, parent_id, perm_id, name, menu_type,
        path, component, icon, sort_order, is_hidden,
        is_deleted, create_id, create_time, update_id, update_time, remark
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
        new_menu.id,
        new_menu.parent_id,
        new_menu.perm_id,
        &new_menu.name,
        new_menu.menu_type,
        new_menu.path.as_deref(),
        new_menu.component.as_deref(),
        new_menu.icon.as_deref(),
        new_menu.sort_order,
        new_menu.is_hidden,
        new_menu.is_deleted,
        new_menu.create_id,
        new_menu.create_time,
        new_menu.update_id,
        new_menu.update_time,
        new_menu.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(new_menu)
}

pub async fn remove(pool: &PgPool, dto: &IdsDto) -> R<()> {
    sqlx::query!("DELETE FROM sys_menu WHERE id = ANY($1)", &dto.ids)
        .execute(pool)
        .await?;
    success(())
}

pub async fn update(pool: &PgPool, dto: &SysMenuUpdateDto) -> R<()> {
    sqlx::query!(
        "UPDATE sys_menu SET
            name = COALESCE($2, name),
            path = COALESCE($3, path),
            component = COALESCE($4, component),
            icon = COALESCE($5, icon),
            sort_order = COALESCE($6, sort_order),
            is_hidden = COALESCE($7, is_hidden),
            remark = COALESCE($8, remark),
            update_time = now()
        WHERE id = $1",
        dto.id,
        dto.name.as_deref(),
        dto.path.as_deref(),
        dto.component.as_deref(),
        dto.icon.as_deref(),
        dto.sort_order,
        dto.is_hidden,
        dto.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn detail(pool: &PgPool, dto: &IdDto) -> R<SysMenu> {
    let data = query_as!(SysMenu, "select * from sys_menu where id = $1", dto.id)
        .fetch_one(pool)
        .await?;
    success(data)
}

pub async fn list(pool: &PgPool, _dto: &SysMenuListDto) -> R<Vec<SysMenu>> {
    let data = query_as!(SysMenu, "select * from sys_menu where is_deleted = false")
        .fetch_all(pool)
        .await?;
    success(data)
}
