import request from '@/api/request.js';

export function login(data: any) {
    return request({
        url: '/login/login',
        method: 'post',
        data: data
    });
}

export function register(data: any) {
    return request({
        url: '/login/register',
        method: 'post',
        data: data
    });
}

export function logout() {
    return request({
        url: '/login/logout',
        method: 'get',
    });
}

export function getRegisterCode(data: any) {
    return request({
        url: '/login/getRegisterCode',
        method: 'get',
        params: data
    });
}

export function getLoginInfo() {
    return request({
        url: '/login/getLoginInfo',
        method: 'get'
    });
}
