use crate::services::auth_service::AuthService;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// 请求体结构体
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub verification_code: String,
}

#[derive(Deserialize)]
pub struct VerificationCodeRequest {
    pub email: String,
}

// 响应体结构体
#[derive(Serialize)]
pub struct LoginResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<LoginData>,
}

// 为 LoginResponse 实现 IntoResponse trait
impl IntoResponse for LoginResponse {
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

#[derive(Serialize)]
pub struct LoginData {
    pub user_id: u64,
    pub user_name: String,
    pub token: String,
}

// 登录处理函数
pub async fn login_handler(
    State((auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = auth_service.login(payload.username, payload.password).await;
    match result {
        Ok((user, token)) => LoginResponse {
            code: 200,
            message: "登录成功".to_string(),
            data: Some(LoginData {
                user_id: user.id,
                user_name: user.user_name,
                token,
            }),
        },
        Err(e) => LoginResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
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
            payload.username,
            payload.password,
            payload.email,
            payload.verification_code,
        )
        .await;
    match result {
        Ok(_) => LoginResponse {
            code: 200,
            message: "注册成功".to_string(),
            data: None,
        },
        Err(e) => LoginResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
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
        Ok(_) => LoginResponse {
            code: 200,
            message: "验证码发送成功".to_string(),
            data: None,
        },
        Err(e) => LoginResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
    }
}

// 登出处理函数
pub async fn logout_handler(
    State((auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
) -> impl IntoResponse {
    // 这里简化处理，实际应该有token验证等逻辑
    LoginResponse {
        code: 200,
        message: "登出成功".to_string(),
        data: None,
    }
}

// 获取登录信息处理函数
pub async fn get_login_info_handler(
    State((auth_service, _)): State<(
        Arc<AuthService>,
        Arc<crate::services::blog_service::BlogService>,
    )>,
) -> impl IntoResponse {
    // 这里简化处理，实际应该有token验证和用户信息查询逻辑
    LoginResponse {
        code: 200,
        message: "获取成功".to_string(),
        data: None,
    }
}
