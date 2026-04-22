use std::sync::Arc;

use axum::extract::{Query, State};
use common::{
    extractor::ValidatedJson,
    model::{IdDto, IdsDto},
    result::R,
};
use macros::perm;

use super::model::*;
use crate::system::dept::service;
use common::state::AppState;

#[utoipa::path(
    post, path = "/system/dept/save",
    request_body = SysDeptSaveDto,
    responses((status = 200, description = "success", body = SysDept)),
    security(("bearer_auth" = []))
)]
#[perm("sys:dept:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<SysDeptSaveDto>,
) -> R<SysDept> {
    service::save(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/dept/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:dept:remove")]
pub async fn remove_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<IdsDto>,
) -> R<()> {
    service::remove(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/dept/update",
    request_body = SysDeptUpdateDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:dept:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<SysDeptUpdateDto>,
) -> R<()> {
    service::update(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/dept/detail",
    params(IdDto),
    responses((status = 200, description = "success", body = SysDept)),
    security(("bearer_auth" = []))
)]
#[perm("sys:dept:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysDept> {
    service::detail(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/dept/list-all",
    params(SysDeptListDto),
    responses((status = 200, description = "success", body = Vec<SysDept>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:dept:list")]
pub async fn list_all_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysDeptListDto>,
) -> R<Vec<SysDept>> {
    service::list_all(&state.db_pool, &dto).await
}
