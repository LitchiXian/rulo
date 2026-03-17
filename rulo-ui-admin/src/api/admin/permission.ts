import request from '@/util/request'
import type {
  SysPermission,
  SysPermissionSaveDto,
  SysPermissionUpdateDto,
  SysPermissionListDto,
} from '@/type/permission'
import type { IdsDto } from '@/type/user'

const permissionApi = {
  list(params?: SysPermissionListDto) {
    return request({ url: '/system/permission/list', method: 'get', params }) as Promise<SysPermission[]>
  },

  detail(id: number) {
    return request({ url: '/system/permission/detail', method: 'get', params: { id } }) as Promise<SysPermission>
  },

  save(data: SysPermissionSaveDto) {
    return request({ url: '/system/permission/save', method: 'post', data }) as Promise<SysPermission>
  },

  update(data: SysPermissionUpdateDto) {
    return request({ url: '/system/permission/update', method: 'post', data })
  },

  remove(data: IdsDto) {
    return request({ url: '/system/permission/remove', method: 'post', data })
  },
}

export default permissionApi
