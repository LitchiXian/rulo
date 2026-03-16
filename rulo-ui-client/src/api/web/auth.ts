import request from '@/util/request.ts';
import type {UserInfo} from "@/type/user.ts";

// 定义请求参数类型
// interface LoginParams {
//     username: string;
//     password: string;
// }
//
// interface RegisterParams {
//     username: string;
//     password: string;
//     email: string;
//     verificationCode: string;
// }
//
// interface VerificationCodeParams {
//     email: string;
// }
//
// interface LoginInfoResponse {
//     userId: number;
//     username: string;
//     roles: string[];
//     permissions: string[];
// }

// 统一导出认证API对象
const authApi = {
    /**
     * 用户登录
     * @param data 登录凭证
     */
    login(data: any) {
        return request({
            url: '/login/login',
            method: 'post',
            data
        });
    },

    /**
     * 用户注册
     * @param data 注册信息
     */
    register(data: any) {
        return request({
            url: '/login/register',
            method: 'post',
            data
        });
    },

    /**
     * 用户登出
     */
    logout() {
        return request({
            url: '/login/logout',
            method: 'get'
        });
    },

    /**
     * 获取注册验证码
     * @param data 邮箱信息
     */
    getRegisterCode(data: any) {
        return request({
            url: '/login/getRegisterCode',
            method: 'get',
            params: data
        });
    },

    /**
     * 获取当前登录用户信息
     */
    // 指定返回类型
    getLoginInfo(): Promise<UserInfo> {
        return request({
            url: '/login/getLoginInfo',
            method: 'get'
        });
    }
};

export default authApi;