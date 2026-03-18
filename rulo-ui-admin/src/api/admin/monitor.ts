import request from '@/util/request'
import type { ServerInfo } from '@/type/monitor'

const monitorApi = {
  getServerInfo() {
    return request({ url: '/system/monitor/server_info', method: 'get' }) as Promise<ServerInfo>
  },
}

export default monitorApi
