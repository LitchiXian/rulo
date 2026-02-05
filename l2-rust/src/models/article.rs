use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 文章数据库模型结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub published_time: Option<DateTime<Utc>>,
    pub is_published: bool,
    pub is_deleted: bool,
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
}

// 前端响应 DTO（驼峰命名，时间戳格式）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub user_id: String,
    pub user_name: String,
    pub create_time: i64,  // 毫秒时间戳
    pub tags: String,      // 逗号分隔的标签ID
}

impl From<Article> for ArticleResponse {
    fn from(article: Article) -> Self {
        ArticleResponse {
            id: article.id.to_string(),
            title: article.title,
            content: article.content,
            user_id: article.user_id.to_string(),
            user_name: article.user_name.unwrap_or_else(|| "未知用户".to_string()),
            create_time: article.create_time.timestamp_millis(),
            tags: String::new(),  // 需要单独查询
        }
    }
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
