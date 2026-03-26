use std::sync::Arc;

use axum::{Json, extract::State};
use rulo_common::{result::R, state::AppState};
use rulo_macro::perm;
use serde::Serialize;
use utoipa::ToSchema;

use crate::system::monitor::{model::ServerInfo, service};

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
    pub db: &'static str,
    pub cache: &'static str,
    pub storage: &'static str,
}

#[utoipa::path(
    get, path = "/system/monitor/health",
    responses((status = 200, description = "健康检查", body = HealthResponse)),
)]
pub async fn health_handler(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let db = match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
    {
        Ok(_) => "ok",
        Err(_) => "down",
    };

    let cache = match state.redis_pool.get().await {
        Ok(mut conn) => match redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
        {
            Ok(_) => "ok",
            Err(_) => "down",
        },
        Err(_) => "down",
    };

    let storage = match state
        .s3_bucket
        .list("".to_string(), Some("/".to_string()))
        .await
    {
        Ok(_) => "ok",
        Err(_) => "down",
    };

    let status = if db == "ok" && cache == "ok" && storage == "ok" {
        "healthy"
    } else {
        "degraded"
    };

    Json(HealthResponse { status, db, cache, storage })
}

#[utoipa::path(
    get, path = "/system/monitor/server-info",
    responses((status = 200, description = "success", body = ServerInfo)),
    security(("bearer_auth" = []))
)]
#[perm("sys:monitor:server-info")]
pub async fn server_info_handler() -> R<ServerInfo> {
    tokio::task::spawn_blocking(service::get_server_info)
        .await
        .map_err(|e| rulo_common::error::AppError::Internal(e.to_string()))?
}
