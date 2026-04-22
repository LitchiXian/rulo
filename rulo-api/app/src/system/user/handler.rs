use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Extension;
use common::{
    constant::redis_constant,
    extractor::ValidatedJson,
    model::{IdDto, IdsDto, IsSuperAdmin, PageResult},
    result::R,
    util::redis_util,
};
use macros::perm;

use crate::system::user::service;

use super::model::*;
use common::state::AppState;

#[utoipa::path(
    post, path = "/system/user/save",
    request_body = SysUserSaveDto,
    responses((status = 200, description = "success", body = SysUser)),
    security(("bearer_auth" = []))
)]
#[perm("sys:user:save")]
pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<SysUserSaveDto>,
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
pub async fn remove_handler(
    State(state): State<Arc<AppState>>,
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    ValidatedJson(dto): ValidatedJson<IdsDto>,
) -> R<()> {
    let result = service::remove(&state.db_pool, &dto, caller_is_super).await;
    if result.is_ok() {
        // 清理被删除用户的所有身份缓存，配合 query_user_auth 的存活检查可实现下次请求立即下线
        for user_id in &dto.ids {
            let suffix = user_id.to_string();
            let _ = redis_util::del(
                &state.redis_pool,
                &(redis_constant::USER_AUTH.to_owned() + &suffix),
            )
            .await;
            let _ = redis_util::del(
                &state.redis_pool,
                &(redis_constant::USER_MENUS.to_owned() + &suffix),
            )
            .await;
            let _ = redis_util::del(
                &state.redis_pool,
                &(redis_constant::USER_INFO.to_owned() + &suffix),
            )
            .await;
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
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    ValidatedJson(dto): ValidatedJson<SysUserUpdateDto>,
) -> R<()> {
    let user_id = dto.id;
    let result = service::update(&state.db_pool, &state.storage_config, &dto, caller_is_super).await;
    if result.is_ok() {
        let info_key = redis_constant::USER_INFO.to_owned() + &user_id.to_string();
        let _ = redis_util::del(&state.redis_pool, &info_key).await;
    }
    result
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
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    ValidatedJson(dto): ValidatedJson<BindRolesDto>,
) -> R<()> {
    let user_id = dto.user_id;
    let result = service::update_bind_roles(&state.db_pool, &dto, caller_is_super).await;
    // 清除该用户的鉴权上下文和菜单缓存
    let auth_key = redis_constant::USER_AUTH.to_owned() + &user_id.to_string();
    let menus_key = redis_constant::USER_MENUS.to_owned() + &user_id.to_string();
    let _ = redis_util::del(&state.redis_pool, &auth_key).await;
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
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    Query(dto): Query<IdDto>,
) -> R<SysUser> {
    service::detail(
        &state.db_pool,
        &state.s3_bucket,
        &state.storage_config,
        &dto,
        caller_is_super,
    )
    .await
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
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    Query(dto): Query<SysUserListDto>,
) -> R<PageResult<SysUser>> {
    service::list(
        &state.db_pool,
        &state.s3_bucket,
        &state.storage_config,
        &dto,
        caller_is_super,
    )
    .await
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
    Extension(IsSuperAdmin(caller_is_super)): Extension<IsSuperAdmin>,
    Query(dto): Query<IdDto>,
) -> R<Vec<i64>> {
    service::list_bind_roles(&state.db_pool, dto.id, caller_is_super).await
}
