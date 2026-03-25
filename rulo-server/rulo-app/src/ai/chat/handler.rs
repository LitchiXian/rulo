use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use rulo_common::{result::R, state::AppState};

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
pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> R<ChatResponse> {
    let content = service::chat(req.messages, &state.ai_config).await?;
    rulo_common::result::success(ChatResponse { content })
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
pub async fn chat_stream_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Response {
    let body = service::chat_stream(req.messages, &state.ai_config);

    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .unwrap()
        .into_response()
}
