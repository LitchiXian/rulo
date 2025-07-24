// 用户信息类型
export interface UserInfo {
    id: number;
    userName: string;
    nickName: string;
    avatar: string;
    email: string;
    remark: string;
    // 添加其他用户字段
}

// API 响应类型
export interface ApiResponse<T> {
    code: number;
    message: string;
    data: T;
}

// 登录凭证类型
export interface LoginDto {
    userName: string;
    password: string;
    remember?: boolean;
    redirect?: string;
}