use axum::{Json, response::IntoResponse};
use serde::Serialize;

use crate::error::AppError;

pub type R<T> = Result<ApiResult<T>, AppError>;

pub fn success<T: Serialize>(data: T) -> R<T> {
    Ok(ApiResult::ok(data))
}

pub fn error(code: i32, message: &str) -> R<()> {
    Ok(ApiResult::err(code, message))
}

#[derive(Serialize)]
pub struct ApiResult<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

impl<T: Serialize> IntoResponse for ApiResult<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> ApiResult<T> {
    pub fn ok(data: T) -> Self {
        ApiResult {
            code: 200,
            data: Some(data),
            message: "success".to_string(),
        }
    }

    pub fn err(code: i32, message: impl Into<String>) -> Self {
        ApiResult {
            code,
            data: None,
            message: message.into(),
        }
    }
}
