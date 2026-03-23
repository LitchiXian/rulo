// 对应后端 SysMenu
export interface SysMenu {
  id: number
  parent_id: number
  perm_id: number | null
  name: string
  menu_type: number        // 1=目录 2=菜单 3=按钮
  path: string | null
  component: string | null
  icon: string | null
  sort_order: number
  is_hidden: boolean
  is_deleted: boolean
  create_id: number
  create_time: string
  update_id: number
  update_time: string
  remark: string | null
}

export interface SysMenuSaveDto {
  parent_id?: number
  name: string
  menu_type: number
  path?: string
  component?: string
  icon?: string
  sort_order?: number
  remark?: string
  /** 仅 menu_type=2 新增时填写，后端自动创建对应菜单权限并关联 */
  auto_perm_code?: string
}

export interface SysMenuUpdateDto {
  id: number
  name?: string
  path?: string
  component?: string
  icon?: string
  sort_order?: number
  is_hidden?: boolean
  remark?: string
}

export interface SysMenuListDto {
  name?: string
  menu_type?: number
  is_hidden?: boolean
  parent_id?: number
  create_start_time?: string
  create_end_time?: string
  page_num?: number
  page_size?: number
}
