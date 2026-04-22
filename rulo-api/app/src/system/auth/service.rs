use crate::system::{
    auth::model::{LoginInfoVo, MenuTreeNode, UserInfoVo},
    user::{
        model::{SysUser, SysUserSaveDto},
        service,
    },
};
use deadpool_redis::Pool;
use common::{
    config::{JwtConfig, StorageConfig},
    constant::redis_constant,
    error::AppError,
    result::{R, success},
    util::{jwt_util, password_util, redis_util, storage_util},
};
use s3::Bucket;
use sqlx::{PgPool, query_as, query_scalar};
use tracing::info;

use crate::system::auth::model::AuthUserDto;

pub async fn login(db_pool: &PgPool, redis_pool: &Pool, jwt_config: &JwtConfig, dto: &AuthUserDto) -> R<String> {
    info!("login attempt: username={}", &dto.username);

    // validate user existence
    let db_user = query_as!(
        SysUser,
        "SELECT * FROM sys_user
        WHERE user_name = $1 AND is_deleted = false",
        &dto.username
    )
    .fetch_optional(db_pool)
    .await?
    .ok_or_else(|| AppError::ServiceError("用户名或密码有误".to_string()))?;

    // validate that user can perform normal operations
    if db_user.is_active == false {
        return Err(AppError::ServiceError(
            "账号已冻结,请联系管理员解冻".to_string(),
        ));
    }

    // validate password against stored hash
    if !password_util::verify_password(&dto.password, &db_user.password)? {
        return Err(AppError::ServiceError("用户名或密码有误".to_string()));
    }

    info!("login success: user_id={}, user_name={}", db_user.id, &db_user.user_name);

    // generate token
    let token =
        jwt_util::generate_token(db_user.id, &jwt_config.secret, jwt_config.expire_hours)
            .map_err(|e| AppError::Internal(e.to_string()))?;

    // save token to Redis
    redis_util::set_obj(
        redis_pool,
        &(redis_constant::USER_TOKEN.to_owned() + &token),
        &db_user.id,
        redis_constant::ONE_DAY,
    )
    .await?;

    // save user info to Redis
    redis_util::set_obj(
        redis_pool,
        &(redis_constant::USER_INFO.to_owned() + &db_user.id.to_string()),
        &db_user,
        redis_constant::ONE_DAY,
    )
    .await?;

    // 预加载权限和菜单到 Redis（共用 is_super 避免重复查 DB）
    let auth = query_user_auth(db_pool, db_user.id).await?;
    let menus = query_user_menu_tree(db_pool, db_user.id, auth.is_super).await?;

    redis_util::set_obj(
        redis_pool,
        &(redis_constant::USER_AUTH.to_owned() + &db_user.id.to_string()),
        &auth,
        redis_constant::ONE_DAY,
    )
    .await?;

    redis_util::set_obj(
        redis_pool,
        &(redis_constant::USER_MENUS.to_owned() + &db_user.id.to_string()),
        &menus,
        redis_constant::ONE_DAY,
    )
    .await?;

    success(token)
}

pub async fn register(db_pool: &PgPool, dto: &AuthUserDto) -> R<()> {
    info!("register attempt: username={}", &dto.username);

    // validate user existence
    if query_as!(
        SysUser,
        "SELECT * FROM sys_user
        WHERE user_name = $1 AND is_deleted = false",
        &dto.username
    )
    .fetch_optional(db_pool)
    .await?
    .is_some()
    {
        return Err(AppError::ServiceError(
            "用户名已被占用,请换一个试试".to_string(),
        ));
    }

    // create user account（service::save 内部会 hash 密码）
    let user_save_dto = SysUserSaveDto {
        user_name: dto.username.clone(),
        nick_name: dto.username.clone(),
        password: dto.password.clone(),
        email: dto.email.clone(),
        remark: None,
    };
    service::save(db_pool, &user_save_dto).await?;

    success(())
}

pub async fn logout(redis_pool: &Pool, token: &str) -> R<()> {
    let token_key = redis_constant::USER_TOKEN.to_owned() + token;

    // 先按 token 反查 user_id（用于清理用户维度的缓存）
    // 即使 token 已失效或缓存被驱逐，也按"已登出"返回成功，符合幂等语义
    if let Some(user_id) = redis_util::get_obj::<i64>(redis_pool, &token_key).await? {
        let suffix = user_id.to_string();
        let _ = redis_util::del(redis_pool, &(redis_constant::USER_INFO.to_owned() + &suffix)).await;
        let _ = redis_util::del(redis_pool, &(redis_constant::USER_AUTH.to_owned() + &suffix)).await;
        let _ = redis_util::del(redis_pool, &(redis_constant::USER_MENUS.to_owned() + &suffix)).await;
    }

    redis_util::del(redis_pool, &token_key).await?;
    success(())
}

pub async fn info(
    db_pool: &PgPool,
    redis_pool: &Pool,
    bucket: &Bucket,
    storage_config: &StorageConfig,
    user_id: i64,
) -> R<LoginInfoVo> {
    // 1.用户信息 -- 优先 Redis
    let user_info_key = redis_constant::USER_INFO.to_owned() + &user_id.to_string();
    let db_user = match redis_util::get_obj::<SysUser>(redis_pool, &user_info_key).await? {
        Some(u) => u,
        None => {
            // Redis 无缓存, 查 DB 回填
            let u = query_as!(SysUser, "SELECT * FROM sys_user WHERE id = $1 AND is_deleted = false", user_id)
                .fetch_optional(db_pool)
                .await?
                .ok_or_else(|| AppError::Unauthorized("用户不存在".to_string()))?;
            redis_util::set_obj(redis_pool, &user_info_key, &u, redis_constant::ONE_DAY).await?;
            u
        }
    };

    // 2.权限列表 -- 优先 Redis（与 IsSuperAdmin 共用 UserAuth 缓存）
    let auth = get_user_auth_from_cache(db_pool, redis_pool, user_id).await?;
    let perms = auth.perms;

    // 3.菜单树 -- 优先 Redis
    let menus_key = redis_constant::USER_MENUS.to_owned() + &user_id.to_string();
    let menus = match redis_util::get_obj::<Vec<MenuTreeNode>>(redis_pool, &menus_key).await? {
        Some(m) => m,
        None => {
            let m = query_user_menu_tree(db_pool, user_id, auth.is_super).await?;
            redis_util::set_obj(redis_pool, &menus_key, &m, redis_constant::ONE_DAY).await?;
            m
        }
    };

    let user_vo = UserInfoVo {
        id: db_user.id,
        user_name: db_user.user_name,
        nick_name: db_user.nick_name,
        email: db_user.email,
        is_active: db_user.is_active,
        remark: db_user.remark,
        avatar_url: storage_util::resolve_object_url(
            bucket,
            storage_config,
            db_user.avatar_url.as_deref(),
        )
        .await
        .map_err(AppError::Internal)?,
    };

    success(LoginInfoVo {
        user: user_vo,
        perms,
        menus,
    })
}

/// 判断指定用户是否拥有超级管理员角色（is_super=true 且角色启用未删除）。
/// 仅供 auth 模块内部使用：中间件预解析时一次性写入缓存。
async fn is_super_admin_user(db_pool: &PgPool, user_id: i64) -> Result<bool, AppError> {
    let is_super = query_scalar!(
        "SELECT EXISTS(
            SELECT 1 FROM sys_user_role ur
            JOIN sys_role r ON r.id = ur.role_id
            WHERE ur.user_id = $1 AND r.is_super = true AND r.is_active = true AND r.is_deleted = false
        ) AS \"is_super!\"",
        user_id
    )
    .fetch_one(db_pool)
    .await?;
    Ok(is_super)
}

/// 用户鉴权上下文（超管标记 + 权限码），由中间件预解析后缓存到 redis。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserAuth {
    pub is_super: bool,
    pub perms: Vec<String>,
}

/// 一次查出超管标记 + 权限码集合 (含超管逻辑)。
/// 同时验证用户未被软删除且处于启用状态——中间件调用路径上出现这些情况则返回 401，
/// 从而实现“删除/冻结账号下次请求立即生效”。
async fn query_user_auth(db_pool: &PgPool, user_id: i64) -> Result<UserAuth, AppError> {
    let user_alive = query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM sys_user WHERE id = $1 AND is_deleted = false AND is_active = true) AS \"alive!\"",
        user_id
    )
    .fetch_one(db_pool)
    .await?;
    if !user_alive {
        return Err(AppError::Unauthorized(
            "账号不可用, 请重新登录".to_string(),
        ));
    }

    let is_super = is_super_admin_user(db_pool, user_id).await?;

    if is_super {
        // 超管拥有所有权限
        let perms: Vec<String> =
            query_scalar!("SELECT perm_code FROM sys_permission WHERE is_deleted = false")
                .fetch_all(db_pool)
                .await?;
        return Ok(UserAuth { is_super, perms });
    }

    // 普通用户: 通过角色关联查询权限
    let perms: Vec<String> = query_scalar!(
        "SELECT DISTINCT p.perm_code 
        FROM sys_permission p
        JOIN sys_role_permission rp ON rp.perm_id = p.id
        JOIN sys_user_role ur ON ur.role_id = rp.role_id
        JOIN sys_role r ON r.id = rp.role_id
        WHERE ur.user_id = $1
            AND r.is_active = true AND r.is_deleted = false
            AND p.is_deleted = false",
        user_id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(UserAuth { is_super, perms })
}

/// 从 Redis 获取用户鉴权上下文 (供中间件调用) - 未命中时从 DB 重建并缓存。
pub async fn get_user_auth_from_cache(
    db_pool: &PgPool,
    redis_pool: &Pool,
    user_id: i64,
) -> Result<UserAuth, AppError> {
    let key = redis_constant::USER_AUTH.to_owned() + &user_id.to_string();
    match redis_util::get_obj::<UserAuth>(redis_pool, &key).await? {
        Some(a) => Ok(a),
        None => {
            let a = query_user_auth(db_pool, user_id).await?;
            redis_util::set_obj(redis_pool, &key, &a, redis_constant::ONE_DAY).await?;
            Ok(a)
        }
    }
}

// 查询用户菜单并组装为树 (通过 role_permission + perm_type=2 推导)
async fn query_user_menu_tree(
    db_pool: &PgPool,
    user_id: i64,
    is_super: bool,
) -> Result<Vec<MenuTreeNode>, AppError> {
    // 查可见菜单(含目录)
    let rows = if is_super {
        // 超管看所有未删除菜单
        query_as!(
            MenuRow,
            "SELECT id, parent_id, name, menu_type, path, component, icon, sort_order, is_hidden
            FROM sys_menu WHERE is_deleted = false
            ORDER BY sort_order, id"
        )
        .fetch_all(db_pool)
        .await?
    } else {
        // 普通用户: 有权限的菜单 + 所有目录 (parent_id=0 且 menu_type=1)
        query_as!(
            MenuRow,
            "SELECT DISTINCT m.id, m.parent_id, m.name, m.menu_type, m.path, m.component, m.icon, m.sort_order, m.is_hidden
            FROM sys_menu m
            LEFT JOIN sys_permission p ON p.id = m.perm_id
            LEFT JOIN sys_role_permission rp ON rp.perm_id = p.id
            LEFT JOIN sys_user_role ur ON ur.role_id = rp.role_id
            LEFT JOIN sys_role r ON r.id = rp.role_id
            WHERE m.is_deleted = false
                AND (
                    m.perm_id IS NULL -- 目录节点无权限, 先全部带上
                    OR(ur.user_id = $1 AND r.is_active = true AND r.is_deleted = false AND p.is_deleted = false)
                )
            ORDER BY m.sort_order, m.id",
            user_id
        )
        .fetch_all(db_pool)
        .await?
    };

    Ok(build_menu_tree(&rows, 0))
}

#[derive(sqlx::FromRow)]
struct MenuRow {
    id: i64,
    parent_id: i64,
    name: String,
    menu_type: i16,
    path: Option<String>,
    component: Option<String>,
    icon: Option<String>,
    sort_order: i32,
    is_hidden: bool,
}

// 递归组装菜单树, 自动过滤没有子菜单的空目录
fn build_menu_tree(rows: &[MenuRow], parent_id: i64) -> Vec<MenuTreeNode> {
    let mut nodes: Vec<MenuTreeNode> = Vec::new();
    for row in rows {
        if row.parent_id != parent_id {
            continue;
        }
        let children = build_menu_tree(rows, row.id);
        // 目录节点 (menu_type=1) 如果没有子菜单则不显示
        if row.menu_type == 1 && children.is_empty() {
            continue;
        }
        nodes.push(MenuTreeNode {
            id: row.id,
            parent_id: row.parent_id,
            name: row.name.clone(),
            menu_type: row.menu_type,
            path: row.path.clone(),
            component: row.component.clone(),
            icon: row.icon.clone(),
            sort_order: row.sort_order,
            is_hidden: row.is_hidden,
            children,
        });
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_row(id: i64, parent_id: i64, name: &str, menu_type: i16, sort_order: i32) -> MenuRow {
        MenuRow {
            id,
            parent_id,
            name: name.to_string(),
            menu_type,
            path: Some(format!("/{name}")),
            component: Some(format!("{name}.vue")),
            icon: None,
            sort_order,
            is_hidden: false,
        }
    }

    #[test]
    fn build_menu_tree_empty_rows() {
        let result = build_menu_tree(&[], 0);
        assert!(result.is_empty());
    }

    #[test]
    fn build_menu_tree_flat_menus() {
        // menu_type=2 为叶子菜单，不会因无子节点被过滤
        let rows = vec![
            make_row(1, 0, "home", 2, 1),
            make_row(2, 0, "about", 2, 2),
        ];
        let tree = build_menu_tree(&rows, 0);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree[0].name, "home");
        assert_eq!(tree[1].name, "about");
    }

    #[test]
    fn build_menu_tree_nested() {
        // 目录(1) -> 菜单(2)
        let rows = vec![
            make_row(1, 0, "system", 1, 1),   // 目录
            make_row(2, 1, "user", 2, 1),      // 菜单
            make_row(3, 1, "role", 2, 2),      // 菜单
        ];
        let tree = build_menu_tree(&rows, 0);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].name, "system");
        assert_eq!(tree[0].children.len(), 2);
        assert_eq!(tree[0].children[0].name, "user");
        assert_eq!(tree[0].children[1].name, "role");
    }

    #[test]
    fn build_menu_tree_empty_directory_filtered() {
        // 目录没有子菜单时应被过滤
        let rows = vec![
            make_row(1, 0, "empty_dir", 1, 1),
        ];
        let tree = build_menu_tree(&rows, 0);
        assert!(tree.is_empty());
    }

    #[test]
    fn build_menu_tree_sort_order_preserved() {
        let rows = vec![
            make_row(1, 0, "c", 2, 3),
            make_row(2, 0, "a", 2, 1),
            make_row(3, 0, "b", 2, 2),
        ];
        // build_menu_tree 按数组顺序遍历，排序由 SQL ORDER BY 保证
        let tree = build_menu_tree(&rows, 0);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree[0].name, "c");
        assert_eq!(tree[1].name, "a");
        assert_eq!(tree[2].name, "b");
    }

    #[test]
    fn build_menu_tree_deeply_nested() {
        let rows = vec![
            make_row(1, 0, "l1", 1, 1),
            make_row(2, 1, "l2", 1, 1),
            make_row(3, 2, "l3", 2, 1),
        ];
        let tree = build_menu_tree(&rows, 0);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children[0].name, "l3");
    }
}
