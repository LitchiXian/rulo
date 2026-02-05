//! 统一错误处理模块
//! 
//! 提供应用级别的错误类型和错误处理机制

use axum::response::{IntoResponse, Response};
use std::fmt;
use crate::utils::response::{ApiResponse, codes};

/// 应用统一错误类型
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    /// 资源不存在
    NotFound(String),
    /// 未授权/登录过期
    Unauthorized(String),
    /// 禁止访问
    Forbidden(String),
    /// 参数错误
    BadRequest(String),
    /// 数据库错误
    Database(String),
    /// 内部服务器错误
    Internal(String),
    /// 业务逻辑错误（带自定义状态码）
    Business { code: String, message: String },
}

#[allow(dead_code)]
impl AppError {
    /// 创建资源不存在错误
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// 创建未授权错误
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    /// 创建禁止访问错误
    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden(msg.into())
    }

    /// 创建参数错误
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    /// 创建数据库错误
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// 创建内部错误
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// 创建业务错误
    pub fn business(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::Business {
            code: code.into(),
            message: msg.into(),
        }
    }

    /// 获取错误码
    pub fn code(&self) -> &str {
        match self {
            AppError::NotFound(_) => codes::NOT_FOUND,
            AppError::Unauthorized(_) => codes::UNAUTHORIZED,
            AppError::Forbidden(_) => codes::FORBIDDEN,
            AppError::BadRequest(_) => codes::BAD_REQUEST,
            AppError::Database(_) => codes::ERROR,
            AppError::Internal(_) => codes::ERROR,
            AppError::Business { code, .. } => code,
        }
    }

    /// 获取错误消息
    pub fn message(&self) -> &str {
        match self {
            AppError::NotFound(msg) => msg,
            AppError::Unauthorized(msg) => msg,
            AppError::Forbidden(msg) => msg,
            AppError::BadRequest(msg) => msg,
            AppError::Database(msg) => msg,
            AppError::Internal(msg) => msg,
            AppError::Business { message, .. } => message,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        ApiResponse::error_with_code(self.code(), self.message()).into_response()
    }
}

// ============ 从其他错误类型转换 ============

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("记录不存在".to_string()),
            _ => AppError::Database(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON解析错误: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("IO错误: {}", err))
    }
}

// ============ 从现有业务错误类型转换 ============

use crate::models::user::UserError;
use crate::models::article::ArticleError;
use crate::models::tag::TagError;

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::UserNotFound => AppError::not_found("用户不存在"),
            UserError::UsernameExists => AppError::bad_request("用户名已存在"),
            UserError::EmailExists => AppError::bad_request("邮箱已存在"),
            UserError::PasswordIncorrect => AppError::unauthorized("密码错误"),
            UserError::VerificationCodeError => AppError::bad_request("验证码错误"),
            UserError::DatabaseError(msg) => AppError::database(msg),
            UserError::Other(msg) => AppError::internal(msg),
        }
    }
}

impl From<ArticleError> for AppError {
    fn from(err: ArticleError) -> Self {
        match err {
            ArticleError::ArticleNotFound => AppError::not_found("文章不存在"),
            ArticleError::DatabaseError(msg) => AppError::database(msg),
            ArticleError::Other(msg) => AppError::internal(msg),
        }
    }
}

impl From<TagError> for AppError {
    fn from(err: TagError) -> Self {
        match err {
            TagError::TagNotFound => AppError::not_found("标签不存在"),
            TagError::TagNameExists => AppError::bad_request("标签名称已存在"),
            TagError::DatabaseError(msg) => AppError::database(msg),
            TagError::Other(msg) => AppError::internal(msg),
        }
    }
}

/// Result 类型别名，使用 AppError 作为错误类型
#[allow(dead_code)]
pub type AppResult<T> = Result<T, AppError>;

/// 将 Option 转换为 AppResult
#[allow(dead_code)]
pub trait OptionExt<T> {
    /// 将 None 转换为 NotFound 错误
    fn ok_or_not_found(self, msg: impl Into<String>) -> AppResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_not_found(self, msg: impl Into<String>) -> AppResult<T> {
        self.ok_or_else(|| AppError::not_found(msg))
    }
}
