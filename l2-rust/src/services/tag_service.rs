use crate::models::tag::{Tag, TagError};
use chrono::Utc;
use sqlx::{Error as SqlxError, PgPool, Row};
use std::sync::Arc;

// 标签服务结构体
#[derive(Clone)]
pub struct TagService {
    pub db_pool: PgPool,
}

impl TagService {
    // 创建新的标签服务
    pub fn new(db_pool: PgPool) -> Arc<Self> {
        Arc::new(Self { db_pool })
    }

    // 获取标签列表
    pub async fn list(&self) -> Result<Vec<Tag>, TagError> {
        let tags = sqlx::query(
            "SELECT id, name, remark, status, create_id, create_time, update_id, update_time \
             FROM b_tag \
             WHERE status = TRUE \
             ORDER BY create_time DESC"
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| TagError::DatabaseError(e.to_string()))?
        .into_iter()
        .map(|row| Tag {
            id: row.get("id"),
            name: row.get("name"),
            remark: row.try_get("remark").ok(),
            status: row.get("status"),
            create_id: row.get("create_id"),
            create_time: row.get("create_time"),
            update_id: row.try_get("update_id").ok(),
            update_time: row.try_get("update_time").ok(),
        })
        .collect();

        Ok(tags)
    }

    // 根据ID获取标签
    pub async fn get_by_id(&self, id: i64) -> Result<Tag, TagError> {
        let row = sqlx::query(
            "SELECT id, name, remark, status, create_id, create_time, update_id, update_time \
             FROM b_tag \
             WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => TagError::TagNotFound,
            _ => TagError::DatabaseError(e.to_string()),
        })?;

        Ok(Tag {
            id: row.get("id"),
            name: row.get("name"),
            remark: row.try_get("remark").ok(),
            status: row.get("status"),
            create_id: row.get("create_id"),
            create_time: row.get("create_time"),
            update_id: row.try_get("update_id").ok(),
            update_time: row.try_get("update_time").ok(),
        })
    }

    // 保存标签
    pub async fn save(&self, name: String, remark: Option<String>, create_id: i64) -> Result<i64, TagError> {
        // 检查标签名是否已存在
        if let Ok(_) = sqlx::query("SELECT id FROM b_tag WHERE name = $1")
            .bind(&name)
            .fetch_one(&self.db_pool)
            .await
        {
            return Err(TagError::TagNameExists);
        }

        // 生成ID
        let id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let now = Utc::now();
        sqlx::query(
            "INSERT INTO b_tag (id, name, remark, status, create_id, create_time) \
             VALUES ($1, $2, $3, TRUE, $4, $5)"
        )
        .bind(id)
        .bind(&name)
        .bind(remark)
        .bind(create_id)
        .bind(&now)
        .execute(&self.db_pool)
        .await
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;

        Ok(id)
    }

    // 更新标签
    pub async fn update(&self, id: i64, name: String, remark: Option<String>, update_id: i64) -> Result<(), TagError> {
        let now = Utc::now();
        let result = sqlx::query(
            "UPDATE b_tag SET name = $1, remark = $2, update_id = $3, update_time = $4 WHERE id = $5"
        )
        .bind(&name)
        .bind(remark)
        .bind(update_id)
        .bind(&now)
        .bind(id)
        .execute(&self.db_pool)
        .await
        .map_err(|e| TagError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            Err(TagError::TagNotFound)
        } else {
            Ok(())
        }
    }

    // 删除标签
    pub async fn remove(&self, id: i64) -> Result<(), TagError> {
        let result = sqlx::query("DELETE FROM b_tag WHERE id = $1")
            .bind(id)
            .execute(&self.db_pool)
            .await
            .map_err(|e| TagError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            Err(TagError::TagNotFound)
        } else {
            Ok(())
        }
    }
}
