use serde::Deserialize;
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

// 权限码集合, 存入 request Extension
#[derive(Debug, Clone)]
pub struct PermCodes(pub Vec<String>);
