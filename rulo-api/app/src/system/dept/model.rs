use chrono::{DateTime, Utc};
use common::util::serde_util;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysDept {
    #[serde(with = "serde_util::i64_str")]
    pub id: i64,
    #[serde(with = "serde_util::i64_str")]
    pub parent_id: i64,
    pub name: String,
    pub sort_order: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub is_deleted: bool,
    #[serde(with = "serde_util::i64_str")]
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    #[serde(with = "serde_util::i64_str")]
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysDept {
    pub fn new_dept_from_save_dto(dto: &SysDeptSaveDto) -> Self {
        let dept_id: i64 = common::util::id_util::next_id();
        let now_time = Utc::now();

        SysDept {
            id: dept_id,
            parent_id: dto.parent_id.unwrap_or(0),
            name: dto.name.clone(),
            sort_order: dto.sort_order.unwrap_or(0),
            leader: dto.leader.clone(),
            phone: dto.phone.clone(),
            email: dto.email.clone(),
            is_active: true,
            is_deleted: false,
            create_id: dept_id,
            create_time: now_time,
            update_id: dept_id,
            update_time: now_time,
            remark: dto.remark.clone(),
        }
    }
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysDeptSaveDto {
    #[serde(default, deserialize_with = "serde_util::opt_i64_str::deserialize")]
    pub parent_id: Option<i64>,
    #[validate(length(min = 1, max = 50, message = "部门名称长度必须在 1-50 之间"))]
    pub name: String,
    pub sort_order: Option<i32>,
    #[validate(length(max = 50, message = "负责人长度不能超过 50"))]
    pub leader: Option<String>,
    #[validate(length(max = 30, message = "联系电话长度不能超过 30"))]
    pub phone: Option<String>,
    #[validate(email(message = "邮箱格式不正确"), length(max = 50, message = "邮箱长度不能超过 50"))]
    pub email: Option<String>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysDeptUpdateDto {
    #[serde(deserialize_with = "serde_util::i64_str::deserialize")]
    pub id: i64,
    #[serde(default, deserialize_with = "serde_util::opt_i64_str::deserialize")]
    pub parent_id: Option<i64>,
    #[validate(length(min = 1, max = 50, message = "部门名称长度必须在 1-50 之间"))]
    pub name: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
    #[validate(length(max = 50, message = "负责人长度不能超过 50"))]
    pub leader: Option<String>,
    #[validate(length(max = 30, message = "联系电话长度不能超过 30"))]
    pub phone: Option<String>,
    #[validate(email(message = "邮箱格式不正确"), length(max = 50, message = "邮箱长度不能超过 50"))]
    pub email: Option<String>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, IntoParams, ToSchema)]
pub struct SysDeptListDto {
    pub name: Option<String>,
    pub is_active: Option<bool>,
    #[serde(default, deserialize_with = "serde_util::opt_i64_str::deserialize")]
    pub parent_id: Option<i64>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
}
