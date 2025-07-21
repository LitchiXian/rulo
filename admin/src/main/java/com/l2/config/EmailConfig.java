package com.l2.config;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

/**
 * @Author: Linzx
 * @Date: 2025/7/21 19:23
 * @Desc: EmailConfig
 */
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

    public String getProtocol() {
        return protocol;
    }

    public void setProtocol(String protocol) {
        this.protocol = protocol;
    }

    public String getHost() {
        return host;
    }

    public void setHost(String host) {
        this.host = host;
    }

    public String getPort() {
        return port;
    }

    public void setPort(String port) {
        this.port = port;
    }

    public String getAccount() {
        return account;
    }

    public void setAccount(String account) {
        this.account = account;
    }

    public String getPassword() {
        return password;
    }

    public void setPassword(String password) {
        this.password = password;
    }
}
