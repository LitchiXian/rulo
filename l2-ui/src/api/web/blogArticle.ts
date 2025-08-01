import request from '@/util/request.ts';
import type {Article} from "@/type/article.ts";

// 定义 API 方法集合
const blogArticleApi = {
    /**
     * 保存文章
     * @param data 文章数据
     */
    save(data: any) {
        return request({
            url: '/blog/article/save',
            method: 'post',
            data: data
        });
    },

    /**
     * 删除文章
     * @param data 包含文章ID的数据
     */
    remove(data: { id: string }) {
        return request({
            url: '/blog/article/remove',
            method: 'post',
            data: data
        });
    },

    /**
     * 更新文章
     * @param data 更新后的文章数据
     */
    update(data: Article) {
        return request({
            url: '/blog/article/update',
            method: 'post',
            data: data
        });
    },

    /**
     * 获取文章列表
     */
    list() {
        return request<Article[]>({
            url: '/blog/article/list',
            method: 'get'
        });
    },

    /**
     * 获取单篇文章详情
     * @param params 包含文章ID的参数
     */
    getInfo(params: { id: string }): Promise<Article> {
        return request({
            url: '/blog/article/get',
            method: 'get',
            params: params
        });
    },

    /**
     * 获取用户文章列表
     * @param params 获取用户文章列表的参数
     */
    getUserArticleList(params: { id: string }) {
        return request({
            url: '/blog/article/getUserArticleList',
            method: 'get',
            params: params
        });
    },
};

// 统一导出 API 对象
export default blogArticleApi;