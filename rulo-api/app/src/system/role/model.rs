use chrono::{DateTime, Utc};
use common::util::serde_util;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, FromRow, ToSchema)]
pub struct SysRole {
    #[serde(serialize_with = "serde_util::i64_str::serialize")]
    pub id: i64,
    pub role_name: String,
    pub role_key: String,
    pub is_super: bool,
    pub is_active: bool,
    pub is_deleted: bool,
    #[serde(serialize_with = "serde_util::i64_str::serialize")]
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    #[serde(serialize_with = "serde_util::i64_str::serialize")]
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysRole {
    pub fn new_role_from_save_dto(dto: &SysRoleSaveDto) -> Self {
        let role_id: i64 = common::util::id_util::next_id();

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
    #[serde(deserialize_with = "serde_util::i64_str::deserialize")]
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
    #[serde(deserialize_with = "serde_util::i64_str::deserialize")]
    pub role_id: i64,
    #[serde(deserialize_with = "serde_util::vec_i64_str::deserialize")]
    pub menu_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct BindPermsDto {
    #[serde(deserialize_with = "serde_util::i64_str::deserialize")]
    pub role_id: i64,
    #[serde(deserialize_with = "serde_util::vec_i64_str::deserialize")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_role_from_save_dto_sets_defaults() {
        let dto = SysRoleSaveDto {
            role_name: "管理员".to_string(),
            role_key: "admin".to_string(),
            remark: Some("test remark".to_string()),
        };
        let role = SysRole::new_role_from_save_dto(&dto);
        assert_eq!(role.role_name, "管理员");
        assert_eq!(role.role_key, "admin");
        assert!(!role.is_super);
        assert!(role.is_active);
        assert!(!role.is_deleted);
        assert_eq!(role.remark, Some("test remark".to_string()));
        assert!(role.id != 0);
    }

    #[test]
    fn new_role_generates_unique_ids() {
        let dto = SysRoleSaveDto {
            role_name: "r".to_string(),
            role_key: "k".to_string(),
            remark: None,
        };
        let r1 = SysRole::new_role_from_save_dto(&dto);
        let r2 = SysRole::new_role_from_save_dto(&dto);
        assert_ne!(r1.id, r2.id);
    }
}
