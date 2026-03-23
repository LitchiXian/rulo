use axum::body::Body;
use futures_util::StreamExt;
use reqwest::Client;

use crate::ai::chat::model::{ChatMessage, ZhipuRequest, ZhipuResponse};
use rulo_common::error::AppError;

const ZHIPU_API_URL: &str = "";
const ZHIPU_API_KEY: &str = "";
const ZHIPU_MODEL: &str = "";

/// 非流式调用智谱 API
pub async fn chat(messages: Vec<ChatMessage>) -> Result<String, AppError> {
    let client = Client::new();
    let body = ZhipuRequest {
        model: ZHIPU_MODEL.to_string(),
        messages,
        stream: false,
    };

    let resp = client
        .post(ZHIPU_API_URL)
        .header("Authorization", format!("Bearer {}", ZHIPU_API_KEY))
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("AI 请求失败: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp
            .text()
            .await
            .unwrap_or_else(|_| "无法读取响应".to_string());
        return Err(AppError::Internal(format!(
            "AI 接口返回异常: status={status}, body={text}"
        )));
    }

    let data: ZhipuResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("AI 响应解析失败: {e}")))?;

    let content = data
        .choices
        .and_then(|c| c.into_iter().next())
        .and_then(|c| c.message)
        .map(|m| m.content)
        .unwrap_or_default();

    Ok(content)
}

/// 流式调用智谱 API，返回 SSE Body
pub fn chat_stream(messages: Vec<ChatMessage>) -> Body {
    let stream = async_stream::stream! {
        let client = Client::new();
        let body = ZhipuRequest {
            model: ZHIPU_MODEL.to_string(),
            messages,
            stream: true,
        };

        let resp = client
            .post(ZHIPU_API_URL)
            .header("Authorization", format!("Bearer {}", ZHIPU_API_KEY))
            .json(&body)
            .send()
            .await;

        let resp = match resp {
            Ok(r) => r,
            Err(e) => {
                let err_msg = format!("data: {{\"error\":\"{e}\"}}\n\n");
                yield Ok::<_, std::io::Error>(err_msg);
                return;
            }
        };

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            let err_msg = format!("data: {{\"error\":\"AI 接口异常: {status} {text}\"}}\n\n");
            yield Ok::<_, std::io::Error>(err_msg);
            return;
        }

        let mut byte_stream = resp.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = byte_stream.next().await {
            match chunk {
                Ok(bytes) => {
                    buffer.push_str(&String::from_utf8_lossy(&bytes));

                    // 按行处理 SSE 数据
                    while let Some(pos) = buffer.find('\n') {
                        let line = buffer[..pos].to_string();
                        buffer = buffer[pos + 1..].to_string();

                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

                        if let Some(data) = line.strip_prefix("data: ") {
                            if data.trim() == "[DONE]" {
                                yield Ok("data: [DONE]\n\n".to_string());
                                return;
                            }

                            // 解析 delta.content
                            if let Ok(resp) = serde_json::from_str::<ZhipuResponse>(data) {
                                if let Some(content) = resp
                                    .choices
                                    .and_then(|c| c.into_iter().next())
                                    .and_then(|c| c.delta)
                                    .and_then(|d| d.content)
                                {
                                    if !content.is_empty() {
                                        let escaped = serde_json::to_string(&content).unwrap_or_default();
                                        yield Ok(format!("data: {{\"content\":{escaped}}}\n\n"));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    yield Ok(format!("data: {{\"error\":\"{e}\"}}\n\n"));
                    return;
                }
            }
        }

        yield Ok("data: [DONE]\n\n".to_string());
    };

    Body::from_stream(stream)
}
