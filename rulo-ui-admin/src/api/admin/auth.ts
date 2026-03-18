import request from '@/util/request'
import type { LoginInfoVo } from '@/type/user'

const authApi = {
  login(data: { username: string; password: string }) {
    return request({ url: '/system/auth/login', method: 'post', data })
  },

  logout() {
    return request({ url: '/system/auth/logout', method: 'post' })
  },

  getLoginInfo(): Promise<LoginInfoVo> {
    return request({ url: '/system/auth/info', method: 'get' }) as Promise<LoginInfoVo>
  },
}

export default authApi
