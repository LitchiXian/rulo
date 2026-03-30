use chrono::{DateTime, Utc};
use common::util::serde_util;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysMenu {
    #[serde(with = "serde_util::i64_str")]
    pub id: i64,
    #[serde(with = "serde_util::i64_str")]
    pub parent_id: i64,
    #[serde(with = "serde_util::opt_i64_str")]
    pub perm_id: Option<i64>,
    pub name: String,
    pub menu_type: i16,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub is_hidden: bool,
    pub is_deleted: bool,
    #[serde(with = "serde_util::i64_str")]
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    #[serde(with = "serde_util::i64_str")]
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysMenu {
    pub fn new_menu_from_save_dto(dto: &SysMenuSaveDto) -> Self {
        let menu_id: i64 = common::util::id_util::next_id();

        let now_time = Utc::now();

        SysMenu {
            id: menu_id,
            parent_id: dto.parent_id.unwrap_or(0),
            perm_id: None,
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

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysMenuSaveDto {
    #[serde(default, deserialize_with = "serde_util::opt_i64_str::deserialize")]
    pub parent_id: Option<i64>,
    #[validate(length(min = 1, max = 50, message = "菜单名称长度必须在 1-50 之间"))]
    pub name: String,
    #[validate(range(min = 1, max = 2, message = "菜单类型仅支持 1(目录) 或 2(菜单)"))]
    pub menu_type: i16,
    #[validate(length(max = 200, message = "路由路径长度不能超过 200"))]
    pub path: Option<String>,
    #[validate(length(max = 200, message = "组件路径长度不能超过 200"))]
    pub component: Option<String>,
    #[validate(length(max = 100, message = "图标长度不能超过 100"))]
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
    /// 仅 menu_type=2 时有效，填写则自动新建 perm_type=2 的菜单权限并关联
    #[validate(length(max = 100, message = "权限码长度不能超过 100"))]
    pub auto_perm_code: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysMenuUpdateDto {
    #[serde(deserialize_with = "serde_util::i64_str::deserialize")]
    pub id: i64,
    #[validate(length(min = 1, max = 50, message = "菜单名称长度必须在 1-50 之间"))]
    pub name: Option<String>,
    #[validate(length(max = 200, message = "路由路径长度不能超过 200"))]
    pub path: Option<String>,
    #[validate(length(max = 200, message = "组件路径长度不能超过 200"))]
    pub component: Option<String>,
    #[validate(length(max = 100, message = "图标长度不能超过 100"))]
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_hidden: Option<bool>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, IntoParams, ToSchema)]
pub struct SysMenuListDto {
    pub name: Option<String>,
    pub menu_type: Option<i16>,
    pub is_hidden: Option<bool>,
    pub parent_id: Option<i64>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
