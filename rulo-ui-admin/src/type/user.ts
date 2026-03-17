// 用户信息（对应后端 SysUser 序列化字段）
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
