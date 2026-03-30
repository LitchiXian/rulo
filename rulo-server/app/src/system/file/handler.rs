use std::sync::Arc;

use axum::extract::State;
use axum_extra::extract::Multipart;
use common::{error::AppError, result::R, state::AppState};

use crate::system::file::service;

#[utoipa::path(
    post, path = "/system/file/upload",
    responses((status = 200, description = "上传成功，返回对象 key", body = String)),
    security(("bearer_auth" = []))
)]
pub async fn upload_handler(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> R<String> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::ServiceError(format!("解析上传文件失败: {}", e)))?
    {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name != "file" {
            continue;
        }

        let file_name = field
            .file_name()
            .unwrap_or("unknown")
            .to_string();

        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::ServiceError(format!("读取文件数据失败: {}", e)))?;

        return service::upload(
            &state.s3_bucket,
            &state.storage_config,
            &file_name,
            &content_type,
            &data,
        )
        .await;
    }

    Err(AppError::ServiceError("未找到上传文件，请使用 field name 'file'".to_string()))
}
