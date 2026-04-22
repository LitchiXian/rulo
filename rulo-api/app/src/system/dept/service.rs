use common::{
    error::AppError,
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, Postgres, QueryBuilder, query, query_as, query_scalar};

use crate::system::dept::model::{SysDept, SysDeptListDto, SysDeptSaveDto, SysDeptUpdateDto};

pub async fn save(pool: &PgPool, dto: &SysDeptSaveDto) -> R<SysDept> {
    // 校验 parent_id 有效性（0 表示顶级）
    if let Some(parent_id) = dto.parent_id {
        if parent_id != 0 {
            let exists: i64 = query_scalar!(
                "SELECT COUNT(*) FROM sys_dept WHERE id = $1 AND is_deleted = false",
                parent_id
            )
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
            if exists == 0 {
                return Err(AppError::ServiceError("父级部门不存在或已删除".to_string()));
            }
        }
    }

    let new_dept = SysDept::new_dept_from_save_dto(dto);
    query!(
        "INSERT INTO sys_dept(
            id, parent_id, name, sort_order, leader, phone, email,
            is_active, is_deleted, create_id, create_time, update_id, update_time, remark
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
        new_dept.id,
        new_dept.parent_id,
        &new_dept.name,
        new_dept.sort_order,
        new_dept.leader.as_deref(),
        new_dept.phone.as_deref(),
        new_dept.email.as_deref(),
        new_dept.is_active,
        new_dept.is_deleted,
        new_dept.create_id,
        new_dept.create_time,
        new_dept.update_id,
        new_dept.update_time,
        new_dept.remark.as_deref(),
    )
    .execute(pool)
    .await?;
    success(new_dept)
}

pub async fn remove(pool: &PgPool, dto: &IdsDto) -> R<()> {
    // 禁止删除存在子部门的节点
    let child_count: i64 = query_scalar!(
        "SELECT COUNT(*) FROM sys_dept WHERE parent_id = ANY($1) AND is_deleted = false",
        &dto.ids
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    if child_count > 0 {
        return Err(AppError::ServiceError(
            "存在子部门，请先删除子部门".to_string(),
        ));
    }

    let result = sqlx::query!(
        "UPDATE sys_dept SET is_deleted = true, update_time = now() WHERE id = ANY($1)",
        &dto.ids
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("部门不存在或已删除".to_string()));
    }
    success(())
}

pub async fn update(pool: &PgPool, dto: &SysDeptUpdateDto) -> R<()> {
    // 校验 parent_id：不能等于自己；不能是自己的后代（防止环）；不存在则报错
    if let Some(parent_id) = dto.parent_id {
        if parent_id == dto.id {
            return Err(AppError::ServiceError("父级部门不能为自身".to_string()));
        }
        if parent_id != 0 {
            let exists: i64 = query_scalar!(
                "SELECT COUNT(*) FROM sys_dept WHERE id = $1 AND is_deleted = false",
                parent_id
            )
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
            if exists == 0 {
                return Err(AppError::ServiceError("父级部门不存在或已删除".to_string()));
            }
            // 递归检查：new_parent 不能是 self 的后代
            let in_subtree: i64 = query_scalar!(
                r#"WITH RECURSIVE subtree AS (
                    SELECT id FROM sys_dept WHERE id = $1 AND is_deleted = false
                    UNION ALL
                    SELECT d.id FROM sys_dept d
                    INNER JOIN subtree s ON d.parent_id = s.id
                    WHERE d.is_deleted = false
                )
                SELECT COUNT(*) FROM subtree WHERE id = $2"#,
                dto.id,
                parent_id
            )
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
            if in_subtree > 0 {
                return Err(AppError::ServiceError(
                    "父级部门不能是当前部门的子节点".to_string(),
                ));
            }
        }
    }

    let result = sqlx::query!(
        "UPDATE sys_dept SET
            parent_id  = COALESCE($2, parent_id),
            name       = COALESCE($3, name),
            sort_order = COALESCE($4, sort_order),
            is_active  = COALESCE($5, is_active),
            leader     = COALESCE($6, leader),
            phone      = COALESCE($7, phone),
            email      = COALESCE($8, email),
            remark     = COALESCE($9, remark),
            update_time = now()
        WHERE id = $1 AND is_deleted = false",
        dto.id,
        dto.parent_id,
        dto.name.as_deref(),
        dto.sort_order,
        dto.is_active,
        dto.leader.as_deref(),
        dto.phone.as_deref(),
        dto.email.as_deref(),
        dto.remark.as_deref(),
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("部门不存在或已删除".to_string()));
    }
    success(())
}

pub async fn detail(pool: &PgPool, dto: &IdDto) -> R<SysDept> {
    let data = query_as!(
        SysDept,
        "SELECT * FROM sys_dept WHERE id = $1 AND is_deleted = false",
        dto.id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("部门不存在".to_string()))?;
    success(data)
}

/// 部门列表（树形场景一次性返回全部，前端构建树）
pub async fn list_all(pool: &PgPool, dto: &SysDeptListDto) -> R<Vec<SysDept>> {
    let mut data_builder = QueryBuilder::<Postgres>::new(
        "SELECT * FROM sys_dept WHERE is_deleted = false",
    );
    append_dept_filters(&mut data_builder, dto);
    data_builder.push(" ORDER BY parent_id ASC, sort_order ASC, update_time DESC");
    let list = data_builder
        .build_query_as::<SysDept>()
        .fetch_all(pool)
        .await?;
    success(list)
}

fn append_dept_filters(builder: &mut QueryBuilder<Postgres>, dto: &SysDeptListDto) {
    if let Some(name) = dto.name.as_deref().filter(|v| !v.trim().is_empty()) {
        builder.push(" AND name ILIKE ").push_bind(format!("%{}%", name.trim()));
    }
    if let Some(is_active) = dto.is_active {
        builder.push(" AND is_active = ").push_bind(is_active);
    }
    if let Some(parent_id) = dto.parent_id {
        builder.push(" AND parent_id = ").push_bind(parent_id);
    }
    if let Some(t) = dto.create_start_time {
        builder.push(" AND create_time >= ").push_bind(t);
    }
    if let Some(t) = dto.create_end_time {
        builder.push(" AND create_time <= ").push_bind(t);
    }
}
