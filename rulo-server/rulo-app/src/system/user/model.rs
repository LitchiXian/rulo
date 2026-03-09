use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct DbSysUser {
    pub id: i64,
    pub user_name: String,
    pub nick_name: String,
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

impl DbSysUser {
    pub fn new_user_from_save_dto(dto: SysUserSaveDto) -> Self {
        // todo 生成 雪花ID
        let id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // todo 密码加密
        let now_time = Utc::now();

        DbSysUser {
            id: id,
            user_name: id.to_string(),
            nick_name: dto.nick_name,
            password: dto.password,
            email: dto.email,
            is_active: true,
            is_deleted: false,
            create_id: id,
            create_time: now_time.clone(),
            update_id: id,
            update_time: now_time.clone(),
            remark: dto.remark,
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

#[derive(Clone, Serialize)]
pub struct SysUser {
    pub user_id: u64,
    pub username: String,
    pub sex: Option<u8>,
    pub email: Option<String>,
    pub remark: Option<String>,
}

impl SysUser {
    pub fn new(id: u64, dto: SysUserSaveDto) -> SysUser {
        SysUser {
            user_id: id,
            username: dto.username,
            sex: dto.sex,
            email: dto.email,
            remark: dto.remark,
        }
    }
}
