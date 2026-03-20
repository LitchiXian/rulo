use rulo_common::{
    error::AppError,
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, query, query_as};

use crate::system::menu::model::{SysMenu, SysMenuListDto, SysMenuSaveDto, SysMenuUpdateDto};

pub async fn save(pool: &PgPool, dto: &SysMenuSaveDto) -> R<SysMenu> {
    if dto.name.trim().is_empty() {
        return Err(AppError::ServiceError("菜单名称不能为空".to_string()));
    }
    if dto.menu_type != 1 && dto.menu_type != 2 {
        return Err(AppError::ServiceError("菜单类型仅支持 1(目录) 或 2(菜单)".to_string()));
    }
    // menu_type=2 必须提供路由路径、组件路径和菜单权限码
    if dto.menu_type == 2 {
        let path_ok = dto.path.as_deref().map(|s| !s.trim().is_empty()).unwrap_or(false);
        if !path_ok {
            return Err(AppError::ServiceError("菜单类型为菜单时，路由路径不能为空".to_string()));
        }
        let comp_ok = dto.component.as_deref().map(|s| !s.trim().is_empty()).unwrap_or(false);
        if !comp_ok {
            return Err(AppError::ServiceError("菜单类型为菜单时，组件路径不能为空".to_string()));
        }
        let code_ok = dto.auto_perm_code.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
        if !code_ok {
            return Err(AppError::ServiceError("菜单类型为菜单时，菜单权限码不能为空".to_string()));
        }
    }

    let mut new_menu = SysMenu::new_menu_from_save_dto(dto);
    let mut tx = pool.begin().await?;

    // menu_type=2（菜单）且提供了非空权限编码时，在同一事务内自动创建菜单权限
    if dto.menu_type == 2 {
        if let Some(ref perm_code) = dto.auto_perm_code {
            if !perm_code.is_empty() {
                // 检查 perm_code 是否已存在
                let exists: i64 = sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM sys_permission WHERE perm_code = $1 AND is_deleted = false",
                    perm_code
                )
                .fetch_one(&mut *tx)
                .await?
                .unwrap_or(0);
                if exists > 0 {
                    return Err(AppError::ServiceError(format!("权限码 {} 已存在，请更换", perm_code)));
                }
                let perm_id = new_menu.id - 1;
                let perm_name = format!("{}-页面入口", dto.name);
                let perm_type: i16 = 2;
                let is_deleted = false;
                let ts = new_menu.create_time;
                query!(
                    "INSERT INTO sys_permission(id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                    perm_id, perm_code, &perm_name, perm_type, is_deleted, perm_id, ts, perm_id, ts
                )
                .execute(&mut *tx)
                .await?;
                new_menu.perm_id = Some(perm_id);
            }
        }
    }

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
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    success(new_menu)
}

pub async fn remove(pool: &PgPool, dto: &IdsDto) -> R<()> {
    let mut tx = pool.begin().await?;

    // 先查出这批菜单关联的 perm_id（只有 menu_type=2 auto 创建的才有）
    let perm_ids: Vec<i64> = sqlx::query_scalar!(
        "SELECT perm_id FROM sys_menu WHERE id = ANY($1) AND perm_id IS NOT NULL AND is_deleted = false",
        &dto.ids
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .flatten()
    .collect();

    // 逻辑删除菜单
    sqlx::query!(
        "UPDATE sys_menu SET is_deleted = true, update_time = now() WHERE id = ANY($1)",
        &dto.ids
    )
    .execute(&mut *tx)
    .await?;

    // 级联逻辑删除对应的菜单权限
    if !perm_ids.is_empty() {
        sqlx::query!(
            "UPDATE sys_permission SET is_deleted = true, update_time = now() WHERE id = ANY($1)",
            &perm_ids
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    success(())
}

pub async fn update(pool: &PgPool, dto: &SysMenuUpdateDto) -> R<()> {
    if let Some(ref name) = dto.name {
        if name.trim().is_empty() {
            return Err(AppError::ServiceError("菜单名称不能为空字符串".to_string()));
        }
    }
    if let Some(ref path) = dto.path {
        if path.trim().is_empty() {
            return Err(AppError::ServiceError("路由路径不能为空字符串".to_string()));
        }
    }
    if let Some(ref component) = dto.component {
        if component.trim().is_empty() {
            return Err(AppError::ServiceError("组件路径不能为空字符串".to_string()));
        }
    }
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
        WHERE id = $1 AND is_deleted = false",
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
    let data = query_as!(SysMenu, "select * from sys_menu where id = $1 AND is_deleted = false", dto.id)
        .fetch_one(pool)
        .await?;
    success(data)
}

pub async fn list(pool: &PgPool, _dto: &SysMenuListDto) -> R<Vec<SysMenu>> {
    let data = query_as!(SysMenu, "select * from sys_menu where is_deleted = false ORDER BY sort_order ASC")
        .fetch_all(pool)
        .await?;
    success(data)
}
