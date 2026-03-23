use serde::{Deserialize, Serialize};

/// 前端发送的聊天请求
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    /// 对话历史 messages，格式与 OpenAI 兼容
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// 非流式返回
#[derive(Debug, Serialize)]
pub struct ChatResponse {
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
