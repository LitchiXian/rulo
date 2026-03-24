use s3::creds::Credentials;
use s3::{Bucket, Region};

use crate::config::StorageConfig;

/// 根据配置创建 S3 Bucket 客户端（兼容 MinIO / RustFS）
pub fn create_s3_bucket(cfg: &StorageConfig) -> Box<Bucket> {
    let region = Region::Custom {
        region: cfg.region.clone(),
        endpoint: cfg.endpoint.clone(),
    };

    let credentials = Credentials::new(
        Some(&cfg.access_key),
        Some(&cfg.secret_key),
        None,
        None,
        None,
    )
    .expect("S3 凭证创建失败");

    Bucket::new(&cfg.bucket, region, credentials)
        .expect("S3 Bucket 创建失败")
        .with_path_style()
}
