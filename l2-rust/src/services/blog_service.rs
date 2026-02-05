use crate::models::article::{Article, ArticleError, ArticleResponse};
use chrono::Utc;
use sqlx::{Error as SqlxError, PgPool, Row};
use std::sync::Arc;

// 博客服务结构体
#[derive(Clone)]
pub struct BlogService {
    pub db_pool: PgPool,
}

impl BlogService {
    // 创建新的博客服务
    pub fn new(db_pool: PgPool) -> Arc<Self> {
        Arc::new(Self { db_pool })
    }

    // 获取文章的标签（返回逗号分隔的标签ID）
    async fn get_article_tags(&self, article_id: i64) -> Result<String, ArticleError> {
        let tag_ids = sqlx::query(
            "SELECT tag_id FROM b_article_tag WHERE article_id = $1 ORDER BY tag_id"
        )
            .bind(article_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(|row| row.get::<i64, _>("tag_id").to_string())
            .collect::<Vec<String>>()
            .join(",");
        
        Ok(tag_ids)
    }

    // 保存文章
    pub async fn save_article(
        &self,
        title: String,
        content: String,
        user_id: i64,
        is_published: i32,
    ) -> Result<Article, ArticleError> {
        // 生成雪花ID
        let id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let now = Utc::now();
        let published_time = if is_published == 1 { Some(now) } else { None };
        let is_pub = is_published == 1;
        
        // 查询用户名
        let user_name: Option<String> = sqlx::query("SELECT nick_name FROM sys_user WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?
            .map(|row| row.get("nick_name"));
        
        sqlx::query(
            "INSERT INTO b_blog_article (id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, FALSE, $8, $9, $10, $11)"
        )
            .bind(id)
            .bind(&title)
            .bind(&content)
            .bind(user_id)
            .bind(&user_name)
            .bind(published_time)
            .bind(is_pub)
            .bind(user_id)
            .bind(&now)
            .bind(user_id)
            .bind(&now)
            .execute(&self.db_pool)
            .await
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        // 获取插入的文章
        let article = sqlx::query(
            "SELECT id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time 
             FROM b_blog_article WHERE id = $1"
        )
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
            .map(|row| Article {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                user_id: row.get("user_id"),
                user_name: row.get("user_name"),
                published_time: row.get("published_time"),
                is_published: row.get("is_published"),
                is_deleted: row.get("is_deleted"),
                create_id: row.get("create_id"),
                create_time: row.get("create_time"),
                update_id: row.get("update_id"),
                update_time: row.get("update_time"),
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

        let result = sqlx::query("UPDATE b_blog_article SET is_deleted = TRUE WHERE id = $1")
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
        let is_pub = status == 1;
        let result = sqlx::query(
            "UPDATE b_blog_article SET title = $1, content = $2, is_published = $3, update_time = $4 WHERE id = $5",
        )
        .bind(&title)
        .bind(&content)
        .bind(is_pub)
        .bind(&now)
        .bind(id)
        .execute(&self.db_pool)
        .await
        .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ArticleError::ArticleNotFound);
        }

        // 获取更新后的文章
        let article = sqlx::query(
            "SELECT id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time 
             FROM b_blog_article WHERE id = $1"
        )
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
            .map(|row| Article {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                user_id: row.get("user_id"),
                user_name: row.get("user_name"),
                published_time: row.get("published_time"),
                is_published: row.get("is_published"),
                is_deleted: row.get("is_deleted"),
                create_id: row.get("create_id"),
                create_time: row.get("create_time"),
                update_id: row.get("update_id"),
                update_time: row.get("update_time"),
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

    // 获取文章列表（返回前端格式）
    pub async fn get_article_list(&self) -> Result<Vec<ArticleResponse>, ArticleError> {
        let articles = sqlx::query(
            "SELECT id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time 
             FROM b_blog_article 
             WHERE is_deleted = FALSE 
             ORDER BY create_time DESC"
        )
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        let mut responses = Vec::new();
        for row in articles {
            let article = Article {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                user_id: row.get("user_id"),
                user_name: row.get("user_name"),
                published_time: row.get("published_time"),
                is_published: row.get("is_published"),
                is_deleted: row.get("is_deleted"),
                create_id: row.get("create_id"),
                create_time: row.get("create_time"),
                update_id: row.get("update_id"),
                update_time: row.get("update_time"),
            };
            
            let tags = self.get_article_tags(article.id).await?;
            let mut response: ArticleResponse = article.into();
            response.tags = tags;
            responses.push(response);
        }

        Ok(responses)
    }

    // 根据ID获取文章（返回前端格式）
    pub async fn get_article_by_id(&self, article_id: &str) -> Result<ArticleResponse, ArticleError> {
        let id = match article_id.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Err(ArticleError::ArticleNotFound),
        };

        let article = sqlx::query(
            "SELECT id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time 
             FROM b_blog_article 
             WHERE id = $1 AND is_deleted = FALSE"
        )
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
            .map(|row| Article {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                user_id: row.get("user_id"),
                user_name: row.get("user_name"),
                published_time: row.get("published_time"),
                is_published: row.get("is_published"),
                is_deleted: row.get("is_deleted"),
                create_id: row.get("create_id"),
                create_time: row.get("create_time"),
                update_id: row.get("update_id"),
                update_time: row.get("update_time"),
            })
            .map_err(|e| {
                if let SqlxError::RowNotFound = e {
                    ArticleError::ArticleNotFound
                } else {
                    ArticleError::DatabaseError(e.to_string())
                }
            })?;

        let tags = self.get_article_tags(article.id).await?;
        let mut response: ArticleResponse = article.into();
        response.tags = tags;

        Ok(response)
    }

    // 根据用户ID获取文章列表（返回前端格式）
    pub async fn get_articles_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<ArticleResponse>, ArticleError> {
        let id = match user_id.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Err(ArticleError::ArticleNotFound),
        };

        let articles = sqlx::query(
            "SELECT id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time 
             FROM b_blog_article 
             WHERE user_id = $1 AND is_deleted = FALSE 
             ORDER BY create_time DESC"
        )
            .bind(id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| ArticleError::DatabaseError(e.to_string()))?;

        let mut responses = Vec::new();
        for row in articles {
            let article = Article {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                user_id: row.get("user_id"),
                user_name: row.get("user_name"),
                published_time: row.get("published_time"),
                is_published: row.get("is_published"),
                is_deleted: row.get("is_deleted"),
                create_id: row.get("create_id"),
                create_time: row.get("create_time"),
                update_id: row.get("update_id"),
                update_time: row.get("update_time"),
            };
            
            let tags = self.get_article_tags(article.id).await?;
            let mut response: ArticleResponse = article.into();
            response.tags = tags;
            responses.push(response);
        }

        Ok(responses)
    }
}
