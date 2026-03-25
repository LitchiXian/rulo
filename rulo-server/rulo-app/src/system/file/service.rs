use rulo_common::{
    config::StorageConfig,
    error::AppError,
    result::{R, success},
    util::storage_util,
};
use s3::Bucket;
use uuid::Uuid;

/// 上传文件到 S3，返回文件 key
pub async fn upload(
    bucket: &Bucket,
    storage_config: &StorageConfig,
    file_name: &str,
    content_type: &str,
    data: &[u8],
) -> R<String> {
    // 校验文件大小
    if data.len() > storage_config.max_file_size {
        return Err(AppError::ServiceError(format!(
            "文件大小超出限制，最大允许 {} MB",
            storage_config.max_file_size / 1024 / 1024
        )));
    }

    // 校验文件类型
    if !storage_config.allowed_types.contains(&content_type.to_string()) {
        return Err(AppError::ServiceError(format!(
            "不支持的文件类型: {}，允许: {}",
            content_type,
            storage_config.allowed_types.join(", ")
        )));
    }

    // 生成唯一 key：按日期分目录 + UUID + 扩展名
    let ext = file_name
        .rsplit('.')
        .next()
        .unwrap_or("bin");
    let date_prefix = chrono::Utc::now().format("%Y/%m/%d");
    let key = format!("{}/{}.{}", date_prefix, Uuid::new_v4(), ext);

    // 上传到 S3
    bucket
        .put_object_with_content_type(&key, data, content_type)
        .await
        .map_err(|e| AppError::Internal(format!("文件上传失败: {}", e)))?;

    success(storage_util::extract_object_key(storage_config, &key))
}
