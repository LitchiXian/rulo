// 用户信息类型（对应后端 SysUser）
export interface UserInfo {
    id: number;
    user_name: string;
    nick_name: string;
    email: string | null;
    is_active: boolean;
    is_deleted: boolean;
    create_id: number;
    create_time: string;
    update_id: number;
    update_time: string;
    remark: string | null;
}

// API 响应类型
export interface ApiResponse<T> {
    code: number;
    message: string;
    data: T;
}

// 登录凭证类型（对应后端 AuthUserDto）
export interface LoginDto {
    username: string;
    password: string;
}