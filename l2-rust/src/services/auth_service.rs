use crate::models::user::{User, UserError};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Error as SqlxError, MySqlPool, Row};
use std::sync::Arc;

// 认证服务结构体
#[derive(Clone)]
pub struct AuthService {
    pub db_pool: MySqlPool,
    // 这里可以添加其他依赖，比如Redis连接等
}

impl AuthService {
    // 创建新的认证服务
    pub fn new(db_pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { db_pool })
    }

    // 用户登录
    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<(User, String), UserError> {
        // 查询用户
        let user = match sqlx::query("SELECT id, user_name, nick_name, email, password, is_active, is_deleted, remark, create_id, create_time, update_id, update_time FROM sys_user WHERE username = ?")
            .bind(&username)
            .fetch_one(&self.db_pool)
            .await {
            Ok(row) => {
                User {
                    id: row.get("id"),
                    user_name: row.get("user_name"),
                    nick_name: row.get("nick_name"),
                    email: row.get("email"),
                    password: row.get("password"),
                    is_active: row.get("is_active"),
                    is_deleted: row.get("is_deleted"),
                    remark: row.get("remark"),
                    create_id: row.get("create_id"),
                    create_time: row.get("create_time"),
                    update_id: row.get("update_id"),
                    update_time: row.get("update_time"),
                }
            },
            Err(SqlxError::RowNotFound) => return Err(UserError::UserNotFound),
            Err(e) => return Err(UserError::DatabaseError(e.to_string())),
        };

        // 验证密码
        if let Err(_) = verify(&password, &user.password) {
            return Err(UserError::PasswordIncorrect);
        }

        // 生成token（简化处理，实际应该有更复杂的token生成逻辑）
        let token = self.generate_token().await;

        Ok((user, token))
    }

    // 用户注册
    pub async fn register(
        &self,
        user_name: String,
        password: String,
        email: String,
        verification_code: String,
    ) -> Result<(), UserError> {
        // 检查用户名是否存在
        if let Ok(_) = sqlx::query("SELECT id FROM sys_user WHERE user_name = ?")
            .bind(&user_name)
            .fetch_one(&self.db_pool)
            .await
        {
            return Err(UserError::UsernameExists);
        }

        // 检查邮箱是否存在
        if let Ok(_) = sqlx::query("SELECT id FROM sys_user WHERE email = ?")
            .bind(&email)
            .fetch_one(&self.db_pool)
            .await
        {
            return Err(UserError::EmailExists);
        }

        // 这里应该有验证码验证逻辑
        // 简化处理，假设验证码正确
        if verification_code.is_empty() {
            return Err(UserError::VerificationCodeError);
        }

        // 密码加密
        let password = match hash(&password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(e) => return Err(UserError::Other(e.to_string())),
        };

        // 创建用户
        let now = Utc::now();
        match sqlx::query("INSERT INTO user (user_name, email, password, is_active, create_time, update_time) VALUES (?, ?, ?, 1, ?, ?)")
            .bind(&user_name)
            .bind(&email)
            .bind(&password)
            .bind(&now)
            .bind(&now)
            .execute(&self.db_pool)
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(UserError::DatabaseError(e.to_string())),
        }
    }

    // 发送验证码
    pub async fn send_verification_code(&self, email: &str) -> Result<(), UserError> {
        // 这里应该有发送邮件的逻辑
        // 简化处理，假设验证码发送成功
        Ok(())
    }

    // 生成token
    async fn generate_token(&self) -> String {
        let mut rng = rand::thread_rng();
        let token: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();
        token
    }
}
