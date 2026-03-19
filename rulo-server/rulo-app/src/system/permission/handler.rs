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

#[perm("sys:permission:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysPermissionSaveDto>,
) -> R<SysPermission> {
    service::save_handle(&state.db_pool, &dto).await
}

#[perm("sys:permission:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    service::remove_handle(&state.db_pool, &dto).await
}

#[perm("sys:permission:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysPermissionUpdateDto>,
) -> R<()> {
    service::update_handle(&state.db_pool, &dto).await
}

#[perm("sys:permission:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysPermission> {
    service::get_one_handle(&state.db_pool, &dto).await
}

#[perm("sys:permission:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysPermissionListDto>,
) -> R<Vec<SysPermission>> {
    service::list_handle(&state.db_pool, &dto).await
}
