use std::sync::Arc;

use axum::{
    extract::{Query, State},
};
use common::{
    extractor::ValidatedJson,
    model::{IdDto, IdsDto, PageResult},
    result::R,
};

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
    ValidatedJson(dto): ValidatedJson<SysPermissionSaveDto>,
) -> R<SysPermission> {
    service::save(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/permission/remove",
    request_body = IdsDto,
    responses((status = 200, description = "success")),
    security(("bearer_auth" = []))
)]
#[perm("sys:permission:remove")]
pub async fn remove_handler(State(state): State<Arc<AppState>>, ValidatedJson(dto): ValidatedJson<IdsDto>) -> R<()> {
    let result = service::remove(&state.db_pool, &dto).await;
    // 清除所有受影响角色的用户缓存
    if result.is_ok() {
        clear_all_role_user_cache(&state.redis_pool, &state.db_pool).await;
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
    ValidatedJson(dto): ValidatedJson<SysPermissionUpdateDto>,
) -> R<()> {
    let result = service::update(&state.db_pool, &dto).await;
    if result.is_ok() {
        clear_all_role_user_cache(&state.redis_pool, &state.db_pool).await;
    }
    result
}
/// 权限定义变动后清理所有已绑定角色的用户鉴权上下文与菜单缓存。
async fn clear_all_role_user_cache(redis_pool: &deadpool_redis::Pool, db_pool: &sqlx::PgPool) {
    let user_ids = sqlx::query_scalar!("SELECT DISTINCT user_id FROM sys_user_role").fetch_all(db_pool).await;
    if let Ok(ids) = user_ids {
        for uid in ids {
            let auth_key = common::constant::redis_constant::USER_AUTH.to_owned() + &uid.to_string();
            let menus_key = common::constant::redis_constant::USER_MENUS.to_owned() + &uid.to_string();
            let _ = common::util::redis_util::del(redis_pool, &auth_key).await;
            let _ = common::util::redis_util::del(redis_pool, &menus_key).await;
        }
    }
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
