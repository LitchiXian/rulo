use std::sync::Arc;

use argon2::password_hash;
use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

use crate::result::ApiResult;

// The kinds of errors we can hit in ou application.
#[derive(Debug)]
pub enum AppError {
    ServiceError(String),
    Unauthorized(String),
    Forbidden(String),
    /// 操作被拒绝，原因是该操作仅限超级管理员（业务码 40301，区别于普通权限不足 40300）
    SuperAdminOnly(String),
    NotFound(String),
    TooManyRequests(String),
    DbError(sqlx::Error),
    RedisError(redis::RedisError),
    RedisPoolError(deadpool_redis::PoolError),
    Internal(String),
    PasswordHashError(password_hash::Error),
    ValidationError(ValidationErrors),
}

impl AppError {
    pub fn http_status(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::SuperAdminOnly(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::ServiceError(_) => StatusCode::OK,
            Self::DbError(_) => StatusCode::OK,
            Self::RedisError(_) => StatusCode::OK,
            Self::RedisPoolError(_) => StatusCode::OK,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn biz_code(&self) -> i32 {
        match self {
            Self::ServiceError(_) => 40000,
            Self::Unauthorized(_) => 40100,
            Self::Forbidden(_) => 40300,
            Self::SuperAdminOnly(_) => 40301,
            Self::NotFound(_) => 40400,
            Self::TooManyRequests(_) => 42900,
            Self::DbError(_) => 50001,
            Self::RedisError(_) => 50002,
            Self::RedisPoolError(_) => 50003,
            Self::Internal(_) => 50000,
            Self::PasswordHashError(_) => 50001,
            Self::ValidationError(_) => 40000,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            Self::ServiceError(msg) => msg.clone(),
            Self::Unauthorized(msg) => msg.clone(),
            Self::Forbidden(msg) => msg.clone(),
            Self::SuperAdminOnly(msg) => msg.clone(),
            Self::NotFound(msg) => msg.clone(),
            Self::TooManyRequests(msg) => msg.clone(),
            Self::DbError(_) => "数据库异常".to_string(),
            Self::RedisError(_) => "缓存异常".to_string(),
            Self::RedisPoolError(_) => "缓存连接池异常".to_string(),
            Self::Internal(msg) => msg.clone(),
            Self::PasswordHashError(_) => "密码加密失败,请稍后重试".to_string(),
            Self::ValidationError(errs) => {
                // 提取第一个校验错误信息
                errs.field_errors()
                    .values()
                    .flat_map(|v| v.iter())
                    .find_map(|e| e.message.as_ref().map(|m| m.to_string()))
                    .unwrap_or_else(|| "参数校验失败".to_string())
            }
        }
    }
}

// Tell axum how `AppError` should be converted into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.http_status();
        let body = ApiResult::<()>::err(self.biz_code(), self.user_message());
        let mut response = (status, Json(body)).into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

pub async fn log_app_errors(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    if let Some(err) = response.extensions().get::<Arc<AppError>>() {
        tracing::error!(
            method = %method,
            uri = %uri,
            status = %response.status(),
            error = ?err,
            "request failed"
        );
    }

    response
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::DbError(err)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        Self::RedisError(err)
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        Self::RedisPoolError(err)
    }
}

impl From<password_hash::Error> for AppError {
    fn from(err: password_hash::Error) -> Self {
        Self::PasswordHashError(err)
    }
}
