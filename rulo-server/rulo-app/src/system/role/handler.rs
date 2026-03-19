use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use rulo_common::{
    model::{IdDto, IdsDto},
    result::R,
};

use crate::system::role::service;

use super::model::*;
use rulo_common::state::AppState;
use rulo_macro::perm;

#[perm("sys:role:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysRoleSaveDto>,
) -> R<SysRole> {
    service::save_handle(&state.db_pool, &dto).await
}

#[perm("sys:role:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    service::remove_handle(&state.db_pool, &dto).await
}

#[perm("sys:role:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysRoleUpdateDto>,
) -> R<()> {
    service::update_handle(&state.db_pool, &dto).await
}

#[perm("sys:role:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysRole> {
    service::get_one_handle(&state.db_pool, &dto).await
}

#[perm("sys:role:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysRoleListDto>,
) -> R<Vec<SysRole>> {
    service::list_handle(&state.db_pool, &dto).await
}
