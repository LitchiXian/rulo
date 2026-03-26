use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysAuditLog {
    pub id: i64,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub method: String,
    pub path: String,
    pub params: Option<String>,
    pub status: i16,
    pub duration_ms: i64,
    pub ip: Option<String>,
    pub is_sensitive: bool,
    pub created_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AuditLogListDto {
    pub user_name: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub is_sensitive: Option<bool>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
