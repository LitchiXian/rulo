// 对应后端 SysDept
export interface SysDept {
  id: string
  parent_id: string
  name: string
  sort_order: number
  leader: string | null
  phone: string | null
  email: string | null
  is_active: boolean
  is_deleted: boolean
  create_id: string
  create_time: string
  update_id: string
  update_time: string
  remark: string | null
}

export interface SysDeptSaveDto {
  parent_id?: string
  name: string
  sort_order?: number
  leader?: string
  phone?: string
  email?: string
  remark?: string
}

export interface SysDeptUpdateDto {
  id: string
  parent_id?: string
  name?: string
  sort_order?: number
  is_active?: boolean
  leader?: string
  phone?: string
  email?: string
  remark?: string
}

export interface SysDeptListDto {
  name?: string
  is_active?: boolean
  parent_id?: string
  create_start_time?: string
  create_end_time?: string
}
