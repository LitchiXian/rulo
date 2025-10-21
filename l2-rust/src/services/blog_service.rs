use crate::models::article::{Article, ArticleError};
use chrono::Utc;
use sqlx::{Error as SqlxError, MySqlPool, Row};
use std::sync::Arc;

// 博客服务结构体
#[derive(Clone)]
pub struct BlogService {
    pub db_pool: MySqlPool,
}

impl BlogService {
    // 创建新的博客服务
    pub fn new(db_pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { db_pool })
    }

    // 保存文章
    pub async fn save_article(
        &self,
        title: String,
        content: String,
        user_id: i64,
        status: i32,
    ) -> Result<Article, ArticleError> {
        let now = Utc::now();
        let result = sqlx::query("INSERT INTO article (title, content, user_id, status, create_time, update_time) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&title)
            .bind(&content)
            .bind(&user_id)
            .bind(&status)
            .bind(&now)
            .bind(&now)
            .execute(&self.db_pool)
            .await
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        // 获取插入的文章
        let id = result.last_insert_id() as i64;
        let article = sqlx::query("SELECT id, title, content, user_id, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time FROM b_blog_article WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
            .map(|row| Article {
                id:row.get("id"),
                title:row.get("title"),
                content:row.get("content"),
                user_id:row.get("user_id"),
                published_time:row.get("published_time"),
                is_published:row.get("is_published"),
                is_deleted:row.get("is_deleted"),
                create_id:row.get("create_id"),
                create_time:row.get("create_time"),
                update_id:row.get("update_id"),
                update_time:row.get("update_time"),
            })
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        Ok(article)
    }

    // 删除文章
    pub async fn delete_article(&self, article_id: &str) -> Result<(), ArticleError> {
        let id = match article_id.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Err(ArticleError::ArticleNotFound),
        };

        let result = sqlx::query("DELETE FROM b_blog_article WHERE id = ?")
            .bind(id)
            .execute(&self.db_pool)
            .await
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            Err(ArticleError::ArticleNotFound)
        } else {
            Ok(())
        }
    }

    // 更新文章
    pub async fn update_article(
        &self,
        article_id: &str,
        title: String,
        content: String,
        status: i32,
    ) -> Result<Article, ArticleError> {
        let id = match article_id.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Err(ArticleError::ArticleNotFound),
        };

        let now = Utc::now();
        let result = sqlx::query(
            "UPDATE article SET title = ?, content = ?, status = ?, update_time = ? WHERE id = ?",
        )
        .bind(&title)
        .bind(&content)
        .bind(&status)
        .bind(&now)
        .bind(id)
        .execute(&self.db_pool)
        .await
        .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ArticleError::ArticleNotFound);
        }

        // 获取更新后的文章
        let article = sqlx::query("SELECT id, title, content, user_id, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time FROM b_blog_article WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
            .map(|row| Article {
                id:row.get("id"),
                title:row.get("title"),
                content:row.get("content"),
                user_id:row.get("user_id"),
                published_time:row.get("published_time"),
                is_published:row.get("is_published"),
                is_deleted:row.get("is_deleted"),
                create_id:row.get("create_id"),
                create_time:row.get("create_time"),
                update_id:row.get("update_id"),
                update_time:row.get("update_time"),
            })
            .map_err(|e| {
                if let SqlxError::RowNotFound = e {
                    ArticleError::ArticleNotFound
                } else {
                    ArticleError::DatabaseError(e.to_string())
                }
            })?;

        Ok(article)
    }

    // 获取文章列表
    pub async fn get_article_list(&self) -> Result<Vec<Article>, ArticleError> {
        let articles = sqlx::query("SELECT id, title, content, user_id, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time FROM b_blog_article ORDER BY create_time DESC")
            .fetch_all(&self.db_pool)
            .await
            .map(|rows| {
                rows.into_iter().map(|row| Article {
                    id:row.get("id"),
                    title:row.get("title"),
                    content:row.get("content"),
                    user_id:row.get("user_id"),
                    published_time:row.get("published_time"),
                    is_published:row.get("is_published"),
                    is_deleted:row.get("is_deleted"),
                    create_id:row.get("create_id"),
                    create_time:row.get("create_time"),
                    update_id:row.get("update_id"),
                    update_time:row.get("update_time"),
                }).collect()
            })
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        Ok(articles)
    }

    // 根据ID获取文章
    pub async fn get_article_by_id(&self, article_id: &str) -> Result<Article, ArticleError> {
        let id = match article_id.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Err(ArticleError::ArticleNotFound),
        };

        let article = sqlx::query("SELECT id, title, content, user_id, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time FROM b_blog_article WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
            .map(|row| Article {
                id:row.get("id"),
                title:row.get("title"),
                content:row.get("content"),
                user_id:row.get("user_id"),
                published_time:row.get("published_time"),
                is_published:row.get("is_published"),
                is_deleted:row.get("is_deleted"),
                create_id:row.get("create_id"),
                create_time:row.get("create_time"),
                update_id:row.get("update_id"),
                update_time:row.get("update_time"),
            })
            .map_err(|e| {
                if let SqlxError::RowNotFound = e {
                    ArticleError::ArticleNotFound
                } else {
                    ArticleError::DatabaseError(e.to_string())
                }
            })?;

        Ok(article)
    }

    // 根据用户ID获取文章列表
    pub async fn get_articles_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<Article>, ArticleError> {
        let id = match user_id.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Err(ArticleError::ArticleNotFound),
        };

        let articles = sqlx::query("SELECT id, title, content, user_id, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time FROM b_blog_article WHERE user_id = ? ORDER BY create_time DESC")
            .bind(id)
            .fetch_all(&self.db_pool)
            .await
            .map(|rows| {
                rows.into_iter().map(|row| Article {
                    id:row.get("id"),
                    title:row.get("title"),
                    content:row.get("content"),
                    user_id:row.get("user_id"),
                    published_time:row.get("published_time"),
                    is_published:row.get("is_published"),
                    is_deleted:row.get("is_deleted"),
                    create_id:row.get("create_id"),
                    create_time:row.get("create_time"),
                    update_id:row.get("update_id"),
                    update_time:row.get("update_time"),
                }).collect()
            })
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        Ok(articles)
    }
}
