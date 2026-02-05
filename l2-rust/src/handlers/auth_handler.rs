use crate::services::auth_service::AuthService;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// 请求体结构体 - 兼容前端 userName 字段
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub user_name: String,
    pub password: String,
    pub email: String,
    #[serde(alias = "code")]  // 兼容前端发送的 code 字段
    pub verification_code: Option<String>,
}

#[derive(Deserialize)]
pub struct VerificationCodeRequest {
    pub email: String,
}

// 统一响应体结构体 - 匹配Java的AjaxResult格式
// Java格式: { "code": "200", "msg": "xxx", "data": xxx }
#[derive(Serialize)]
pub struct AjaxResult {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl AjaxResult {
    pub fn success(data: Option<serde_json::Value>) -> Self {
        Self {
            code: "200".to_string(),
            msg: None,
            data,
        }
    }

    pub fn success_with_msg(msg: &str, data: Option<serde_json::Value>) -> Self {
        Self {
            code: "200".to_string(),
            msg: Some(msg.to_string()),
            data,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            code: "500".to_string(),
            msg: Some(msg.to_string()),
            data: None,
        }
    }

    pub fn error_with_code(code: &str, msg: &str) -> Self {
        Self {
            code: code.to_string(),
            msg: Some(msg.to_string()),
            data: None,
        }
    }
}

// 为 AjaxResult 实现 IntoResponse trait
impl IntoResponse for AjaxResult {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        let mut response = body.into_response();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::header::HeaderValue::from_static("application/json"),
        );
        response
    }
}

// 登录处理函数 - 返回token字符串
pub async fn login_handler(
    State((auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = auth_service
        .login(payload.user_name, payload.password)
        .await;
    match result {
        Ok((_user, token)) => {
            // 返回格式与Java一致: { code: "200", data: "token字符串" }
            AjaxResult::success(Some(serde_json::json!(token)))
        }
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 注册处理函数
pub async fn register_handler(
    State((auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let result = auth_service
        .register(
            payload.user_name,
            payload.password,
            payload.email,
            payload.verification_code.unwrap_or_default(),
        )
        .await;
    match result {
        Ok(_) => AjaxResult::success(None),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 获取验证码处理函数
pub async fn get_register_code_handler(
    State((auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
    Json(payload): Json<VerificationCodeRequest>,
) -> impl IntoResponse {
    let result = auth_service.send_verification_code(&payload.email).await;
    match result {
        Ok(_) => AjaxResult::success_with_msg("验证码发送成功", None),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 登出处理函数
pub async fn logout_handler(
    State((_auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
) -> impl IntoResponse {
    // 这里简化处理，实际应该有token验证等逻辑
    AjaxResult::success(None)
}

// 获取登录信息处理函数
pub async fn get_login_info_handler(
    State((auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // 从header获取token
    let token = headers
        .get("satoken")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if token.is_empty() {
        return AjaxResult::error_with_code("A0230", "登录已过期");
    }

    // 根据token获取用户信息
    match auth_service.get_user_by_token(token).await {
        Ok(user_info) => AjaxResult::success(Some(serde_json::to_value(user_info).unwrap())),
        Err(_) => AjaxResult::error_with_code("A0230", "登录已过期"),
    }
}
