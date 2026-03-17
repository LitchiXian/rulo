// 用户信息（对应后端 SysUser 序列化字段）
export interface UserInfo {
  id: number
  user_name: string
  nick_name: string
  email: string | null
  is_active: boolean
}

// 登录参数
export interface LoginDto {
  username: string
  password: string
}

// 统一响应格式（与 Rust 后端对应）
export interface ApiResponse<T> {
  code: number
  message: string
  data: T | null
}
