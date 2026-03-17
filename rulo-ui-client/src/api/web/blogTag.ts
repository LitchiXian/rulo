import request from '@/util/request'
import type { Tag } from '@/type/article'

export const blogTagApi = {

    list(): Promise<Tag[]> {
        return request({
            url: '/blog/tag/list',
            method: 'get'
        });
    },

}