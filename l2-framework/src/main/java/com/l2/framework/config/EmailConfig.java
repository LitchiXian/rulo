package com.l2.framework.config;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

/**
 * @Author: Linzx
 * @Date: 2025/7/21 19:23
 * @Desc: EmailConfig
 */
@Data
@Component
@ConfigurationProperties(prefix = "email")
public class EmailConfig {
    /**
     * 邮件协议
     */
    private String protocol;
    /**
     * 发件人的SMTP服务器地址（普通QQ邮箱）
     */
    private String host;
    /**
     * 端口
     */
    private String port;
    /**
     * 发件人邮箱地址，这个是普通QQ邮箱
     */
    private String account;
    /**
     * 发件人邮箱授权码
     */
    private String password;
    // 发件人信息
    private String fromName;
    private String fromEmail;
    // 高级配置
    private boolean debug;
    // 连接超时
    private int timeout;
    // 读取超时
    private int connectionTimeout;
    private boolean sslEnable;

}
