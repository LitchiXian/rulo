use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

/// 自动校验的 JSON 提取器，替代 axum::Json
/// 先反序列化，再调用 validator::Validate::validate()
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e: JsonRejection| AppError::ServiceError(e.body_text()))?;
        data.validate().map_err(AppError::ValidationError)?;
        Ok(ValidatedJson(data))
    }
}
