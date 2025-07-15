package com.l2.config;

import io.minio.MinioClient;
import org.jetbrains.annotations.NotNull;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@Configuration
public class CorsConfig {

    @Bean
    public WebMvcConfigurer corsConfigurer() {
        return new WebMvcConfigurer() {

            // 配置跨域
            @Override
            public void addCorsMappings(CorsRegistry registry) {
                registry.addMapping("/**")
                        // 允许的域名 // 尝试用*，失败了
                        .allowedOrigins(
                                "http://localhost",
                                "https://sosdan.cn",
                                "https://*.sosdan.cn"
                        )
                        // 允许的HTTP方法
                        .allowedMethods("GET", "POST")
                        // 允许的请求头
                        .allowedHeaders("*")
                        // 是否允许携带凭证（如cookies）
                        .allowCredentials(true);
            }
        };
    }

    @Bean
    public MinioClient minioClient() {
        return MinioClient.builder()
                .endpoint("http://localhost:9000")
                .credentials("minioadmin", "minioadmin")
                .build();
    }
}
