use chrono::{DateTime, Utc};
use common::util::serde_util;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysPermission {
    #[serde(with = "serde_util::i64_str")]
    pub id: i64,
    pub perm_code: String,
    pub perm_name: String,
    pub perm_type: i16,
    pub is_deleted: bool,
    #[serde(with = "serde_util::i64_str")]
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    #[serde(with = "serde_util::i64_str")]
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysPermission {
    pub fn new_permission_from_save_dto(dto: &SysPermissionSaveDto) -> Self {
        let perm_id: i64 = common::util::id_util::next_id();

        let now_time = Utc::now();

        SysPermission {
            id: perm_id,
            perm_code: dto.perm_code.clone(),
            perm_name: dto.perm_name.clone(),
            perm_type: dto.perm_type,
            is_deleted: false,
            create_id: perm_id,
            create_time: now_time.clone(),
            update_id: perm_id,
            update_time: now_time.clone(),
            remark: dto.remark.clone(),
        }
    }
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysPermissionSaveDto {
    #[validate(length(min = 1, max = 100, message = "权限码长度必须在 1-100 之间"))]
    pub perm_code: String,
    #[validate(length(min = 1, max = 50, message = "权限名称长度必须在 1-50 之间"))]
    pub perm_name: String,
    #[validate(range(min = 1, max = 2, message = "权限类型仅支持 1(API权限) 或 2(菜单权限)"))]
    pub perm_type: i16,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysPermissionUpdateDto {
    #[serde(deserialize_with = "serde_util::i64_str::deserialize")]
    pub id: i64,
    #[validate(length(min = 1, max = 50, message = "权限名称长度必须在 1-50 之间"))]
    pub perm_name: Option<String>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, IntoParams, ToSchema)]
pub struct SysPermissionListDto {
    pub perm_code: Option<String>,
    pub perm_name: Option<String>,
    pub perm_type: Option<i16>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_permission_from_save_dto_sets_defaults() {
        let dto = SysPermissionSaveDto {
            perm_code: "user:list".to_string(),
            perm_name: "用户列表".to_string(),
            perm_type: 1,
            remark: None,
        };
        let perm = SysPermission::new_permission_from_save_dto(&dto);
        assert_eq!(perm.perm_code, "user:list");
        assert_eq!(perm.perm_name, "用户列表");
        assert_eq!(perm.perm_type, 1);
        assert!(!perm.is_deleted);
        assert!(perm.id != 0);
    }

    #[test]
    fn new_permission_generates_unique_ids() {
        let dto = SysPermissionSaveDto {
            perm_code: "c".to_string(),
            perm_name: "n".to_string(),
            perm_type: 2,
            remark: None,
        };
        let p1 = SysPermission::new_permission_from_save_dto(&dto);
        let p2 = SysPermission::new_permission_from_save_dto(&dto);
        assert_ne!(p1.id, p2.id);
    }
}
