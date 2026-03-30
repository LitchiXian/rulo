// 对应后端 SysRole
export interface SysRole {
  id: string
  role_name: string
  role_key: string
  is_super: boolean
  is_active: boolean
  is_deleted: boolean
  create_id: string
  create_time: string
  update_id: string
  update_time: string
  remark: string | null
}

export interface SysRoleSaveDto {
  role_name: string
  role_key: string
  remark?: string
}

export interface SysRoleUpdateDto {
  id: string
  role_name?: string
  role_key?: string
  is_active?: boolean
  remark?: string
}

export interface SysRoleListDto {
  role_name?: string
  role_key?: string
  is_active?: boolean
  create_start_time?: string
  create_end_time?: string
  page_num?: number
  page_size?: number
}

export interface BindMenusDto {
  role_id: string
  menu_ids: string[]
}

export interface BindPermsDto {
  role_id: string
  perm_ids: string[]
}
