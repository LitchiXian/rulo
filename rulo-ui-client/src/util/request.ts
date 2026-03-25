import axios from 'axios'
import { ElMessage } from 'element-plus'

// 从 pinia-plugin-persistedstate 持久化的 localStorage 中读取 token
// key 与 store/user.ts 中 persist.key 保持一致
function getToken(): string {
  try {
    const raw = localStorage.getItem('client-user')
    if (!raw) return ''
    return JSON.parse(raw).token || ''
  } catch {
    return ''
  }
}

const request = axios.create({
  baseURL: '',
  timeout: 10000,
  headers: { 'Content-Type': 'application/json' },
})

// 请求拦截器：自动附带 token
request.interceptors.request.use(
  (config) => {
    const token = getToken()
    if (token && config.headers) {
      config.headers['authorization'] = token
    }
    return config
  },
  (error) => Promise.reject(error),
)

// 响应拦截器：统一处理业务码和错误
request.interceptors.response.use(
  (response) => {
    const res = response.data
    const code = res.code

    // 登录过期
    if (code === 'A0230' || code === 40100) {
      ElMessage.error('登录已过期，请重新登录')
      localStorage.removeItem('client-user')
      window.location.href = '/login'
      return Promise.reject(new Error('登录过期'))
    }

    // 限流 (TooManyRequests: 42900)
    if (code === 42900) {
      ElMessage.warning(res.message || res.msg || '请求过于频繁，请稍后再试')
      return Promise.reject(new Error('请求限流'))
    }

    // 业务成功
    if (code === '200' || code === 200) {
      return res.data
    }

    // 业务失败
    const msg = res.msg || res.message || '操作失败'
    ElMessage.error(msg)
    return Promise.reject(new Error(msg))
  },
  (error) => {
    const code = error.response?.data?.code
    if (code === 'A0230' || code === 40100) {
      ElMessage.error('登录已过期，请重新登录')
      localStorage.removeItem('client-user')
      window.location.href = '/login'
      return Promise.reject(error)
    }
    if (code === 42900) {
      ElMessage.warning(error.response?.data?.message || error.response?.data?.msg || '请求过于频繁，请稍后再试')
      return Promise.reject(error)
    }
    const msg = error.response?.data?.msg || error.response?.data?.message || error.message || '请求失败'
    ElMessage.error(msg)
    return Promise.reject(error)
  },
)

export default request