use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use common::{error::AppError, result::R, state::AppState};
use macros::perm;

use crate::ai::chat::{
    model::{ChatRequest, ChatResponse},
    service,
};

#[utoipa::path(
    post, path = "/ai/chat",
    tag = "AI",
    request_body = ChatRequest,
    responses((status = 200, description = "AI 对话（非流式）", body = ChatResponse)),
    security(("bearer_auth" = []))
)]
/// 非流式 AI 对话
#[perm("ai:ask:chat")]
pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> R<ChatResponse> {
    let content = service::chat(req.messages, &state.ai_config).await?;
    common::result::success(ChatResponse { content })
}

#[utoipa::path(
    post, path = "/ai/chat/stream",
    tag = "AI",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "AI 流式对话（SSE），Content-Type: text/event-stream", body = String)
    ),
    security(("bearer_auth" = []))
)]
/// 流式 AI 对话 (SSE)
#[perm("ai:ask:chat")]
pub async fn chat_stream_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Result<Response, AppError> {
    let body = service::chat_stream(req.messages, &state.ai_config);

    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .map(|r| r.into_response())
        .map_err(|e| AppError::Internal(e.to_string()))
}
