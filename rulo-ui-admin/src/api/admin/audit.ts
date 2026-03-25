import request from '@/util/request'
import type { SysAuditLog, AuditLogListDto } from '@/type/audit'
import type { PageResult } from '@/type/common'

const auditApi = {
  list(params?: AuditLogListDto) {
    return request({ url: '/system/audit/list', method: 'get', params }) as Promise<PageResult<SysAuditLog>>
  },
}

export default auditApi
