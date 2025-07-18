import request from '@/api/request';

export function save(data) {
    return request({
        url: '/blog/article/save',
        method: 'post',
        data: data
    });
}

export function remove(data) {
    return request({
        url: '/blog/article/remove',
        method: 'post',
        data: data
    });
}

export function update(data) {
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

export function getArticle(data) {
    return request({
        url: '/blog/article/get',
        method: 'get',
        params: data
    });
}