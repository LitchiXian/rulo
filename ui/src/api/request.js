import axios from 'axios';

// 创建 axios 实例
const service = axios.create({
    //baseURL: import.meta.env.VITE_API_BASE, // 从环境变量获取基础 URL
    baseURL: import.meta.env.VITE_API_BASE || 'http://localhost:8090', // 从环境变量获取基础 URL
    // baseURL: import.meta.env.VITE_API_BASE || '/api', // 从环境变量获取基础 URL
    timeout: 7000 // 请求超时时间
})

// 请求拦截器
service.interceptors.request.use(
    config => {
        // 这里可以添加 token 等认证信息
        return config
    },
    error => {
        return Promise.reject(error)
    }
)

// 响应拦截器
service.interceptors.response.use(
    response => {
        // console.log('API Response', response)
        // 处理成功响应数据
        return response.data
    },
    error => {
        // 统一处理错误
        console.error('API Error', error)
        return Promise.reject(error)
    }
)

export default service;