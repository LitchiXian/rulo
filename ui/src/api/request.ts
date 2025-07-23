import axios from 'axios';
import router from '@/router'; // 引入路由实例
import { ElMessage } from 'element-plus'; // 引入消息提示组件

// 创建 axios 实例
const service = axios.create({
    baseURL: '/api',
    timeout: 7000,
});

// 请求拦截器
service.interceptors.request.use(
    (config) => {
        // 保存当前路由路径用于登录后跳回
        if (!config.url?.includes('/login') && !(config as any)._isRetry) {
            (config as any)._returnUrl = window.location.pathname + window.location.search;
        }

        // 添加 token 认证信息
        const tokenValue = localStorage.getItem('satoken');
        if (tokenValue && config.headers) {
            config.headers['satoken'] = tokenValue;
        }
        return config;
    },
    (error) => {
        return Promise.reject(error);
    }
);

// 响应拦截器
service.interceptors.response.use(
    (response) => {
        const res = response.data;

        console.log('API Response', res);
        // 检查业务错误码
        if (res.data.code === 'A0230') { // 登录过期
            return handleSessionExpired(response.config);
        }

        return res;
    },
    (error) => {
        // 统一处理错误
        console.error('API Error', error);

        // 处理登录过期情况
        if (error.response?.data?.code === 'A0230') {
            return handleSessionExpired(error.config);
        }

        // 显示错误消息
        const errorMessage = error.response?.data?.message || error.message || '请求失败';
        ElMessage.error(errorMessage);

        return Promise.reject(error);
    }
);

/**
 * 处理会话过期逻辑
 * @param originalRequest 原始请求配置
 */
function handleSessionExpired(originalRequest?: any): Promise<never> {
    // 避免重复处理
    if (originalRequest?._isRetry) return Promise.reject();
    if (originalRequest) originalRequest._isRetry = true;

    // 保存原始请求路径
    const returnUrl = originalRequest?._returnUrl || window.location.pathname + window.location.search;
    localStorage.setItem('returnUrl', returnUrl);

    // 清除过期 token
    localStorage.removeItem('satoken');

    // 显示提示
    ElMessage.warning('登录已过期，请重新登录');

    // 跳转到登录页
    router.replace({
        path: '/login',
        query: { redirect: returnUrl }
    });

    return Promise.reject(new Error('登录已过期'));
}

export default service;