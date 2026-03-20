use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

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
        // todo 生成 雪花ID
        let perm_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

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

#[derive(Deserialize, Debug, ToSchema)]
pub struct SysPermissionSaveDto {
    pub perm_code: String,
    pub perm_name: String,
    pub perm_type: i16,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct SysPermissionUpdateDto {
    pub id: i64,
    pub perm_name: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, IntoParams, ToSchema)]
pub struct SysPermissionListDto {
    pub perm_code: Option<String>,
    pub perm_name: Option<String>,
    pub perm_type: Option<i16>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
}
