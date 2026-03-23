use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use rulo_common::{
    model::{IdDto, IdsDto, PageResult},
    result::R,
};

use crate::system::menu::service;

use super::model::*;
use rulo_common::state::AppState;
use rulo_macro::perm;

#[utoipa::path(
    post, path = "/system/menu/save",
    request_body = SysMenuSaveDto,
    responses((status = 200, description = "success", body = SysMenu)),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysMenuSaveDto>,
) -> R<SysMenu> {
    service::save(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/menu/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    service::remove(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/menu/update",
    request_body = SysMenuUpdateDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysMenuUpdateDto>,
) -> R<()> {
    service::update(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/menu/detail",
    params(IdDto),
    responses((status = 200, description = "success", body = SysMenu)),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysMenu> {
    service::detail(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/menu/list",
    params(SysMenuListDto),
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysMenuListDto>,
) -> R<PageResult<SysMenu>> {
    service::list(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/menu/list-all",
    params(SysMenuListDto),
    responses((status = 200, description = "success", body = Vec<SysMenu>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:list")]
pub async fn list_all_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysMenuListDto>,
) -> R<Vec<SysMenu>> {
    service::list_all(&state.db_pool, &dto).await
}
