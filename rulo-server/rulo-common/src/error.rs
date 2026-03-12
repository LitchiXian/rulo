use std::sync::Arc;

use argon2::password_hash;
use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::result::ApiResult;

// The kinds of errors we can hit in ou application.
#[derive(Debug)]
pub enum AppError {
    ServiceError(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    DbError(sqlx::Error),
    RedisError(redis::RedisError),
    RedisPoolError(deadpool_redis::PoolError),
    Internal(String),
    PasswordHashError(password_hash::Error),
}

impl AppError {
    pub fn http_status(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::ServiceError(_) => StatusCode::OK,
            Self::DbError(_) => StatusCode::OK,
            Self::RedisError(_) => StatusCode::OK,
            Self::RedisPoolError(_) => StatusCode::OK,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn biz_code(&self) -> i32 {
        match self {
            Self::ServiceError(_) => 40000,
            Self::Unauthorized(_) => 40100,
            Self::Forbidden(_) => 40300,
            Self::NotFound(_) => 40400,
            Self::DbError(_) => 50001,
            Self::RedisError(_) => 50002,
            Self::RedisPoolError(_) => 50003,
            Self::Internal(_) => 50000,
            Self::PasswordHashError(_) => 50001,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            Self::ServiceError(msg) => msg.clone(),
            Self::Unauthorized(msg) => msg.clone(),
            Self::Forbidden(msg) => msg.clone(),
            Self::NotFound(msg) => msg.clone(),
            Self::DbError(_) => "数据库异常".to_string(),
            Self::RedisError(_) => "缓存异常".to_string(),
            Self::RedisPoolError(_) => "缓存连接池异常".to_string(),
            Self::Internal(msg) => msg.clone(),
            Self::PasswordHashError(_) => "密码加密失败,请稍后充实".to_string(),
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

// Imagine this is some third party library that we're using, It sometimes returns errors which we
// want to log
pub mod time_library {
    use serde::Serialize;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[derive(Serialize, Debug)]
    pub struct Timestamp(pub u64);

    impl Timestamp {
        pub fn now() -> Result<Self, Error> {
            static COUNTER: AtomicU64 = AtomicU64::new(0);

            if COUNTER.fetch_add(1, Ordering::SeqCst).is_multiple_of(3) {
                Err(Error::FailedToGetTime)
            } else {
                Ok(Self(1337))
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Error {
        FailedToGetTime,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "failed to get time")
        }
    }
}
