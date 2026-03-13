use crate::system::user::{
    model::{SysUser, SysUserSaveDto},
    service,
};
use deadpool_redis::Pool;
use rulo_common::{
    constant::redis_constant,
    error::AppError,
    result::{R, success},
    util::{jwt_util, password_util, redis_util},
};
use sqlx::{PgPool, query_as};
use tracing::info;

use crate::system::auth::model::AuthUserDto;

pub async fn login(db_pool: &PgPool, redis_pool: &Pool, dto: &AuthUserDto) -> R<String> {
    info!("login auth_user {:?}", &dto);

    // validate input parameters
    if dto.password.is_empty() || dto.username.is_empty() {
        return Err(AppError::ServiceError("用户名或密码不能为空".to_string()));
    }

    // validate user existence
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

    // validate that user can perform normal operations
    if db_user.is_active == false {
        return Err(AppError::ServiceError(
            "账号已冻结,请联系管理员解冻".to_string(),
        ));
    }

    // validate password against stored hash
    if !password_util::verify_password(&dto.password, &db_user.password)? {
        return Err(AppError::ServiceError("用户名或密码有误".to_string()));
    }

    // generate token
    let token =
        jwt_util::generate_token(db_user.id).map_err(|e| AppError::Internal(e.to_string()))?;

    // save token to Redis
    redis_util::set_obj(
        redis_pool,
        &(redis_constant::USER_TOKEN.to_owned() + &token),
        &db_user.id,
        redis_constant::ONE_DAY,
    )
    .await?;

    // save user info to Redis
    redis_util::set_obj(
        redis_pool,
        &(redis_constant::USER_INFO.to_owned() + &db_user.id.to_string()),
        &db_user,
        redis_constant::ONE_DAY,
    )
    .await?;

    success(token)
}

pub async fn register(db_pool: &PgPool, dto: &AuthUserDto) -> R<()> {
    info!("register auth_user {:?}", &dto);

    // validate input parameters
    if dto.password.is_empty() || dto.username.is_empty() {
        return Err(AppError::ServiceError("用户名或密码不能为空".to_string()));
    }

    // validate user existence
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

    // generate hash password
    let hash_password = password_util::hash_password(&dto.password)?;

    // create user account
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
