use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 文章模型结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub user_id: u64,
    pub published_time: DateTime<Utc>,
    pub is_published: u8,
    pub is_deleted: u8,
    pub create_id: u64,
    pub create_time: DateTime<Utc>,
    pub update_id: u64,
    pub update_time: DateTime<Utc>,
}

// 自定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum ArticleError {
    #[error("文章不存在")]
    ArticleNotFound,
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    #[error("其他错误: {0}")]
    #[allow(dead_code)]
    Other(String),
}

// 移除了手动实现的 Display trait，因为 thiserror::Error 已经自动生成了
