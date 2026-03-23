use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use deadpool_redis::Pool as RedisPool;
use rulo_common::{
    constant::redis_constant,
    model::{IdDto, IdsDto, PageResult},
    result::R,
    util::redis_util,
};

use crate::system::role::service;

use super::model::*;
use rulo_common::state::AppState;
use rulo_macro::perm;

#[utoipa::path(
    post, path = "/system/role/save",
    request_body = SysRoleSaveDto,
    responses((status = 200, description = "success", body = SysRole)),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysRoleSaveDto>,
) -> R<SysRole> {
    service::save(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/role/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    let result = service::remove(&state.db_pool, &dto).await;
    if result.is_ok() {
        for role_id in &dto.ids {
            clear_role_user_cache(&state.redis_pool, &state.db_pool, *role_id).await;
        }
    }
    result
}

#[utoipa::path(
    post, path = "/system/role/update",
    request_body = SysRoleUpdateDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysRoleUpdateDto>,
) -> R<()> {
    service::update(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/role/update-bind-menus",
    request_body = BindMenusDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:update-bind-menus")]
pub async fn update_bind_menus_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<BindMenusDto>,
) -> R<()> {
    let result = service::update_bind_menus(&state.db_pool, &dto).await;
    clear_role_user_cache(&state.redis_pool, &state.db_pool, dto.role_id).await;
    result
}

#[utoipa::path(
    post, path = "/system/role/update-bind-perms",
    request_body = BindPermsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:update-bind-perms")]
pub async fn update_bind_perms_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<BindPermsDto>,
) -> R<()> {
    let result = service::update_bind_perms(&state.db_pool, &dto).await;
    clear_role_user_cache(&state.redis_pool, &state.db_pool, dto.role_id).await;
    result
}

#[utoipa::path(
    get, path = "/system/role/detail",
    params(IdDto),
    responses((status = 200, description = "success", body = SysRole)),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysRole> {
    service::detail(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/role/list",
    params(SysRoleListDto),
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysRoleListDto>,
) -> R<PageResult<SysRole>> {
    service::list(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/role/list-all",
    params(SysRoleListDto),
    responses((status = 200, description = "success", body = Vec<SysRole>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:list")]
pub async fn list_all_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysRoleListDto>,
) -> R<Vec<SysRole>> {
    service::list_all(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/role/list-bind-menus",
    params(IdDto),
    responses((status = 200, description = "success", body = Vec<i64>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:list-bind-menus")]
pub async fn list_bind_menus_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<Vec<i64>> {
    service::list_bind_menus(&state.db_pool, dto.id).await
}

#[utoipa::path(
    get, path = "/system/role/list-bind-perms",
    params(IdDto),
    responses((status = 200, description = "success", body = Vec<i64>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:role:list-bind-perms")]
pub async fn list_bind_perms_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<Vec<i64>> {
    service::list_bind_perms(&state.db_pool, dto.id).await
}

/// 清除该角色下所有用户的权限和菜单缓存
async fn clear_role_user_cache(redis_pool: &RedisPool, db_pool: &sqlx::PgPool, role_id: i64) {
    let user_ids = sqlx::query_scalar!(
        "SELECT user_id FROM sys_user_role WHERE role_id = $1",
        role_id
    )
    .fetch_all(db_pool)
    .await;

    if let Ok(ids) = user_ids {
        for uid in ids {
            let perms_key = redis_constant::USER_PERMS.to_owned() + &uid.to_string();
            let menus_key = redis_constant::USER_MENUS.to_owned() + &uid.to_string();
            let _ = redis_util::del(redis_pool, &perms_key).await;
            let _ = redis_util::del(redis_pool, &menus_key).await;
        }
    }
}
