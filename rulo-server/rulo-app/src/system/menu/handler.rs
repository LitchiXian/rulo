use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use rulo_common::{
    model::{IdDto, IdsDto},
    result::R,
};

use crate::system::menu::service;

use super::model::*;
use rulo_common::state::AppState;
use rulo_macro::perm;

#[perm("sys:menu:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysMenuSaveDto>,
) -> R<SysMenu> {
    service::save_handle(&state.db_pool, &dto).await
}

#[perm("sys:menu:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    service::remove_handle(&state.db_pool, &dto).await
}

#[perm("sys:menu:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysMenuUpdateDto>,
) -> R<()> {
    service::update_handle(&state.db_pool, &dto).await
}

#[perm("sys:menu:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysMenu> {
    service::get_one_handle(&state.db_pool, &dto).await
}

#[perm("sys:menu:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysMenuListDto>,
) -> R<Vec<SysMenu>> {
    service::list_handle(&state.db_pool, &dto).await
}
