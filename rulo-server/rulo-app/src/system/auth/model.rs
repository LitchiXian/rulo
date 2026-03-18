use serde::{Deserialize, Serialize};

use crate::system::{menu::model::SysMenu, user::model::SysUser};

#[derive(Debug, Deserialize)]
pub struct AuthUserDto {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserId(pub i64);

#[derive(Debug, Clone)]
pub struct UserToken(pub String);

#[derive(Debug, Clone)]
pub struct UserPermissions(pub Vec<String>);

#[derive(Debug, Serialize)]
pub struct LoginUserInfo {
    pub user_info: SysUser,
    pub perms: Vec<String>,
    pub menus: Vec<SysMenu>,
}
