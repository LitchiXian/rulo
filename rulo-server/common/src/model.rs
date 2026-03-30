use crate::util::serde_util;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct IdDto {
    #[serde(deserialize_with = "serde_util::i64_str::deserialize")]
    pub id: i64,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct IdsDto {
    #[validate(length(min = 1, message = "ID 列表不能为空"))]
    #[serde(deserialize_with = "serde_util::vec_i64_str::deserialize")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_page_defaults() {
        assert_eq!(normalize_page(None, None), (1, 10));
    }

    #[test]
    fn normalize_page_zero_becomes_one() {
        assert_eq!(normalize_page(Some(0), Some(0)), (1, 1));
    }

    #[test]
    fn normalize_page_clamps_upper_bound() {
        assert_eq!(normalize_page(Some(5), Some(200)), (5, 100));
    }

    #[test]
    fn normalize_page_normal_values() {
        assert_eq!(normalize_page(Some(3), Some(20)), (3, 20));
    }

    #[test]
    fn normalize_page_boundary() {
        assert_eq!(normalize_page(Some(1), Some(1)), (1, 1));
        assert_eq!(normalize_page(Some(1), Some(100)), (1, 100));
    }
}
