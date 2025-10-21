use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 用户模型结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub email: String,
    pub is_active: u8,
    pub is_deleted: u8,
    pub remark: String,
    pub create_id: u64,
    pub create_time: DateTime<Utc>,
    pub update_id: u64,
    pub update_time: DateTime<Utc>,
}

// 用户信息结构体（用于返回给前端）
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub user_name: String,
    pub email: String,
    pub is_active: u8,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

// 从User转换为UserInfo
impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: user.id,
            user_name: user.user_name,
            email: user.email,
            is_active: user.is_active,
            create_time: user.create_time,
            update_time: user.update_time,
        }
    }
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

// 移除了手动实现的 Display trait，因为 thiserror::Error 已经自动生成了
