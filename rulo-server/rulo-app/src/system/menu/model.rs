use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct SysMenu {
    pub id: i64,
    pub parent_id: i64,
    pub perm_id: Option<i64>,
    pub name: String,
    pub menu_type: i16,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub is_hidden: bool,
    pub is_deleted: bool,
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysMenu {
    pub fn new_menu_from_save_dto(dto: &SysMenuSaveDto) -> Self {
        // todo 生成 雪花ID
        let menu_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let now_time = Utc::now();

        SysMenu {
            id: menu_id,
            parent_id: dto.parent_id.unwrap_or(0),
            perm_id: dto.perm_id,
            name: dto.name.clone(),
            menu_type: dto.menu_type,
            path: dto.path.clone(),
            component: dto.component.clone(),
            icon: dto.icon.clone(),
            sort_order: dto.sort_order.unwrap_or(0),
            is_hidden: false,
            is_deleted: false,
            create_id: menu_id,
            create_time: now_time.clone(),
            update_id: menu_id,
            update_time: now_time.clone(),
            remark: dto.remark.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SysMenuSaveDto {
    pub parent_id: Option<i64>,
    pub perm_id: Option<i64>,
    pub name: String,
    pub menu_type: i16,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SysMenuUpdateDto {
    pub id: i64,
    pub name: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_hidden: Option<bool>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SysMenuListDto {
    pub name: Option<String>,
    pub menu_type: Option<i16>,
    pub is_hidden: Option<bool>,
    pub parent_id: Option<i64>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
}
