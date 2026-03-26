use rulo_common::{
    model::{PageResult, normalize_page},
    result::{R, success},
};
use sqlx::{PgPool, Postgres, QueryBuilder};

use super::model::{AuditLogListDto, SysAuditLog};

pub async fn list(pool: &PgPool, dto: &AuditLogListDto) -> R<PageResult<SysAuditLog>> {
    let (page_num, page_size) = normalize_page(dto.page_num, dto.page_size);
    let offset = ((page_num - 1) * page_size) as i64;

    let mut count_builder = QueryBuilder::<Postgres>::new(
        "SELECT COUNT(*)::bigint FROM sys_audit_log a LEFT JOIN sys_user u ON a.user_id = u.id WHERE 1=1",
    );
    append_filters(&mut count_builder, dto);
    let total = count_builder
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await? as u64;

    let mut data_builder = QueryBuilder::<Postgres>::new(
        "SELECT a.id, a.user_id, u.user_name, a.method, a.path, a.params, a.status, a.duration_ms, a.ip, a.is_sensitive, a.created_time FROM sys_audit_log a LEFT JOIN sys_user u ON a.user_id = u.id WHERE 1=1",
    );
    append_filters(&mut data_builder, dto);
    data_builder
        .push(" ORDER BY a.created_time DESC")
        .push(" LIMIT ")
        .push_bind(page_size as i64)
        .push(" OFFSET ")
        .push_bind(offset);

    let list = data_builder
        .build_query_as::<SysAuditLog>()
        .fetch_all(pool)
        .await?;

    success(PageResult {
        list,
        total,
        page_num,
        page_size,
    })
}

fn append_filters(builder: &mut QueryBuilder<Postgres>, dto: &AuditLogListDto) {
    if let Some(user_name) = dto.user_name.as_deref().filter(|v| !v.trim().is_empty()) {
        builder
            .push(" AND u.user_name ILIKE ")
            .push_bind(format!("%{}%", user_name.trim()));
    }
    if let Some(method) = dto.method.as_deref().filter(|v| !v.trim().is_empty()) {
        builder
            .push(" AND a.method = ")
            .push_bind(method.trim().to_uppercase());
    }
    if let Some(path) = dto.path.as_deref().filter(|v| !v.trim().is_empty()) {
        builder
            .push(" AND a.path ILIKE ")
            .push_bind(format!("%{}%", path.trim()));
    }
    if let Some(is_sensitive) = dto.is_sensitive {
        builder
            .push(" AND a.is_sensitive = ")
            .push_bind(is_sensitive);
    }
    if let Some(start_time) = dto.start_time {
        builder
            .push(" AND a.created_time >= ")
            .push_bind(start_time);
    }
    if let Some(end_time) = dto.end_time {
        builder
            .push(" AND a.created_time <= ")
            .push_bind(end_time);
    }
}

/// 异步插入审计日志（由中间件调用，不阻塞请求）
pub async fn insert_log(
    pool: &PgPool,
    id: i64,
    user_id: Option<i64>,
    method: &str,
    path: &str,
    params: Option<String>,
    status: i16,
    duration_ms: i64,
    ip: Option<String>,
    is_sensitive: bool,
) {
    let result = sqlx::query(
        "INSERT INTO sys_audit_log (id, user_id, method, path, params, status, duration_ms, ip, is_sensitive, created_time)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP)",
    )
    .bind(id)
    .bind(user_id)
    .bind(method)
    .bind(path)
    .bind(params.as_deref())
    .bind(status)
    .bind(duration_ms)
    .bind(ip.as_deref())
    .bind(is_sensitive)
    .execute(pool)
    .await;

    if let Err(e) = result {
        tracing::error!("failed to insert audit log: {}", e);
    }
}
