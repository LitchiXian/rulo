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

export function logout(data: any) {
    return request({
        url: '/login/logout',
        method: 'post',
        data: data
    });
}

export function getRegisterCode(data: any) {
    return request({
        url: '/login/getRegisterCode',
        method: 'get',
        params: data
    });
}
