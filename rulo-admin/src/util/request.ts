import axios from 'axios'
import { ElMessage } from 'element-plus'

// 从 pinia-plugin-persistedstate 持久化的 localStorage 中读取 token
// key 与 store/user.ts 中 persist.key 保持一致
export function getToken(): string {
  try {
    const raw = localStorage.getItem('admin-user')
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

// 统一处理业务错误码
function handleBizError(code: number, message?: string): boolean {
  if (code === 40100) {
    ElMessage.error('登录已过期，请重新登录')
    localStorage.removeItem('admin-user')
    window.location.href = '/login'
    return true
  }
  if (code === 42900) {
    ElMessage.warning(message || '请求过于频繁，请稍后再试')
    return true
  }
  if (code === 40300) {
    ElMessage.error('权限不足，拒绝访问')
    return true
  }
  if (code === 40400) {
    ElMessage.warning('资源不存在')
    return true
  }
  return false
}

// 响应拦截器：统一处理业务码和错误
request.interceptors.response.use(
  (response) => {
    const res = response.data
    const code = res.code

    if (handleBizError(code, res.message)) {
      return Promise.reject(new Error(res.message || '请求错误'))
    }

    if (code === 200) {
      return res.data
    }

    const msg = res.message || '操作失败'
    ElMessage.error(msg)
    return Promise.reject(new Error(msg))
  },
  (error) => {
    const code = error.response?.data?.code
    const message = error.response?.data?.message

    if (code && handleBizError(code, message)) {
      return Promise.reject(error)
    }

    ElMessage.error(message || error.message || '请求失败')
    return Promise.reject(error)
  },
)

export default request
