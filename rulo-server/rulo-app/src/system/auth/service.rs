use crate::system::user::{
    model::{SysUser, SysUserSaveDto},
    service,
};
use deadpool_redis::Pool;
use rulo_common::{
    error::AppError,
    result::{R, success},
    util::{jwt_util, password_util},
};
use sqlx::{PgPool, query_as};
use tracing::info;

use crate::system::auth::model::AuthUserDto;

pub async fn login(db_pool: &PgPool, redis_pool: &Pool, dto: &AuthUserDto) -> R<String> {
    info!("login auth_user {:?}", &dto);
    let db_user = query_as!(
        SysUser,
        "SELECT * FROM sys_user
        WHERE user_name = $1 AND is_deleted = false",
        &dto.username
    )
    .fetch_optional(db_pool)
    .await?
    .ok_or_else(|| AppError::ServiceError("用户名或密码有误".to_string()))?;
    info!("login user {:?}", &db_user);

    if db_user.is_active == false {
        return Err(AppError::ServiceError(
            "账号已冻结,请联系管理员解冻".to_string(),
        ));
    }

    if !password_util::verify_password(&dto.password, &db_user.password)? {
        return Err(AppError::ServiceError("用户名或密码有误".to_string()));
    }

    let token =
        jwt_util::generate_token(db_user.id).map_err(|e| AppError::Internal(e.to_string()))?;

    // TODO 存入redis

    success(token)
}

pub async fn register(db_pool: &PgPool, dto: &AuthUserDto) -> R<()> {
    info!("register auth_user {:?}", &dto);
    if query_as!(
        SysUser,
        "SELECT * FROM sys_user
        WHERE user_name = $1 AND is_deleted = false",
        &dto.username
    )
    .fetch_optional(db_pool)
    .await?
    .is_some()
    {
        return Err(AppError::ServiceError(
            "用户名已被占用,请换一个试试".to_string(),
        ));
    }

    let hash_password = password_util::hash_password(&dto.password)?;

    let user_save_dto = SysUserSaveDto {
        nick_name: dto.username.clone(),
        password: hash_password,
        email: dto.email.clone(),
        remark: None,
    };
    service::save_handle(db_pool, &user_save_dto).await?;

    success(())
}

pub fn logout() {}

pub fn info() {}
