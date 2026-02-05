use crate::models::user::{User, UserError, UserInfo};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Error as SqlxError, MySqlPool, Row};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// 认证服务结构体
#[derive(Clone)]
pub struct AuthService {
    pub db_pool: MySqlPool,
    // 简单的token存储（生产环境应使用Redis）
    pub token_store: Arc<RwLock<HashMap<String, u64>>>,
}

impl AuthService {
    // 创建新的认证服务
    pub fn new(db_pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self {
            db_pool,
            token_store: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // 用户登录
    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<(User, String), UserError> {
        // 查询用户 - 修正字段名为 user_name
        let user = match sqlx::query("SELECT id, user_name, nick_name, email, password, is_active, is_deleted, remark, create_id, create_time, update_id, update_time FROM sys_user WHERE user_name = ? AND is_deleted = 0")
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
                    remark: row.try_get("remark").unwrap_or_default(),
                    create_id: row.get("create_id"),
                    create_time: row.get("create_time"),
                    update_id: row.get("update_id"),
                    update_time: row.try_get("update_time").ok(),
                }
            },
            Err(SqlxError::RowNotFound) => return Err(UserError::UserNotFound),
            Err(e) => return Err(UserError::DatabaseError(e.to_string())),
        };

        // 验证密码
        match verify(&password, &user.password) {
            Ok(valid) => {
                if !valid {
                    return Err(UserError::PasswordIncorrect);
                }
            }
            Err(_) => return Err(UserError::PasswordIncorrect),
        }

        // 生成token
        let token = self.generate_token().await;

        // 存储token与用户ID的映射
        if let Ok(mut store) = self.token_store.write() {
            store.insert(token.clone(), user.id);
        }

        Ok((user, token))
    }

    // 根据token获取用户信息
    pub async fn get_user_by_token(&self, token: &str) -> Result<UserInfo, UserError> {
        // 从token存储中获取用户ID
        let user_id = {
            let store = self.token_store.read().map_err(|_| UserError::Other("Token store error".to_string()))?;
            store.get(token).copied()
        };

        let user_id = user_id.ok_or(UserError::UserNotFound)?;

        // 查询用户信息
        let row = sqlx::query("SELECT id, user_name, nick_name, email, remark, create_time, update_time FROM sys_user WHERE id = ? AND is_deleted = 0")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => UserError::UserNotFound,
                _ => UserError::DatabaseError(e.to_string()),
            })?;

        Ok(UserInfo {
            id: row.get::<u64, _>("id").to_string(),
            user_name: row.get("user_name"),
            nick_name: row.get("nick_name"),
            avatar: String::new(), // 表中没有avatar字段
            email: row.try_get("email").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
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
        if let Ok(_) = sqlx::query("SELECT id FROM sys_user WHERE user_name = ? AND is_deleted = 0")
            .bind(&user_name)
            .fetch_one(&self.db_pool)
            .await
        {
            return Err(UserError::UsernameExists);
        }

        // 检查邮箱是否存在
        if let Ok(_) = sqlx::query("SELECT id FROM sys_user WHERE email = ? AND is_deleted = 0")
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
        let password_hash = match hash(&password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(e) => return Err(UserError::Other(e.to_string())),
        };

        // 生成雪花ID（简化处理，实际应用雪花算法）
        let id: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // 创建用户
        let now = Utc::now();
        match sqlx::query("INSERT INTO sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time) VALUES (?, ?, ?, ?, ?, 0, 0, ?, ?, ?, ?)")
            .bind(id)
            .bind(&user_name)
            .bind(&user_name) // nick_name默认为user_name
            .bind(&password_hash)
            .bind(&email)
            .bind(id) // create_id
            .bind(&now)
            .bind(id) // update_id
            .bind(&now)
            .execute(&self.db_pool)
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(UserError::DatabaseError(e.to_string())),
        }
    }

    // 发送验证码
    pub async fn send_verification_code(&self, _email: &str) -> Result<(), UserError> {
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
