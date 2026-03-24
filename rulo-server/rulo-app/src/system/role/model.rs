use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, FromRow, ToSchema)]
pub struct SysRole {
    pub id: i64,
    pub role_name: String,
    pub role_key: String,
    pub is_super: bool,
    pub is_active: bool,
    pub is_deleted: bool,
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysRole {
    pub fn new_role_from_save_dto(dto: &SysRoleSaveDto) -> Self {
        let role_id: i64 = rulo_common::util::id_util::next_id();

        let now_time = Utc::now();

        SysRole {
            id: role_id,
            role_name: dto.role_name.clone(),
            role_key: dto.role_key.clone(),
            is_super: false,
            is_active: true,
            is_deleted: false,
            create_id: role_id,
            create_time: now_time.clone(),
            update_id: role_id,
            update_time: now_time.clone(),
            remark: dto.remark.clone(),
        }
    }
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysRoleSaveDto {
    #[validate(length(min = 1, max = 30, message = "角色名称长度必须在 1-30 之间"))]
    pub role_name: String,
    #[validate(length(min = 1, max = 50, message = "角色标识长度必须在 1-50 之间"))]
    pub role_key: String,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysRoleUpdateDto {
    pub id: i64,
    #[validate(length(min = 1, max = 30, message = "角色名称长度必须在 1-30 之间"))]
    pub role_name: Option<String>,
    #[validate(length(min = 1, max = 50, message = "角色标识长度必须在 1-50 之间"))]
    pub role_key: Option<String>,
    pub is_active: Option<bool>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct BindMenusDto {
    pub role_id: i64,
    pub menu_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct BindPermsDto {
    pub role_id: i64,
    pub perm_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, IntoParams, ToSchema)]
pub struct SysRoleListDto {
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub is_active: Option<bool>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
