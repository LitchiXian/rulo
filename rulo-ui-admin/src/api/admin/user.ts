import request from '@/util/request'
import type { UserInfo, SysUserSaveDto, SysUserUpdateDto, SysUserListDto, IdsDto, BindRolesDto } from '@/type/user'

const userApi = {
  list(params?: SysUserListDto) {
    return request({ url: '/system/user/list', method: 'get', params }) as Promise<UserInfo[]>
  },

  detail(id: number) {
    return request({ url: '/system/user/detail', method: 'get', params: { id } }) as Promise<UserInfo>
  },

  save(data: SysUserSaveDto) {
    return request({ url: '/system/user/save', method: 'post', data }) as Promise<UserInfo>
  },

  update(data: SysUserUpdateDto) {
    return request({ url: '/system/user/update', method: 'post', data })
  },

  remove(data: IdsDto) {
    return request({ url: '/system/user/remove', method: 'post', data })
  },

  updateBindRoles(data: BindRolesDto) {
    return request({ url: '/system/user/update-bind-roles', method: 'post', data })
  },

  listRoles(id: number) {
    return request({ url: '/system/user/list-bind-roles', method: 'get', params: { id } }) as Promise<number[]>
  },
}

export default userApi
