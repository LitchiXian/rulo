// 对应后端 SysPermission
export interface SysPermission {
  id: string
  perm_code: string        // e.g. "sys:user:list"
  perm_name: string
  perm_type: number        // 1=菜单权限 2=按钮权限 3=API权限
  is_deleted: boolean
  create_id: string
  create_time: string
  update_id: string
  update_time: string
  remark: string | null
}

export interface SysPermissionSaveDto {
  perm_code: string
  perm_name: string
  perm_type: number
  remark?: string
}

export interface SysPermissionUpdateDto {
  id: string
  perm_name?: string
  perm_type?: number
  remark?: string
}

export interface SysPermissionListDto {
  perm_code?: string
  perm_name?: string
  perm_type?: number
  create_start_time?: string
  create_end_time?: string
  page_num?: number
  page_size?: number
}
