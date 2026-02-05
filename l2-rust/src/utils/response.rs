//! 统一响应体模块
//! 
//! 提供标准化的 API 响应格式，兼容前端 AjaxResult 结构

use axum::http::{header, HeaderValue};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::Value;

/// 统一响应体结构
/// 
/// 格式: `{ "code": "200", "msg": "xxx", "data": xxx }`
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse {
    /// 业务状态码 (字符串格式，兼容前端)
    pub code: String,
    /// 响应消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// 常用业务状态码
#[allow(dead_code)]
pub mod codes {
    /// 成功
    pub const SUCCESS: &str = "200";
    /// 服务器内部错误
    pub const ERROR: &str = "500";
    /// 登录过期/未授权
    pub const UNAUTHORIZED: &str = "A0230";
    /// 参数错误
    pub const BAD_REQUEST: &str = "400";
    /// 资源不存在
    pub const NOT_FOUND: &str = "404";
    /// 禁止访问
    pub const FORBIDDEN: &str = "403";
}

impl ApiResponse {
    /// 创建成功响应（无数据）
    pub fn success() -> Self {
        Self {
            code: codes::SUCCESS.to_string(),
            msg: None,
            data: None,
        }
    }

    /// 创建成功响应（带数据）
    pub fn success_with_data<T: Serialize>(data: T) -> Self {
        Self {
            code: codes::SUCCESS.to_string(),
            msg: None,
            data: Some(serde_json::to_value(data).unwrap_or(Value::Null)),
        }
    }

    /// 创建成功响应（带消息和数据）
    pub fn success_with_msg<T: Serialize>(msg: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code: codes::SUCCESS.to_string(),
            msg: Some(msg.into()),
            data: data.map(|d| serde_json::to_value(d).unwrap_or(Value::Null)),
        }
    }

    /// 创建错误响应
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            code: codes::ERROR.to_string(),
            msg: Some(msg.into()),
            data: None,
        }
    }

    /// 创建带状态码的错误响应
    pub fn error_with_code(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            msg: Some(msg.into()),
            data: None,
        }
    }

    /// 资源不存在响应
    #[allow(dead_code)]
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::error_with_code(codes::NOT_FOUND, msg)
    }

    /// 未授权响应
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::error_with_code(codes::UNAUTHORIZED, msg)
    }

    /// 参数错误响应
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::error_with_code(codes::BAD_REQUEST, msg)
    }

    /// 设置自定义状态码
    #[allow(dead_code)]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }

    /// 设置消息
    pub fn with_msg(mut self, msg: impl Into<String>) -> Self {
        self.msg = Some(msg.into());
        self
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap_or_else(|_| {
            r#"{"code":"500","msg":"序列化响应失败"}"#.to_string()
        });

        let mut response = body.into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        response
    }
}

/// 为 Result 类型提供便捷的响应转换
pub trait IntoApiResponse<T> {
    /// 转换为 ApiResponse
    #[allow(dead_code)]
    fn into_api_response(self) -> ApiResponse;
    /// 转换为带成功消息的 ApiResponse
    fn into_api_response_with_msg(self, success_msg: &str) -> ApiResponse;
}

impl<T, E> IntoApiResponse<T> for Result<T, E>
where
    T: Serialize,
    E: std::fmt::Display,
{
    fn into_api_response(self) -> ApiResponse {
        match self {
            Ok(data) => ApiResponse::success_with_data(data),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    fn into_api_response_with_msg(self, success_msg: &str) -> ApiResponse {
        match self {
            Ok(data) => ApiResponse::success_with_msg(success_msg, Some(data)),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}

// 为了兼容旧代码，提供类型别名
#[allow(dead_code)]
pub type AjaxResult = ApiResponse;
