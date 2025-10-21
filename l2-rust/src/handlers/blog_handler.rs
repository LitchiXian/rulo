use crate::services::blog_service::BlogService;
use axum::response::{IntoResponse, Response};
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// 请求体结构体
#[derive(Deserialize)]
pub struct ArticleRequest {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub user_id: i64,
    pub status: Option<i32>,
}

#[derive(Deserialize)]
pub struct ArticleIdRequest {
    pub id: String,
}

#[derive(Deserialize)]
pub struct UserArticleRequest {
    pub id: String,
}

// 响应体结构体
#[derive(Serialize)]
pub struct ArticleResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// 为 ArticleResponse 实现 IntoResponse trait
impl IntoResponse for ArticleResponse {
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

// 保存文章处理函数
pub async fn save_article_handler(
    State((_, blog_service)): State<(
        Arc<crate::services::auth_service::AuthService>,
        Arc<BlogService>,
    )>,
    Json(payload): Json<ArticleRequest>,
) -> impl IntoResponse {
    let result = blog_service
        .save_article(
            payload.title,
            payload.content,
            payload.user_id,
            payload.status.unwrap_or(1),
        )
        .await;
    match result {
        Ok(article) => ArticleResponse {
            code: 200,
            message: "保存成功".to_string(),
            data: Some(serde_json::to_value(article).unwrap()),
        },
        Err(e) => ArticleResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
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
        Ok(_) => ArticleResponse {
            code: 200,
            message: "删除成功".to_string(),
            data: None,
        },
        Err(e) => ArticleResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
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
                payload.status.unwrap_or(1),
            )
            .await;
        match result {
            Ok(article) => ArticleResponse {
                code: 200,
                message: "更新成功".to_string(),
                data: Some(serde_json::to_value(article).unwrap()),
            },
            Err(e) => ArticleResponse {
                code: 400,
                message: e.to_string(),
                data: None,
            },
        }
    } else {
        ArticleResponse {
            code: 400,
            message: "文章ID不能为空".to_string(),
            data: None,
        }
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
        Ok(articles) => ArticleResponse {
            code: 200,
            message: "获取成功".to_string(),
            data: Some(serde_json::to_value(articles).unwrap()),
        },
        Err(e) => ArticleResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
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
        Ok(article) => ArticleResponse {
            code: 200,
            message: "获取成功".to_string(),
            data: Some(serde_json::to_value(article).unwrap()),
        },
        Err(e) => ArticleResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
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
        Ok(articles) => ArticleResponse {
            code: 200,
            message: "获取成功".to_string(),
            data: Some(serde_json::to_value(articles).unwrap()),
        },
        Err(e) => ArticleResponse {
            code: 400,
            message: e.to_string(),
            data: None,
        },
    }
}
