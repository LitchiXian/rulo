use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct IdDto {
    pub id: i64,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct IdsDto {
    #[validate(length(min = 1, message = "ID 列表不能为空"))]
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct PageDto {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub page_num: u64,
    pub page_size: u64,
}

pub fn normalize_page(page_num: Option<u64>, page_size: Option<u64>) -> (u64, u64) {
    let page_num = page_num.unwrap_or(1).max(1);
    let page_size = page_size.unwrap_or(10).clamp(1, 100);
    (page_num, page_size)
}

// 权限码集合, 存入 request Extension
#[derive(Debug, Clone)]
pub struct PermCodes(pub Vec<String>);
