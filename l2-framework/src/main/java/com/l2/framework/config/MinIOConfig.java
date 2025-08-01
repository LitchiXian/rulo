package com.l2.framework.config;

import lombok.Data;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Configuration;

/**
 * @Author: LitchiXian
 * @Date: 2025/8/2 7:31
 * @Desc: MinIOConfig
 */
@Data
@Configuration
public class MinIOConfig {
    @Value("${minio.endPoint}")
    private String endpoint;
    @Value("${minio.fileHost}")
    private String fileHost;
    @Value("${minio.bucketName}")
    private String bucketName;
    @Value("${minio.accessKey}")
    private String accessKey;
    @Value("${minio.secretKey}")
    private String secretKey;
}
