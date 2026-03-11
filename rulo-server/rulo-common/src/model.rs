use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IdDto {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct IdsDto {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PageDto {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
