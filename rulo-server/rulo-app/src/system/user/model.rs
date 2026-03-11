use chrono::{DateTime, Utc};
use rulo_common::model::PageDto;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct SysUser {
    pub id: i64,
    pub user_name: String,
    pub nick_name: String,
    // #[serde(skip)] // 序列化和反序列化都跳过
    #[serde(skip_serializing)] // 序列化时跳过,不返回给前端
    pub password: String,
    pub email: Option<String>,
    pub is_active: bool,
    pub is_deleted: bool,
    pub create_id: i64,
    pub create_time: DateTime<Utc>,
    pub update_id: i64,
    pub update_time: DateTime<Utc>,
    pub remark: Option<String>,
}

impl SysUser {
    pub fn new_user_from_save_dto(dto: &SysUserSaveDto) -> Self {
        // todo 生成 雪花ID
        let user_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // todo 密码加密
        let now_time = Utc::now();

        SysUser {
            id: user_id,
            user_name: user_id.to_string(),
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
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SysUserSaveDto {
    pub nick_name: String,
    pub password: String,
    pub email: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SysUserUpdateDto {
    pub id: i64,
    pub nick_name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SysUserListDto {
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub create_start_time: Option<DateTime<Utc>>,
    pub create_end_time: Option<DateTime<Utc>>,
    pub remark: Option<String>,
    // #[serde(flatten)]
    // pub page: PageDto,
}
