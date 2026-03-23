use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

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
        // todo 生成 雪花ID
        let role_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

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

#[derive(Deserialize, Debug, ToSchema)]
pub struct SysRoleSaveDto {
    pub role_name: String,
    pub role_key: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct SysRoleUpdateDto {
    pub id: i64,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub is_active: Option<bool>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct BindMenusDto {
    pub role_id: i64,
    pub menu_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, ToSchema)]
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
