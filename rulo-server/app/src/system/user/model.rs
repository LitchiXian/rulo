use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysUser {
    pub id: i64,
    pub user_name: String,
    pub nick_name: String,
    // #[serde(skip)] // 序列化和反序列化都跳过
    #[serde(skip_serializing, skip_deserializing)]
    #[schema(ignore)]
    pub password: String,
    pub email: Option<String>,
    pub is_active: bool,
    pub is_deleted: bool,
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
    pub avatar_url: Option<String>,
}

impl SysUser {
    pub fn new_user_from_save_dto(dto: &SysUserSaveDto) -> Self {
        let user_id: i64 = rulo_common::util::id_util::next_id();
        let now_time = Utc::now();

        SysUser {
            id: user_id,
            user_name: dto.user_name.clone(),
            nick_name: dto.nick_name.clone(),
            password: dto.password.clone(),
            email: dto.email.clone(),
            is_active: true,
            is_deleted: false,
            create_id: user_id,
            create_time: now_time.clone(),
            update_id: user_id,
            update_time: now_time.clone(),
            remark: dto.remark.clone(),
            avatar_url: None,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysUserSaveDto {
    #[validate(length(min = 2, max = 30, message = "用户名长度必须在 2-30 之间"))]
    pub user_name: String,
    #[validate(length(min = 1, max = 30, message = "昵称长度必须在 1-30 之间"))]
    pub nick_name: String,
    #[validate(length(min = 6, max = 64, message = "密码长度必须在 6-64 之间"))]
    pub password: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct SysUserUpdateDto {
    pub id: i64,
    #[validate(length(min = 1, max = 30, message = "昵称长度必须在 1-30 之间"))]
    pub nick_name: Option<String>,
    #[validate(length(min = 6, max = 64, message = "密码长度必须在 6-64 之间"))]
    pub password: Option<String>,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    #[validate(length(max = 500, message = "备注长度不能超过 500"))]
    pub remark: Option<String>,
    #[validate(length(max = 500, message = "头像 URL 长度不能超过 500"))]
    pub avatar_url: Option<String>,
}

#[derive(Deserialize, Debug, IntoParams, ToSchema)]
pub struct SysUserListDto {
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
    pub remark: Option<String>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct BindRolesDto {
    pub user_id: i64,
    pub role_ids: Vec<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_user_from_save_dto_sets_defaults() {
        let dto = SysUserSaveDto {
            user_name: "testuser".to_string(),
            nick_name: "Test".to_string(),
            password: "hashed_pw".to_string(),
            email: Some("a@b.com".to_string()),
            remark: None,
        };
        let user = SysUser::new_user_from_save_dto(&dto);
        assert_eq!(user.user_name, "testuser");
        assert_eq!(user.nick_name, "Test");
        assert_eq!(user.password, "hashed_pw");
        assert_eq!(user.email, Some("a@b.com".to_string()));
        assert!(user.is_active);
        assert!(!user.is_deleted);
        assert!(user.avatar_url.is_none());
        assert!(user.id != 0);
    }

    #[test]
    fn new_user_generates_unique_ids() {
        let dto = SysUserSaveDto {
            user_name: "u1".to_string(),
            nick_name: "N".to_string(),
            password: "pw".to_string(),
            email: None,
            remark: None,
        };
        let u1 = SysUser::new_user_from_save_dto(&dto);
        let u2 = SysUser::new_user_from_save_dto(&dto);
        assert_ne!(u1.id, u2.id);
    }
}
