use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use rulo_common::{
    model::{IdDto, IdsDto},
    result::R,
};

use crate::system::permission::service;

use super::model::*;
use rulo_common::state::AppState;
use rulo_macro::perm;

#[utoipa::path(
    post, path = "/system/permission/save",
    request_body = SysPermissionSaveDto,
    responses((status = 200, description = "success", body = SysPermission)),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysPermissionSaveDto>,
) -> R<SysPermission> {
    service::save_handle(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/permission/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    service::remove_handle(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/permission/update",
    request_body = SysPermissionUpdateDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysPermissionUpdateDto>,
) -> R<()> {
    service::update_handle(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/permission/detail",
    params(IdDto),
    responses((status = 200, description = "success", body = SysPermission)),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysPermission> {
    service::get_one_handle(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/permission/list",
    params(SysPermissionListDto),
    responses((status = 200, description = "success", body = Vec<SysPermission>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysPermissionListDto>,
) -> R<Vec<SysPermission>> {
    service::list_handle(&state.db_pool, &dto).await
}
