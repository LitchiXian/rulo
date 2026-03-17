import request from '@/util/request'
import type { UserInfo } from '@/type/user'

const authApi = {
  login(data: { username: string; password: string }) {
    return request({ url: '/system/auth/login', method: 'post', data })
  },

  logout() {
    return request({ url: '/system/auth/logout', method: 'post' })
  },

  getLoginInfo(): Promise<UserInfo> {
    return request({ url: '/system/auth/info', method: 'get' }) as Promise<UserInfo>
  },
}

export default authApi
