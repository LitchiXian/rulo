use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct R<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

impl<T: Serialize> IntoResponse for R<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> R<T> {
    pub fn ok(data: T) -> R<T> {
        R {
            code: 200,
            data: Some(data),
            message: "success".to_string(),
        }
    }

    pub fn err(message: &str) -> R<T> {
        R {
            code: 500,
            data: None,
            message: message.to_string(),
        }
    }
}
