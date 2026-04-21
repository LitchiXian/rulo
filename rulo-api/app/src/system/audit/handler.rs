use std::sync::Arc;

use axum::extract::{Query, State};
use common::{
    model::PageResult,
    result::R,
};
use macros::perm;

use super::model::*;
use super::service;
use common::state::AppState;

#[utoipa::path(
    get, path = "/system/audit/list",
    params(AuditLogListDto),
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:audit:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<AuditLogListDto>,
) -> R<PageResult<SysAuditLog>> {
    service::list(&state.db_pool, &dto).await
}
