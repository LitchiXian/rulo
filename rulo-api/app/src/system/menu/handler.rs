use std::sync::Arc;

use axum::{
    Extension,
    extract::{Query, State},
};
use common::{
    extractor::ValidatedJson,
    model::{IdDto, IdsDto, IsSuperAdmin, PageResult},
    result::R,
};

use crate::system::auth::cache;
use crate::system::menu::service;

use super::model::*;
use common::state::AppState;
use macros::perm;

#[utoipa::path(
    post, path = "/system/menu/save",
    request_body = SysMenuSaveDto,
    responses((status = 200, description = "success", body = SysMenu)),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    ValidatedJson(dto): ValidatedJson<SysMenuSaveDto>,
) -> R<SysMenu> {
    let result = service::save(&state.db_pool, &dto, caller_is_super).await;
    if result.is_ok() {
        cache::invalidate_all_users_menus(&state.redis_pool, &state.db_pool).await;
    }
    result
}

#[utoipa::path(
    post, path = "/system/menu/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:menu:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>, ValidatedJson(dto): ValidatedJson<IdsDto>) -> R<()> {
    let result = service::remove(&state.db_pool, &dto, caller_is_super).await;
    if result.is_ok() {
        cache::invalidate_all_users_menus(&state.redis_pool, &state.db_pool).await;
    }
    result
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
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    ValidatedJson(dto): ValidatedJson<SysMenuUpdateDto>,
) -> R<()> {
    let result = service::update(&state.db_pool, &dto, caller_is_super).await;
    if result.is_ok() {
        cache::invalidate_all_users_menus(&state.redis_pool, &state.db_pool).await;
    }
    result
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
