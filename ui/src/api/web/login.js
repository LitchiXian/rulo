import request from '@/api/request';

export function login(data) {
    return request({
        url: '/login/login',
        method: 'post',
        data: data
    });
}

export function register(data) {
    return request({
        url: '/login/register',
        method: 'post',
        data: data
    });
}
