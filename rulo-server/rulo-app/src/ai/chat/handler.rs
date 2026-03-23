use axum::{
    Json,
    response::{IntoResponse, Response},
};
use rulo_common::result::R;

use crate::ai::chat::{
    model::{ChatRequest, ChatResponse},
    service,
};

/// 非流式 AI 对话
pub async fn chat_handler(Json(req): Json<ChatRequest>) -> R<ChatResponse> {
    let content = service::chat(req.messages).await?;
    rulo_common::result::success(ChatResponse { content })
}

/// 流式 AI 对话 (SSE)
pub async fn chat_stream_handler(Json(req): Json<ChatRequest>) -> Response {
    let body = service::chat_stream(req.messages);

    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .unwrap()
        .into_response()
}
