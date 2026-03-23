// 脱敏用户信息（对应后端 UserInfoVo，info 接口返回）
export interface UserInfoVo {
  id: number
  user_name: string
  nick_name: string
  email: string | null
  is_active: boolean
  remark: string | null
}

// 菜单树节点（对应后端 MenuTreeNode）
export interface MenuTreeNode {
  id: number
  parent_id: number
  name: string
  menu_type: number // 1=目录 2=菜单 3=外链
  path: string | null
  component: string | null
  icon: string | null
  sort_order: number
  is_hidden: boolean
  children: MenuTreeNode[]
}

// info 接口完整返回（对应后端 LoginInfoVo）
export interface LoginInfoVo {
  user: UserInfoVo
  perms: string[]
  menus: MenuTreeNode[]
}

// 完整用户信息（对应后端 SysUser，管理页面用）
export interface UserInfo {
  id: number
  user_name: string
  nick_name: string
  email: string | null
  is_active: boolean
  is_deleted: boolean
  create_id: number
  create_time: string
  update_id: number
  update_time: string
  remark: string | null
}

// 登录参数
export interface LoginDto {
  username: string
  password: string
}

export interface SysUserSaveDto {
  nick_name: string
  password: string
  email?: string
  remark?: string
}

export interface SysUserUpdateDto {
  id: number
  nick_name?: string
  password?: string
  email?: string
  remark?: string
}

export interface SysUserListDto {
  nick_name?: string
  email?: string
  create_start_time?: string
  create_end_time?: string
  remark?: string
  page_num?: number
  page_size?: number
}

export interface BindRolesDto {
  user_id: number
  role_ids: number[]
}

// 通用 DTO（对应后端 IdDto / IdsDto）
export interface IdDto {
  id: number
}

export interface IdsDto {
  ids: number[]
}

// 统一响应格式（与 Rust 后端对应）
export interface ApiResponse<T> {
  code: number
  message: string
  data: T | null
}
