import request from '@/util/request'
import type { SysMenu, SysMenuSaveDto, SysMenuUpdateDto, SysMenuListDto } from '@/type/menu'
import type { IdsDto } from '@/type/user'
import type { PageResult } from '@/type/common'

const menuApi = {
  list(params?: SysMenuListDto) {
    return request({ url: '/system/menu/list', method: 'get', params }) as Promise<PageResult<SysMenu>>
  },

  listAll(params?: SysMenuListDto) {
    return request({ url: '/system/menu/list-all', method: 'get', params }) as Promise<SysMenu[]>
  },

  detail(id: number) {
    return request({ url: '/system/menu/detail', method: 'get', params: { id } }) as Promise<SysMenu>
  },

  save(data: SysMenuSaveDto) {
    return request({ url: '/system/menu/save', method: 'post', data }) as Promise<SysMenu>
  },

  update(data: SysMenuUpdateDto) {
    return request({ url: '/system/menu/update', method: 'post', data })
  },

  remove(data: IdsDto) {
    return request({ url: '/system/menu/remove', method: 'post', data })
  },
}

export default menuApi
