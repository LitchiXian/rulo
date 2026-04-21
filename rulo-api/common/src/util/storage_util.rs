use s3::creds::Credentials;
use s3::{Bucket, BucketConfiguration, Region};

use crate::config::StorageConfig;

/// 根据配置创建 S3 Bucket 客户端（兼容 MinIO / RustFS）
pub fn create_s3_bucket(cfg: &StorageConfig) -> Result<Box<Bucket>, String> {
    let region = build_s3_region(cfg);
    let credentials = build_s3_credentials(cfg)?;

    let bucket = Bucket::new(&cfg.bucket, region, credentials)
        .map_err(|e| format!("S3 Bucket 创建失败: {e}"))?
        .with_path_style();
    Ok(bucket)
}

pub fn build_s3_region(cfg: &StorageConfig) -> Region {
    Region::Custom {
        region: cfg.region.clone(),
        endpoint: cfg.endpoint.clone(),
    }
}

pub fn build_s3_credentials(cfg: &StorageConfig) -> Result<Credentials, String> {
    Credentials::new(
        Some(&cfg.access_key),
        Some(&cfg.secret_key),
        None,
        None,
        None,
    )
    .map_err(|e| format!("S3 凭证创建失败: {e}"))
}

/// 检查 Bucket 是否存在，不存在则创建，返回 Ok(true) 表示新建，Ok(false) 表示已存在
pub async fn ensure_bucket_exists(cfg: &StorageConfig) -> Result<bool, String> {
    let bucket = create_s3_bucket(cfg)?;

    // 1) 先用 HEAD 检测 Bucket 是否已存在（list 也行，但 head_object 需要 key，这里用 list 限制 0）
    match bucket.list("".to_string(), Some("/".to_string())).await {
        Ok(_) => return Ok(false), // 能 list 说明 Bucket 已存在
        Err(_) => {}               // 继续创建
    }

    // 2) 调用 S3 CreateBucket
    let response = Bucket::create_with_path_style(
        &cfg.bucket,
        build_s3_region(cfg),
        build_s3_credentials(cfg)?,
        BucketConfiguration::default(),
    )
    .await
    .map_err(|e| format!("CreateBucket 请求失败: {e}"))?;

    tracing::debug!(
        "CreateBucket 响应: code={}, body={}",
        response.response_code,
        response.response_text
    );

    match response.response_code {
        200 => {}
        409 => return Ok(false), // 已存在（并发场景）
        code => {
            return Err(format!(
                "CreateBucket 返回异常状态码: {code}, body: {}",
                response.response_text
            ))
        }
    }

    // 3) 创建后再验证一次，确保真的存在
    match bucket.list("".to_string(), Some("/".to_string())).await {
        Ok(_) => Ok(true),
        Err(e) => Err(format!(
            "CreateBucket 返回 200，但验证 Bucket 仍不存在: {e}"
        )),
    }
}

pub fn build_object_url(cfg: &StorageConfig, key: &str) -> String {
    let endpoint = cfg.endpoint.trim_end_matches('/');
    let bucket = cfg.bucket.trim_matches('/');
    let normalized_key = key.trim_start_matches('/');
    format!("{endpoint}/{bucket}/{normalized_key}")
}

pub fn extract_object_key(cfg: &StorageConfig, value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let without_query = trimmed.split('?').next().unwrap_or(trimmed);
    let endpoint_prefix = format!(
        "{}/{}/",
        cfg.endpoint.trim_end_matches('/'),
        cfg.bucket.trim_matches('/')
    );
    if let Some(key) = without_query.strip_prefix(&endpoint_prefix) {
        return key.trim_start_matches('/').to_string();
    }

    without_query.trim_start_matches('/').to_string()
}

pub async fn resolve_object_url(
    bucket: &Bucket,
    cfg: &StorageConfig,
    value: Option<&str>,
) -> Result<Option<String>, String> {
    let Some(raw_value) = value else {
        return Ok(None);
    };

    let key = extract_object_key(cfg, raw_value);
    if key.is_empty() {
        return Ok(None);
    }

    let presigned_url = bucket
        .presign_get(&key, 3600, None)
        .await
        .map_err(|e| e.to_string());

    match presigned_url {
        Ok(url) => {
            // 将内部 endpoint 替换为对外公网 URL，使客户端通过 nginx 反向代理访问
            // nginx 需配合 proxy_set_header Host <internal-endpoint-host> 保证预签名校验通过
            if let Some(pub_url) = &cfg.pub_url {
                let internal_prefix = format!(
                    "{}/{}/",
                    cfg.endpoint.trim_end_matches('/'),
                    cfg.bucket.trim_matches('/')
                );
                let public_prefix = format!(
                    "{}/{}/",
                    pub_url.trim_end_matches('/'),
                    cfg.bucket.trim_matches('/')
                );
                if let Some(rest) = url.strip_prefix(&internal_prefix) {
                    return Ok(Some(format!("{}{}", public_prefix, rest)));
                }
            }
            Ok(Some(url))
        }
        Err(_) => Ok(Some(build_object_url(cfg, &key))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_storage_config() -> StorageConfig {
        StorageConfig {
            endpoint: "http://localhost:9000".to_string(),
            bucket: "rulo".to_string(),
            access_key: "test".to_string(),
            secret_key: "test".to_string(),
            region: "us-east-1".to_string(),
            max_file_size: 10_485_760,
            allowed_types: vec![],
            pub_url: None,
        }
    }

    #[test]
    fn build_object_url_basic() {
        let cfg = test_storage_config();
        let url = build_object_url(&cfg, "avatar/test.png");
        assert_eq!(url, "http://localhost:9000/rulo/avatar/test.png");
    }

    #[test]
    fn build_object_url_strips_slashes() {
        let mut cfg = test_storage_config();
        cfg.endpoint = "http://localhost:9000/".to_string();
        let url = build_object_url(&cfg, "/avatar/test.png");
        assert_eq!(url, "http://localhost:9000/rulo/avatar/test.png");
    }

    #[test]
    fn extract_object_key_full_url() {
        let cfg = test_storage_config();
        let key = extract_object_key(&cfg, "http://localhost:9000/rulo/avatar/test.png");
        assert_eq!(key, "avatar/test.png");
    }

    #[test]
    fn extract_object_key_with_query() {
        let cfg = test_storage_config();
        let key = extract_object_key(&cfg, "http://localhost:9000/rulo/avatar/test.png?token=abc");
        assert_eq!(key, "avatar/test.png");
    }

    #[test]
    fn extract_object_key_relative_path() {
        let cfg = test_storage_config();
        let key = extract_object_key(&cfg, "avatar/test.png");
        assert_eq!(key, "avatar/test.png");
    }

    #[test]
    fn extract_object_key_empty() {
        let cfg = test_storage_config();
        assert_eq!(extract_object_key(&cfg, ""), "");
        assert_eq!(extract_object_key(&cfg, "   "), "");
    }
}
