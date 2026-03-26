# Rulo 项目开发注意事项

---

## 一、命名规范

### 1.1 permission vs perm

- **数据库表名、字段名、Rust 结构体/类型名** 用全称：`permission`
  - 表名：`sys_role_permission`
  - 结构体：`SysPermission`、`SysPermissionSaveDto`
- **其他所有地方** 用缩写：`perm`
  - 权限码：`sys:user:save`
  - 函数名：`bind_perms_handler`、`get_user_perms_from_cache`
  - Redis key：`system:auth:perms:`
  - URL 路径：`/role/bind-perms`

### 1.2 URL 路径用 `-`（kebab-case）

URL 路径分词符统一用 `-`，不用 `_`：

```
✅ /system/role/bind-menus
✅ /system/monitor/server-info
❌ /system/role/bind_menus
❌ /system/monitor/server_info
```

`_` 只用在以下场景：

| 场景 | 示例 |
|------|------|
| Rust 变量/函数名（snake_case） | `bind_menus_handler` |
| 数据库字段名 | `role_id`、`created_time` |
| JSON key | `{ "menu_ids": [1, 2, 3] }` |

### 1.3 字段命名后缀约定

| 类型 | 后缀 | 示例 |
|------|------|------|
| ID 字段 | `_id` | `role_id`、`user_id`、`menu_id` |
| 时间字段 | `_time` | `created_time`、`updated_time`、`deleted_time` |
| 金额字段 | `_amt` | `amount_amt`、`price_amt`、`total_amt` |

### 1.4 布尔字段命名前缀约定

布尔字段统一用 `is_`、`has_`、`can_` 前缀，表示某种状态或能力：

```sql
is_active boolean NOT NULL DEFAULT true,
is_deleted boolean NOT NULL DEFAULT false,
can_login boolean
```

### 1.5 Rust 模型后缀约定

| 后缀 | 用途 | 派生宏 |
|------|------|--------|
| 无（如 `SysUser`） | 数据库实体 | `FromRow`, `Serialize`, `Deserialize`, `ToSchema` |
| `SaveDto` | 创建请求体 | `Deserialize`, `ToSchema` |
| `UpdateDto` | 更新请求体 | `Deserialize`, `ToSchema` |
| `ListDto` | 列表查询参数 | `Deserialize`, `IntoParams` |
| `Vo` | 响应视图对象 | `Serialize`, `ToSchema` |

### 1.6 函数命名与排列约定

#### 排列顺序

handler、service、router 中的函数统一按 **增 → 删 → 改 → 查** 排列，同类操作中越新的放越下面：

```rust
// router.rs
Router::new()
    .route("/save",              post(handler::save_handler))
    .route("/remove",            post(handler::remove_handler))
    .route("/update",            post(handler::update_handler))
    .route("/update-bind-roles", post(handler::update_bind_roles_handler))  // 改 - 绑定
    .route("/detail",            get(handler::detail_handler))
    .route("/list",              get(handler::list_handler))
    .route("/list-bind-roles",   get(handler::list_bind_roles_handler))     // 查 - 绑定
```

#### handler 层命名

handler 函数统一加 `_handler` 后缀，标识其为 HTTP 接口入口：

```rust
pub async fn save_handler(...)              // 基础增
pub async fn remove_handler(...)            // 基础删
pub async fn update_handler(...)            // 基础改
pub async fn update_bind_roles_handler(...) // 绑定改
pub async fn detail_handler(...)            // 单个查
pub async fn list_handler(...)              // 列表查
pub async fn list_bind_roles_handler(...)   // 绑定查
```

#### service 层命名

service 函数 **不加后缀**，纯业务方法名：

```rust
pub async fn save(...)              // 基础增
pub async fn remove(...)            // 基础删
pub async fn update(...)            // 基础改
pub async fn update_bind_roles(...) // 绑定改
pub async fn detail(...)            // 单个查
pub async fn list(...)              // 列表查
pub async fn list_bind_roles(...)   // 绑定查
```

#### list 命名区分

基础列表查询只叫 `list`，其他 list 必须加后缀以区分语义：

| 函数名 | 含义 |
|--------|------|
| `list` | 该实体自身的列表查询 |
| `list_bind_roles` | 查询绑定的角色 ID 列表 |
| `list_bind_menus` | 查询绑定的菜单 ID 列表 |
| `list_bind_perms` | 查询绑定的权限 ID 列表 |
| `list_by_xxx` | 按某条件筛选列表 |

#### 仅含 id 的 DTO

如果查询参数只有一个 `id` 字段，直接用 `rulo-common` 中的 `IdDto`，不要在各模块重复定义：

```rust
// ✅ 用通用 IdDto
Query(dto): Query<IdDto>

// ❌ 不要定义 RoleIdDto { role_id }、UserIdDto { user_id }
```

---

## 二、数据库字段类型规范

### 2.1 时间类型 — `timestamptz`

统一使用 `timestamptz`（带时区），不用 `timestamp`：

- 原因：`timestamp` 无时区，不同地区的服务器解释不同；不同软件显示不同
- 原因：比时间戳更直观，且方便操作（区间查询、索引优化等）

```sql
-- 推荐写法
created_time timestamptz NOT NULL DEFAULT now(),
updated_time timestamptz NOT NULL DEFAULT now(),
deleted_time timestamptz
```

Rust 映射：

```rust
use chrono::{DateTime, Utc};

pub struct User {
    pub created_time: DateTime<Utc>,
    pub deleted_time: Option<DateTime<Utc>>,
}
```

> 时间字段统一 `_time` 结尾

### 2.2 ID 类型 — `int8`

统一使用 `int8`（bigint），能够承载雪花 ID：

```sql
id int8 NOT NULL PRIMARY KEY,
user_id int8 NOT NULL,
```

Rust 映射：`i64`

> ID 字段统一 `_id` 结尾

### 2.3 金额类型 — `decimal(20,6)`

使用 `decimal(20,6)` 而不是 `float/double`，避免浮点数精度丢失：

- `decimal(20,6)`：总共 20 位数字，小数点后固定 6 位
- 适合高精度金额、加密货币等场景

```sql
price_amt      decimal(20,6) NOT NULL DEFAULT 0.000000,
total_amt      decimal(20,6) NOT NULL DEFAULT 0.000000,
balance_amt    decimal(20,6) NOT NULL DEFAULT 0.000000
```

Rust 映射：

```rust
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct Order {
    pub price_amt: Decimal,
    pub total_amt: Decimal,
}

// 使用示例
let price = dec!(19.99);
let quantity = dec!(2.5);
let total = price * quantity;  // 49.975000
```

> 金额字段统一 `_amt` 结尾，使用 `decimal(20,6)` 类型

### 2.4 布尔类型 — `boolean`

```sql
is_active   boolean NOT NULL DEFAULT true,
is_verified boolean NOT NULL DEFAULT false,
can_login   boolean
```

Rust 映射：`bool`

---

## 三、项目架构约定

### 3.1 模块目录结构

每个业务域（auth、user、role、menu、permission、monitor）统一四文件结构：

```
system/
  <domain>/
    handler.rs   — Axum handler，薄层，只做参数提取和转发
    service.rs   — 业务逻辑，DB + Redis 调用
    model.rs     — 实体结构体、DTO、VO
    router.rs    — 路由定义
```

### 3.2 返回类型约定

统一使用 `R<T>`（定义在 `rulo-common::result`）：

```rust
pub type R<T> = Result<ApiResult<T>, AppError>;
```

- 成功：`success(data)` → `code: 200`
- 业务错误：`error(code, msg)`
- 系统错误：返回 `Err(AppError::xxx)`，由框架统一处理为 JSON 响应

### 3.3 错误类型约定

使用 `AppError` 枚举，框架会自动转换为对应 HTTP 状态码和业务码：

| 变体 | 场景 | HTTP 状态 | 业务码 |
|------|------|-----------|--------|
| `ServiceError(msg)` | 业务逻辑错误 | 200 | 40000 |
| `Unauthorized(msg)` | 未登录 / token 无效 | 401 | 40100 |
| `Forbidden(msg)` | 无权限 | 403 | 40300 |
| `NotFound(msg)` | 资源不存在 | 404 | 40400 |
| `DbError(e)` | 数据库异常 | 200 | 50001 |
| `Internal(msg)` | 内部错误 | 500 | 50000 |

### 3.4 鉴权体系

#### `#[perm("...")]` 过程宏

handler 级鉴权使用 `#[perm("...")]` 过程宏（定义在 `rulo-macro`），一行搞定：

```rust
#[perm("sys:user:save")]
pub async fn save_handler(...) -> R<SysUser> { ... }
```

宏展开后会在函数入口自动插入权限检查：

```rust
// 展开后等价于：
pub async fn save_handler(
    axum::Extension(__rulo_perms): axum::Extension<PermCodes>,  // 自动注入
    // ...原参数
) -> R<SysUser> {
    if !__rulo_perms.0.iter().any(|p| p == "sys:user:save") {
        return Err(AppError::Forbidden("缺少权限: sys:user:save"));
    }
    // ...原函数体
}
```

权限码格式：`module:entity:action`，全小写冒号分隔。

#### 认证流程（JWT + Redis）

```
1. POST /system/auth/login { username, password }
   → 校验用户存在 + is_active + 密码 argon2 验证
   → 生成 JWT: jwt_util::generate_token(user_id, secret, expire_hours)
   → 写入 Redis（TTL = ONE_DAY = 86400s）：
       system:auth:user:{token}         → user_id
       system:auth:user_info:{user_id}  → SysUser
       system:auth:perms:{user_id}      → Vec<perm_codes>
       system:auth:menus:{user_id}      → Vec<MenuTreeNode>
   → 返回 { token }

2. 后续请求 Authorization: {token}
   → jwt_auth 中间件：
       a. 验证 JWT 签名
       b. Redis 查 system:auth:user:{token} → user_id
       c. 加载 PermCodes（Cache-Aside：先 Redis，miss 则查 DB 回填）
       d. 注入 UserId / UserToken / PermCodes 到 request.extensions()
   → #[perm("...")] 宏检查 PermCodes 是否包含所需权限码

3. 超级管理员（is_super = true）自动获得所有权限码
```

#### JWT 配置

JWT secret 和过期时间从 `config/default.toml` 读取，支持环境变量覆盖：

```toml
[jwt]
secret = "rulo_jwt_secret_key"
expire_hours = 24
```

```rust
// jwt_util 接收 secret 参数
pub fn generate_token(user_id: i64, secret: &str, expire_hours: u64) -> Result<String>
pub fn verify_token(token: &str, secret: &str) -> Result<Claims>
```

### 3.5 Redis Key 命名约定

格式：`module:submodule:entity:` + 动态后缀，冒号分隔：

```rust
pub const USER_TOKEN:    &str = "system:auth:user:";
pub const USER_PERMS:    &str = "system:auth:perms:";
pub const USER_MENUS:    &str = "system:auth:menus:";
pub const LOGIN_CAPTCHA: &str = "system:auth:login_captcha:";
```

使用时拼接动态 key：

```rust
let key = redis_constant::USER_TOKEN.to_owned() + &token;
let key = redis_constant::USER_PERMS.to_owned() + &user_id.to_string();
```

### 3.6 缓存读取模式（Cache-Aside）

统一的缓存读取写法：先取缓存，缓存未命中再查 DB 并写入：

```rust
match redis_util::get_obj::<T>(&redis_pool, &key).await? {
    Some(val) => val,
    None => {
        let val = query_from_db(...).await?;
        redis_util::set_obj(&redis_pool, &key, &val, ONE_DAY).await?;
        val
    }
}
```

**绑定操作后必须清除相关 Redis 缓存**，下次请求重新加载。

### 3.7 敏感字段处理

密码等敏感字段在实体结构体上必须加双重屏蔽，防止意外序列化到响应：

```rust
#[serde(skip_serializing, skip_deserializing)]
#[schema(ignore)]
pub password: Option<String>,
```

### 3.8 OpenAPI 文档（Swagger）

使用 `utoipa` + `utoipa-scalar` 生成交互式 API 文档。

**访问地址**：`http://localhost:13000/scalar`（仅 debug 构建启用）

```rust
// swagger/router.rs — 仅在 debug 模式注册文档路由
#[cfg(debug_assertions)]
let router = router.merge(Scalar::with_url("/scalar", ApiDoc::openapi()));
```

每个 handler 必须有 `#[utoipa::path(...)]` 注解，包含路径、请求体、响应和 security 声明。需要鉴权的接口加：

```rust
#[utoipa::path(
    post, path = "/system/user/save",
    request_body = SysUserSaveDto,
    responses((status = 200, description = "创建用户", body = ApiResult<SysUser>)),
    security(("bearer_auth" = []))
)]
```

所有 handler 路径和 Schema 在 `swagger/doc.rs` 的 `#[derive(OpenApi)]` 中集中注册。

---

## 四、输入校验

### 4.1 ValidatedJson 提取器

用 `ValidatedJson<T>` 替代 `Json<T>`，自动完成反序列化 + 字段校验：

```rust
// 定义在 common/src/extractor.rs
pub struct ValidatedJson<T>(pub T);
```

自动流程：
1. 反序列化 JSON body → 失败返回 `AppError::ServiceError`
2. 调用 `validator::Validate::validate()` → 失败返回 `AppError::ValidationError`（HTTP 400）

### 4.2 DTO 校验注解

DTO 上派生 `Validate`，字段加 `#[validate(...)]`：

```rust
#[derive(Deserialize, ToSchema, Validate)]
pub struct AuthUserDto {
    #[validate(length(min = 2, max = 30, message = "用户名长度必须在 2-30 之间"))]
    pub username: String,

    #[validate(length(min = 6, max = 64, message = "密码长度必须在 6-64 之间"))]
    pub password: String,
}
```

handler 中直接使用：

```rust
pub async fn login_handler(
    ValidatedJson(dto): ValidatedJson<AuthUserDto>,  // 自动校验
) -> R<String> { ... }
```

校验失败响应：

```json
HTTP 400
{ "code": 40000, "msg": "用户名长度必须在 2-30 之间", "data": null }
```

---

## 五、中间件与安全

### 5.1 中间件执行顺序

```
请求 →
  ┌─ TraceLayer（HTTP 追踪日志）
  ├─ log_app_errors（错误日志记录）
  ├─ global_rate_limit（全局限流 200/min per IP）
  ├─ 公开路由
  │   ├─ /scalar（Swagger 文档，debug only）
  │   ├─ /system/auth/login, /register
  │   │   └─ login_rate_limit（10/min per IP）
  │   │   └─ audit_log（审计日志）
  │   └─ /system/monitor/health（无鉴权）
  ├─ 受保护路由
  │   ├─ jwt_auth（JWT 验证 + PermCodes 注入）
  │   ├─ audit_log（审计日志）
  │   ├─ /system/user|role|menu|permission|file|audit/*
  │   └─ /ai/*
  │       └─ ai_rate_limit（20/min per user）
  └─ #[perm("...")] 宏级权限检查
→ 响应
```

### 5.2 限流（Rate Limiting）

基于 Redis 固定窗口计数器，60 秒窗口，三级限流：

| 层级 | 作用范围 | 维度 | 默认阈值 | 应用路由 |
|------|---------|------|---------|---------|
| Tier 1 | 登录/注册 | IP | 10/min | `/system/auth/login`, `/system/auth/register` |
| Tier 2 | AI 聊天 | user_id（未登录降级为 IP） | 20/min | `/ai/*` |
| Tier 3 | 全局兜底 | IP | 200/min | 所有路由 |

实现原理：

```rust
// Redis INCR + EXPIRE（原子性由 Redis 保证）
let count: u64 = conn.incr(key, 1u64).await?;
if count == 1 {
    conn.expire(key, 60).await?;  // 首次请求设置 60s 过期
}
if count > max_requests {
    return Err(AppError::TooManyRequests("请求过于频繁，请稍后再试"));
}
```

Redis key 格式：

```
rate_limit:login:{ip}          — 登录限流
rate_limit:ai:user:{user_id}   — AI 限流（已登录）
rate_limit:ai:ip:{ip}          — AI 限流（未登录）
rate_limit:global:{ip}         — 全局限流
```

配置（`config/default.toml`）：

```toml
[rate_limit]
login_per_minute = 10
ai_per_minute = 20
global_per_minute = 200
```

超限响应：

```json
HTTP 429
{ "code": 42900, "msg": "请求过于频繁，请稍后再试", "data": null }
```

IP 提取优先级：`x-forwarded-for` → `x-real-ip` → `ConnectInfo<SocketAddr>` → `"unknown"`

### 5.3 审计日志（Audit Log）

Tower 中间件自动记录操作日志，写入 `sys_audit_log` 表。

**记录内容**：user_id、user_name、method、path、params、status、duration_ms、ip、is_sensitive、created_time

**处理规则**：

| 规则 | 路径 | 行为 |
|------|------|------|
| 跳过记录 | `/system/audit/`、`/system/monitor/health`、`/scalar` | 不写日志（防递归） |
| 隐藏参数 | `/password`、`/login`、`/register` | params 字段为空 |
| 标记敏感 | `/remove`、`/update-bind-*`、`/password`、`/logout` | `is_sensitive = true` |
| POST/PUT | 读取 body（最大 64KB） | params = request body |
| GET | 读取 query string | params = query string |

日志写入通过 `tokio::spawn` 异步执行，不阻塞请求响应。

### 5.4 健康检查

```
GET /system/monitor/health（无需鉴权）
```

检查 DB（`SELECT 1`）、Redis（`PING`）、S3（`list bucket`）三个组件：

```json
{ "status": "healthy", "db": "ok", "cache": "ok", "storage": "ok" }
// 任一组件 down 时 status = "degraded"
```

---

## 六、配置管理

### 6.1 配置来源

`AppConfig::load()` 按优先级加载：

1. `config/default.toml` — 基础配置
2. `RULO__` 前缀环境变量 — 覆盖配置（双下划线分隔层级）

```rust
Config::builder()
    .add_source(config::File::with_name("config/default"))
    .add_source(config::Environment::with_prefix("RULO").separator("__"))
    .build()
```

### 6.2 环境变量覆盖示例

```bash
RULO__DATABASE__URL="postgres://user:pass@db:5432/rulo"
RULO__JWT__SECRET="production-secret"
RULO__REDIS__URL="redis://redis:6379/0"
RULO__SERVER__IPADDR="0.0.0.0"
RULO__SERVER__PORT="8080"
RULO__STORAGE__ENDPOINT="https://s3.amazonaws.com"
RULO__RATE_LIMIT__LOGIN_PER_MINUTE="5"
RULO__AI__API_KEY="sk-..."
```

### 6.3 配置段一览

| 段 | 说明 |
|----|------|
| `[server]` | ipaddr、port |
| `[database]` | url、max_connections、acquire_timeout_secs |
| `[redis]` | url |
| `[jwt]` | secret、expire_hours |
| `[storage]` | endpoint、bucket、access_key、secret_key、region、max_file_size、allowed_types |
| `[ai]` | api_key、base_url、model |
| `[rate_limit]` | login_per_minute、ai_per_minute、global_per_minute |

---

## 七、文件上传与对象存储

统一使用 S3 兼容存储（RustFS / MinIO），客户端为 `rust-s3`。

**上传接口**：`POST /system/file/upload`（Multipart，field name = `file`）

**处理流程**：
1. 校验文件大小（默认 10MB）
2. 校验 MIME 类型（默认允许 image/jpeg、png、gif、webp、svg+xml）
3. 生成唯一 key：`{yyyy/MM/dd}/{uuid}.{ext}`
4. 上传至 S3 bucket
5. 返回 object key（非完整 URL）

**URL 工具**（`storage_util`）：

```rust
// key → 完整 URL
build_object_url(&cfg, "avatar/test.png")
// → "http://localhost:9000/rulo/avatar/test.png"

// 完整 URL → key
extract_object_key(&cfg, "http://localhost:9000/rulo/avatar/test.png?token=abc")
// → "avatar/test.png"

// key → 1 小时预签名 URL（失败降级为直链）
resolve_object_url(&bucket, &cfg, Some("avatar/test.png")).await
```

Bootstrap 启动时自动检查并创建 bucket。

---

## 八、启动流程（Bootstrap）

`main.rs` 约 50 行，启动顺序：

```
1. init_tracing("logs", "app.log")    — 日志初始化（控制台 + 滚动日志文件）
2. AppConfig::load()                   — 加载 default.toml + RULO__* 环境变量
3. connect_db(&cfg.database)           — PostgreSQL 连接池
4. connect_redis(&cfg.redis)           — Redis 连接池 + PING 验证
5. connect_s3(&cfg.storage)            — S3 客户端 + 自动建 bucket
6. Arc::new(AppState { ... })          — 共享状态（db_pool, redis_pool, s3_bucket, configs...）
7. Router::new().merge(routes(state))  — 注册路由 + 中间件
8. TcpListener::bind + axum::serve    — 启动 HTTP 服务
```

日志输出至 `logs/app.log.{date}`（按天滚动），**必须持有 `_guard` 直到 `main` 结束**。

---

## 九、测试

### 9.1 测试分类与位置

| 类型 | 位置 | 特点 |
|------|------|------|
| 单元测试 | 各 `.rs` 文件内 `#[cfg(test)] mod tests` | 不需要数据库，纯逻辑验证 |
| 集成测试 | `rulo-app/tests/*.rs` | 需要 `DATABASE_URL`，自动建临时库 |

### 9.2 单元测试

写在对应逻辑的 `.rs` 文件底部，`#[cfg(test)]` 保证不编入生产产物：

```rust
// common/src/util/jwt_util.rs
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "test_secret";

    #[test]
    fn generate_and_verify_token() {
        let token = generate_token(12345i64, TEST_SECRET, 1).unwrap();
        let claims = verify_token(&token, TEST_SECRET).unwrap();
        assert_eq!(claims.sub, 12345i64);
    }

    #[test]
    fn verify_with_wrong_secret_fails() {
        let token = generate_token(1, TEST_SECRET, 1).unwrap();
        assert!(verify_token(&token, "wrong_secret").is_err());
    }
}
```

当前单元测试覆盖（31 个）：

| 模块 | 测试内容 | 数量 |
|------|---------|------|
| `rulo-common/model.rs` | `normalize_page` 分页边界 | 5 |
| `rulo-common/util/password_util.rs` | argon2 哈希与验证 | 4 |
| `rulo-common/util/jwt_util.rs` | JWT 生成、验证、过期、篡改 | 4 |
| `rulo-common/util/storage_util.rs` | URL 构建与解析 | 6 |
| `rulo-app/system/auth/service.rs` | `build_menu_tree` 树构建 | 6 |
| `rulo-app/system/user/model.rs` | User 构造函数 | 2 |
| `rulo-app/system/role/model.rs` | Role 构造函数 | 2 |
| `rulo-app/system/permission/model.rs` | Permission 构造函数 | 2 |

### 9.3 集成测试

放在 `rulo-app/tests/` 目录，使用 `#[sqlx::test]` 宏自动管理数据库：

```rust
// app/tests/role_service_test.rs
use rulo_app::system::role::{model::*, service};
use rulo_common::model::*;
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn test_role_crud(pool: PgPool) {
    // save
    let dto = SysRoleSaveDto { role_name: "测试角色".into(), role_key: "test".into(), ... };
    let result = service::save(&pool, &dto).await.unwrap();
    let role = result.take_data().unwrap();
    assert_eq!(role.role_name, "测试角色");

    // detail → update → list → remove ...
}
```

`#[sqlx::test(migrations = "../migrations")]` 会：
- 自动创建临时数据库（需要 `DATABASE_URL` 用户有 `CREATEDB` 权限）
- 运行全部 migration
- 每个测试独立隔离
- 测试结束后自动清理

当前集成测试（10 个）：

| 文件 | 测试内容 | 数量 |
|------|---------|------|
| `tests/role_service_test.rs` | CRUD、空 name 校验、空 key 校验 | 3 |
| `tests/permission_service_test.rs` | CRUD、重复 code 校验、无效 type 校验 | 3 |
| `tests/user_service_test.rs` | 保存删除、重复 name、无效角色绑定、清空绑定 | 4 |

### 9.4 运行测试

```bash
# 前置：确保 .env 中的 DATABASE_URL 用户有 CREATEDB 权限
# ALTER USER rulo CREATEDB;

# 运行全部测试（单元 + 集成）
cd rulo-server
cargo test

# 仅单元测试（不需数据库）
cargo test -p common                                      # 19 个
cargo test -p app -- "model::tests" "auth::service::tests" # 12 个

# 仅集成测试（需要 DATABASE_URL）
cargo test -p app --test role_service_test
cargo test -p app --test permission_service_test
cargo test -p app --test user_service_test

# 运行单个测试
cargo test -p app --test user_service_test test_user_save_and_remove

# 显示 println! 输出
cargo test -- --nocapture
```

---

## 十、错误处理

### 10.1 AppError 完整枚举

| 变体 | 场景 | HTTP 状态 | 业务码 |
|------|------|-----------|--------|
| `ServiceError(msg)` | 业务逻辑错误 | 200 | 40000 |
| `Unauthorized(msg)` | 未登录 / token 无效 | 401 | 40100 |
| `Forbidden(msg)` | 无权限 | 403 | 40300 |
| `NotFound(msg)` | 资源不存在 | 404 | 40400 |
| `TooManyRequests(msg)` | 限流触发 | 429 | 42900 |
| `ValidationError(errs)` | 输入校验失败 | 400 | 40000 |
| `DbError(e)` | 数据库异常 | 200 | 50001 |
| `RedisError(e)` | 缓存异常 | 200 | 50002 |
| `RedisPoolError(e)` | 缓存连接池异常 | 200 | 50003 |
| `PasswordHashError(e)` | 密码加密失败 | 500 | 50001 |
| `Internal(msg)` | 内部错误 | 500 | 50000 |

### 10.2 错误响应格式

所有错误统一通过 `IntoResponse` 转为 JSON：

```json
{ "code": 40100, "msg": "未登录, 请先登录", "data": null }
```

`log_app_errors` 中间件自动记录错误到 tracing 日志。
