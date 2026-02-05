use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 标签模型结构体
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub remark: Option<String>,
    pub status: bool,
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    pub update_id: Option<i64>,
    pub update_time: Option<DateTime<Utc>>,
}

// 自定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum TagError {
    #[error("标签不存在")]
    TagNotFound,
    #[error("标签名称已存在")]
    TagNameExists,
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    #[error("其他错误: {0}")]
    #[allow(dead_code)]
    Other(String),
}
