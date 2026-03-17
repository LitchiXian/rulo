import request from '@/util/request'
import type { SysRole, SysRoleSaveDto, SysRoleUpdateDto, SysRoleListDto } from '@/type/role'
import type { IdsDto } from '@/type/user'

const roleApi = {
  list(params?: SysRoleListDto) {
    return request({ url: '/system/role/list', method: 'get', params }) as Promise<SysRole[]>
  },

  detail(id: number) {
    return request({ url: '/system/role/detail', method: 'get', params: { id } }) as Promise<SysRole>
  },

  save(data: SysRoleSaveDto) {
    return request({ url: '/system/role/save', method: 'post', data }) as Promise<SysRole>
  },

  update(data: SysRoleUpdateDto) {
    return request({ url: '/system/role/update', method: 'post', data })
  },

  remove(data: IdsDto) {
    return request({ url: '/system/role/remove', method: 'post', data })
  },
}

export default roleApi
