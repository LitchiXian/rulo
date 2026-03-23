use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct IdDto {
    pub id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct IdsDto {
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
