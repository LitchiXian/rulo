use config::Config;
use serde::Deserialize;

/// 服务器配置
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub ipaddr: String,
    pub port: u16,
}

/// JWT 配置
#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expire_hours: u64,
}

/// 数据库配置
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout_secs: u64,
}

/// Redis 配置
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

/// AI 接口配置
#[derive(Debug, Deserialize, Clone)]
pub struct AiConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

/// 对象存储配置（S3 / MinIO / RustFS）
#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    /// 单文件最大字节数（默认 10 MB）
    pub max_file_size: usize,
    /// 允许上传的 MIME 类型
    pub allowed_types: Vec<String>,
    /// 对外公网 URL 前缀（如 https://sosdan.cn/storage），设置后预签名 URL 的内部 endpoint 会被替换为此值
    /// 需与 nginx /storage/ 代理的 proxy_set_header Host <internal-endpoint-host> 配合使用
    pub pub_url: Option<String>,
}

/// 限流配置
#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    /// 登录/注册：每 IP 每分钟最大请求数
    pub login_per_minute: u64,
    /// AI 聊天：每用户每分钟最大请求数
    pub ai_per_minute: u64,
    /// 全局：每 IP 每分钟最大请求数
    pub global_per_minute: u64,
}

/// 总配置，对应 config/default.toml 的完整结构
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub ai: AiConfig,
    pub storage: StorageConfig,
    pub rate_limit: RateLimitConfig,
}

impl AppConfig {
    /// 从 `config/default.toml` 加载配置，环境变量可覆盖（双下划线分隔层级）
    ///
    /// 环境变量命名规则：`RULO__<SECTION>__<KEY>`，例如：
    /// - `RULO__DATABASE__URL` → database.url
    /// - `RULO__REDIS__URL` → redis.url
    /// - `RULO__SERVER__PORT` → server.port
    /// - `RULO__JWT__SECRET` → jwt.secret
    /// - `RULO__STORAGE__ENDPOINT` → storage.endpoint
    pub fn load() -> Self {
        Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(
                config::Environment::with_prefix("RULO")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()
            .expect("配置加载失败（TOML + 环境变量）")
            .try_deserialize::<AppConfig>()
            .expect("配置反序列化失败，请检查 config/default.toml 和环境变量是否正确")
    }
}
