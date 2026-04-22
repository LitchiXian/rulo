import request from '@/util/request'
import type { SysDept, SysDeptSaveDto, SysDeptUpdateDto, SysDeptListDto } from '@/type/dept'
import type { IdsDto } from '@/type/user'

const deptApi = {
  listAll(params?: SysDeptListDto) {
    return request({ url: '/system/dept/list-all', method: 'get', params }) as Promise<SysDept[]>
  },

  detail(id: string) {
    return request({ url: '/system/dept/detail', method: 'get', params: { id } }) as Promise<SysDept>
  },

  save(data: SysDeptSaveDto) {
    return request({ url: '/system/dept/save', method: 'post', data }) as Promise<SysDept>
  },

  update(data: SysDeptUpdateDto) {
    return request({ url: '/system/dept/update', method: 'post', data })
  },

  remove(data: IdsDto) {
    return request({ url: '/system/dept/remove', method: 'post', data })
  },
}

export default deptApi
