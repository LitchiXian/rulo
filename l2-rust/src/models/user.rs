use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 用户模型结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub email: Option<String>,
    pub is_active: u8,
    pub is_deleted: u8,
    pub remark: Option<String>,
    pub create_id: u64,
    pub create_time: DateTime<Utc>,
    pub update_id: u64,
    pub update_time: Option<DateTime<Utc>>,
}

// 用户信息结构体（用于返回给前端）- 与前端UserInfo类型匹配
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub user_name: String,
    pub nick_name: String,
    pub avatar: String,
    pub email: Option<String>,
    pub remark: Option<String>,
}

// 自定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("用户不存在")]
    UserNotFound,
    #[error("用户名已存在")]
    UsernameExists,
    #[error("邮箱已存在")]
    EmailExists,
    #[error("密码错误")]
    PasswordIncorrect,
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    #[error("验证码错误")]
    VerificationCodeError,
    #[error("其他错误: {0}")]
    Other(String),
}
