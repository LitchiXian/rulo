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
}

impl AppConfig {
    /// 从 `config/default.toml` 加载配置，失败则 panic
    pub fn load() -> Self {
        Config::builder()
            .add_source(config::File::with_name("config/default"))
            .build()
            .expect("读取配置文件失败: config/default.toml")
            .try_deserialize::<AppConfig>()
            .expect("配置反序列化失败，请检查 config/default.toml 字段是否完整")
    }
}
