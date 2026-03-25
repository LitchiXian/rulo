use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 前端发送的聊天请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatRequest {
    /// 对话历史 messages，格式与 OpenAI 兼容
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ChatMessage {
    /// 消息角色（user / assistant / system）
    pub role: String,
    /// 消息内容
    pub content: String,
}

/// 非流式返回
#[derive(Debug, Serialize, ToSchema)]
pub struct ChatResponse {
    /// AI 回复内容
    pub content: String,
}

// ---- 智谱 API 请求/响应结构 ----

#[derive(Debug, Serialize)]
pub struct ZhipuRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct ZhipuResponse {
    pub choices: Option<Vec<ZhipuChoice>>,
}

#[derive(Debug, Deserialize)]
pub struct ZhipuChoice {
    pub message: Option<ZhipuMessage>,
    pub delta: Option<ZhipuDelta>,
}

#[derive(Debug, Deserialize)]
pub struct ZhipuMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ZhipuDelta {
    pub content: Option<String>,
}
