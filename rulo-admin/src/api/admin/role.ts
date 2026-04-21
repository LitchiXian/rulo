import request from '@/util/request'
import type { SysRole, SysRoleSaveDto, SysRoleUpdateDto, SysRoleListDto, BindMenusDto, BindPermsDto } from '@/type/role'
import type { IdsDto } from '@/type/user'
import type { PageResult } from '@/type/common'

const roleApi = {
  list(params?: SysRoleListDto) {
    return request({ url: '/system/role/list', method: 'get', params }) as Promise<PageResult<SysRole>>
  },

  listAll(params?: SysRoleListDto) {
    return request({ url: '/system/role/list-all', method: 'get', params }) as Promise<SysRole[]>
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

  updateBindMenus(data: BindMenusDto) {
    return request({ url: '/system/role/update-bind-menus', method: 'post', data })
  },

  updateBindPerms(data: BindPermsDto) {
    return request({ url: '/system/role/update-bind-perms', method: 'post', data })
  },

  listMenus(id: number) {
    return request({ url: '/system/role/list-bind-menus', method: 'get', params: { id } }) as Promise<number[]>
  },

  listPerms(id: number) {
    return request({ url: '/system/role/list-bind-perms', method: 'get', params: { id } }) as Promise<number[]>
  },
}

export default roleApi
