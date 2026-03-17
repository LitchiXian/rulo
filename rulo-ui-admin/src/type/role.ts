// 对应后端 SysRole
export interface SysRole {
  id: number
  role_name: string
  role_key: string
  is_super: boolean
  is_active: boolean
  is_deleted: boolean
  create_id: number
  create_time: string
  update_id: number
  update_time: string
  remark: string | null
}

export interface SysRoleSaveDto {
  role_name: string
  role_key: string
  remark?: string
}

export interface SysRoleUpdateDto {
  id: number
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
}
