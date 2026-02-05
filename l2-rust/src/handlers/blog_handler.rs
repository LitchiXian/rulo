use crate::handlers::auth_handler::AjaxResult;
use crate::services::blog_service::BlogService;
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
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Json(payload): Json<ArticleRequest>,
) -> impl IntoResponse {
    // TODO: 从token获取真实的user_id，这里暂时使用固定值
    let user_id = 1i64;
    
    let result = blog_service
        .save_article(
            payload.title,
            payload.content,
            user_id,
            payload.is_published.unwrap_or(0) as i32,
        )
        .await;
    match result {
        Ok(article) => AjaxResult::success_with_msg("保存成功", Some(serde_json::to_value(article).unwrap())),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 删除文章处理函数
pub async fn remove_article_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Json(payload): Json<ArticleIdRequest>,
) -> impl IntoResponse {
    let result = blog_service.delete_article(&payload.id).await;
    match result {
        Ok(_) => AjaxResult::success_with_msg("删除成功", None),
        Err(e) => AjaxResult::error(&e.to_string()),
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
    if let Some(id) = payload.id {
        let result = blog_service
            .update_article(
                &id,
                payload.title,
                payload.content,
                payload.is_published.unwrap_or(0) as i32,
            )
            .await;
        match result {
            Ok(article) => AjaxResult::success_with_msg("更新成功", Some(serde_json::to_value(article).unwrap())),
            Err(e) => AjaxResult::error(&e.to_string()),
        }
    } else {
        AjaxResult::error("文章ID不能为空")
    }
}

// 获取文章列表处理函数
pub async fn list_articles_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
) -> impl IntoResponse {
    let result = blog_service.get_article_list().await;
    match result {
        Ok(articles) => AjaxResult::success_with_msg("获取成功", Some(serde_json::to_value(articles).unwrap())),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 获取文章详情处理函数
pub async fn get_article_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Query(params): Query<ArticleIdRequest>,
) -> impl IntoResponse {
    let result = blog_service.get_article_by_id(&params.id).await;
    match result {
        Ok(article) => AjaxResult::success_with_msg("获取成功", Some(serde_json::to_value(article).unwrap())),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}

// 获取用户文章列表处理函数
pub async fn get_user_articles_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Query(params): Query<UserArticleRequest>,
) -> impl IntoResponse {
    let result = blog_service.get_articles_by_user_id(&params.id).await;
    match result {
        Ok(articles) => AjaxResult::success_with_msg("获取成功", Some(serde_json::to_value(articles).unwrap())),
        Err(e) => AjaxResult::error(&e.to_string()),
    }
}
