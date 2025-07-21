import request from '@/api/request.js';

export function save(data: any) {
    return request({
        url: '/blog/article/save',
        method: 'post',
        data: data
    });
}

export function remove(data: any) {
    return request({
        url: '/blog/article/remove',
        method: 'post',
        data: data
    });
}

export function update(data: any) {
    return request({
        url: '/blog/article/update',
        method: 'post',
        data: data
    });
}

export function list() {
    return request({
        url: '/blog/article/list',
        method: 'get'
    });
}

export function getArticle(data: any) {
    return request({
        url: '/blog/article/get',
        method: 'get',
        params: data
    });
}