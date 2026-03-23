use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use rulo_common::{
    constant::redis_constant,
    model::{IdDto, IdsDto, PageResult},
    result::R,
    util::redis_util,
};
use rulo_macro::perm;

use crate::system::user::service;

use super::model::*;
use rulo_common::state::AppState;

#[utoipa::path(
    post, path = "/system/user/save",
    request_body = SysUserSaveDto,
    responses((status = 200, description = "success", body = SysUser)),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysUserSaveDto>,
) -> R<SysUser> {
    service::save(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/user/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    let result = service::remove(&state.db_pool, &dto).await;
    if result.is_ok() {
        for user_id in &dto.ids {
            let perms_key = rulo_common::constant::redis_constant::USER_PERMS.to_owned() + &user_id.to_string();
            let menus_key = rulo_common::constant::redis_constant::USER_MENUS.to_owned() + &user_id.to_string();
            let _ = rulo_common::util::redis_util::del(&state.redis_pool, &perms_key).await;
            let _ = rulo_common::util::redis_util::del(&state.redis_pool, &menus_key).await;
        }
    }
    result
}

#[utoipa::path(
    post, path = "/system/user/update",
    request_body = SysUserUpdateDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:update")]
pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysUserUpdateDto>,
) -> R<()> {
    service::update(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/user/update-bind-roles",
    request_body = BindRolesDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:update-bind-roles")]
pub async fn update_bind_roles_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<BindRolesDto>,
) -> R<()> {
    let user_id = dto.user_id;
    let result = service::update_bind_roles(&state.db_pool, &dto).await;
    // 清除该用户的权限和菜单缓存
    let perms_key = redis_constant::USER_PERMS.to_owned() + &user_id.to_string();
    let menus_key = redis_constant::USER_MENUS.to_owned() + &user_id.to_string();
    let _ = redis_util::del(&state.redis_pool, &perms_key).await;
    let _ = redis_util::del(&state.redis_pool, &menus_key).await;
    result
}

#[utoipa::path(
    get, path = "/system/user/detail",
    params(IdDto),
    responses((status = 200, description = "success", body = SysUser)),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:detail")]
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysUser> {
    service::detail(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/user/list",
    params(SysUserListDto),
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:list")]
pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysUserListDto>,
) -> R<PageResult<SysUser>> {
    service::list(&state.db_pool, &dto).await
}

#[utoipa::path(
    get, path = "/system/user/list-bind-roles",
    params(IdDto),
    responses((status = 200, description = "success", body = Vec<i64>)),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:list-bind-roles")]
pub async fn list_bind_roles_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<Vec<i64>> {
    service::list_bind_roles(&state.db_pool, dto.id).await
}
