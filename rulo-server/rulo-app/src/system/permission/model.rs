use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysPermission {
    pub id: i64,
    pub perm_code: String,
    pub perm_name: String,
    pub perm_type: i16,
    pub is_deleted: bool,
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysPermission {
    pub fn new_permission_from_save_dto(dto: &SysPermissionSaveDto) -> Self {
        let perm_id: i64 = rulo_common::util::id_util::next_id();

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
