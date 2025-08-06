package com.l2.framework.config;

import cn.dev33.satoken.context.SaHolder;
import cn.dev33.satoken.context.SaTokenContext;
import cn.dev33.satoken.context.model.SaRequest;
import cn.dev33.satoken.interceptor.SaInterceptor;
import cn.dev33.satoken.stp.StpUtil;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

/**
 * @Author: Linzx
 * @Date: 2025/7/23 14:41
 * @Desc: SaTokenConfigure
 */
@Configuration
public class SaTokenConfig implements WebMvcConfigurer {
    // 注册拦截器
    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        // 注册 Sa-Token 拦截器，校验规则为 StpUtil.checkLogin() 登录校验。
        registry.addInterceptor(new SaInterceptor(handle ->{
                    // 正确获取请求对象的方式
                    SaRequest saRequest = SaHolder.getRequest();

                    // 检查是否为OPTIONS请求
                    if (saRequest != null && "OPTIONS".equalsIgnoreCase(saRequest.getMethod())) {
                        // 跨域预检 // TODO 但我担心会不会有漏洞，方法全带 OPTIONS 来访问，
                        // 放行OPTIONS请求
                        return;
                    }
                    StpUtil.checkLogin();
                }))
                .addPathPatterns("/**")
                .excludePathPatterns("/login/login", "/login/register", "/login/getRegisterCode")
                .excludePathPatterns("/blog/article/list", "/blog/article/get", "/blog/article/getUserArticleList")
                .excludePathPatterns("/blog/tag/list")
                .excludePathPatterns(
                        "/doc.html",
                        "/v3/api-docs/**",
                        "/favicon.ico",
                        "/**/*.js",
                        "/**/*.css",
                        "/**/*.png",
                        "/**/*.jpg",
                        "/**/*.gif",
                        "/**/*.map",
                        "/**/*.ttf",
                        "/**/*.woff",
                        "/**/*.woff2"
                );
    }
}
