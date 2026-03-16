import request from '@/util/request.ts';
import type {Tag} from "@/type/article.ts";

export const blogTagApi = {

    list(): Promise<Tag[]> {
        return request({
            url: '/blog/tag/list',
            method: 'get'
        });
    },

}