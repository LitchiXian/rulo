use crate::handlers::auth_handler::AjaxResult;
use crate::services::tag_service::TagService;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

// 请求体结构体
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveTagRequest {
    pub name: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagRequest {
    pub id: i64,
    pub name: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: i64,
}

// 获取标签列表
pub async fn list_tags_handler(
    State(tag_service): State<Arc<TagService>>,
) -> impl IntoResponse {
    match tag_service.list().await {
        Ok(tags) => AjaxResult::success(Some(serde_json::to_value(tags).unwrap())),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 根据ID获取标签
pub async fn get_tag_handler(
    State(tag_service): State<Arc<TagService>>,
    Query(params): Query<IdQuery>,
) -> impl IntoResponse {
    match tag_service.get_by_id(params.id).await {
        Ok(tag) => AjaxResult::success(Some(serde_json::to_value(tag).unwrap())),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 保存标签
pub async fn save_tag_handler(
    State(tag_service): State<Arc<TagService>>,
    Json(payload): Json<SaveTagRequest>,
) -> impl IntoResponse {
    // 简化处理，使用固定的 create_id，实际应从 token 获取
    let create_id = 1i64;
    
    match tag_service.save(payload.name, payload.remark, create_id).await {
        Ok(_) => AjaxResult::success(None),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 更新标签
pub async fn update_tag_handler(
    State(tag_service): State<Arc<TagService>>,
    Json(payload): Json<UpdateTagRequest>,
) -> impl IntoResponse {
    // 简化处理，使用固定的 update_id，实际应从 token 获取
    let update_id = 1i64;
    
    match tag_service.update(payload.id, payload.name, payload.remark, update_id).await {
        Ok(_) => AjaxResult::success(None),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 删除标签
pub async fn remove_tag_handler(
    State(tag_service): State<Arc<TagService>>,
    Json(params): Json<IdQuery>,
) -> impl IntoResponse {
    match tag_service.remove(params.id).await {
        Ok(_) => AjaxResult::success(None),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}
