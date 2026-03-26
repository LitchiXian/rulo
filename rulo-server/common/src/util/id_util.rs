use snowflake::SnowflakeIdGenerator;
use std::sync::{LazyLock, Mutex};

static ID_GENERATOR: LazyLock<Mutex<SnowflakeIdGenerator>> =
    LazyLock::new(|| {
        // machine_id=1, node_id=1，单实例部署足够；多实例时可从配置读取
        Mutex::new(SnowflakeIdGenerator::new(1, 1))
    });

/// 生成全局唯一的雪花 ID
pub fn next_id() -> i64 {
    ID_GENERATOR
        .lock()
        .expect("snowflake id generator lock poisoned")
        .real_time_generate()
}
