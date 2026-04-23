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
use crate::system::permission::service;

use super::model::*;
use common::state::AppState;
use macros::perm;

#[utoipa::path(
    post, path = "/system/permission/save",
    request_body = SysPermissionSaveDto,
    responses((status = 200, description = "success", body = SysPermission)),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    ValidatedJson(dto): ValidatedJson<SysPermissionSaveDto>,
) -> R<SysPermission> {
    service::save(&state.db_pool, &dto, caller_is_super).await
}

#[utoipa::path(
    post, path = "/system/permission/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>, ValidatedJson(dto): ValidatedJson<IdsDto>) -> R<()> {
    let result = service::remove(&state.db_pool, &dto, caller_is_super).await;
    if result.is_ok() {
        cache::invalidate_all_users_authz(&state.redis_pool, &state.db_pool).await;
    }
    result
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
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    ValidatedJson(dto): ValidatedJson<SysPermissionUpdateDto>,
) -> R<()> {
    let result = service::update(&state.db_pool, &dto, caller_is_super).await;
    if result.is_ok() {
        cache::invalidate_all_users_authz(&state.redis_pool, &state.db_pool).await;
    }
    result
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
    service::detail(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/permission/list",
    params(SysPermissionListDto),
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysPermissionListDto>,
) -> R<PageResult<SysPermission>> {
    service::list(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/permission/list-all",
    params(SysPermissionListDto),
    responses((status = 200, description = "success", body = Vec<SysPermission>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:list")]
pub async fn list_all_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysPermissionListDto>,
) -> R<Vec<SysPermission>> {
    service::list_all(&state.db_pool, &dto).await
}
