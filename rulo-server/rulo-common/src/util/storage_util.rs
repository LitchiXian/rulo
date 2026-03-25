use s3::creds::Credentials;
use s3::{Bucket, BucketConfiguration, Region};

use crate::config::StorageConfig;

/// 根据配置创建 S3 Bucket 客户端（兼容 MinIO / RustFS）
pub fn create_s3_bucket(cfg: &StorageConfig) -> Box<Bucket> {
    let region = build_s3_region(cfg);
    let credentials = build_s3_credentials(cfg);

    Bucket::new(&cfg.bucket, region, credentials)
        .expect("S3 Bucket 创建失败")
        .with_path_style()
}

pub fn build_s3_region(cfg: &StorageConfig) -> Region {
    Region::Custom {
        region: cfg.region.clone(),
        endpoint: cfg.endpoint.clone(),
    }
}

pub fn build_s3_credentials(cfg: &StorageConfig) -> Credentials {
    Credentials::new(
        Some(&cfg.access_key),
        Some(&cfg.secret_key),
        None,
        None,
        None,
    )
    .expect("S3 凭证创建失败")
}

pub async fn ensure_bucket_exists(cfg: &StorageConfig) -> Result<bool, String> {
    let response = Bucket::create_with_path_style(
        &cfg.bucket,
        build_s3_region(cfg),
        build_s3_credentials(cfg),
        BucketConfiguration::private(),
    )
    .await
    .map_err(|e| e.to_string())?;

    match response.response_code {
        200 => Ok(true),
        409 => Ok(false),
        code => Err(format!("创建 Bucket 返回异常状态码: {code}")),
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
        .presign_get(format!("/{}", key), 3600, None)
        .await
        .map_err(|e| e.to_string());

    match presigned_url {
        Ok(url) => Ok(Some(url)),
        Err(_) => Ok(Some(build_object_url(cfg, &key))),
    }
}
