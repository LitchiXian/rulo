use crate::services::blog_service::BlogService;
use crate::utils::{ApiResponse, IntoApiResponse};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

// 请求体结构体
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ArticleRequest {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub tags: Option<String>,
    pub excerpt: Option<String>,
    pub is_published: Option<u8>,
    pub is_featured: Option<bool>,
}

#[derive(Deserialize)]
pub struct ArticleIdRequest {
    pub id: String,
}

#[derive(Deserialize)]
pub struct UserArticleRequest {
    pub id: String,
}

// 保存文章处理函数
pub async fn save_article_handler(
    State((auth_service, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    headers: HeaderMap,
    Json(payload): Json<ArticleRequest>,
) -> impl IntoResponse {
    // 从 header 获取 token 并获取用户ID
    let token = headers
        .get("satoken")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    if token.is_empty() {
        return ApiResponse::unauthorized("请先登录");
    }
    
    // 从 token 获取用户信息
    let user_info = match auth_service.get_user_by_token(token).await {
        Ok(info) => info,
        Err(_) => return ApiResponse::unauthorized("登录已过期"),
    };
    
    let user_id = match user_info.id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return ApiResponse::error("用户ID格式错误"),
    };
    
    blog_service
        .save_article(
            payload.title,
            payload.content,
            user_id,
            payload.is_published.unwrap_or(0) as i32,
        )
        .await
        .into_api_response_with_msg("保存成功")
}

// 删除文章处理函数
pub async fn remove_article_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Json(payload): Json<ArticleIdRequest>,
) -> impl IntoResponse {
    match blog_service.delete_article(&payload.id).await {
        Ok(_) => ApiResponse::success().with_msg("删除成功"),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

// 更新文章处理函数
pub async fn update_article_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Json(payload): Json<ArticleRequest>,
) -> impl IntoResponse {
    let Some(id) = payload.id else {
        return ApiResponse::bad_request("文章ID不能为空");
    };
    
    blog_service
        .update_article(
            &id,
            payload.title,
            payload.content,
            payload.is_published.unwrap_or(0) as i32,
        )
        .await
        .into_api_response_with_msg("更新成功")
}

// 获取文章列表处理函数
pub async fn list_articles_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
) -> impl IntoResponse {
    blog_service
        .get_article_list()
        .await
        .into_api_response_with_msg("获取成功")
}

// 获取文章详情处理函数
pub async fn get_article_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Query(params): Query<ArticleIdRequest>,
) -> impl IntoResponse {
    blog_service
        .get_article_by_id(&params.id)
        .await
        .into_api_response_with_msg("获取成功")
}

// 获取用户文章列表处理函数
pub async fn get_user_articles_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Query(params): Query<UserArticleRequest>,
) -> impl IntoResponse {
    blog_service
        .get_articles_by_user_id(&params.id)
        .await
        .into_api_response_with_msg("获取成功")
}
