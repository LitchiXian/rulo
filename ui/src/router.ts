import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router'

export const constantRoutes: RouteRecordRaw[] = [
    {
        path: '/',
        // 在 App.vue 中定义了 MainLayout.vue，所以这里不用再设置了，不然会嵌套 layout
        // component: () => import('@/layouts/MainLayout.vue'),
        children: [
            {path: '', name: 'Home', component: () => import('@/page/Home.vue')},
            {path: 'post/:id', name: 'Post', component: () => import('@/page/Post.vue')},
            {path: 'saveArticle', name: 'SaveArticle', component: () => import('@/page/SaveArticle.vue')},
        ],
        meta: {noLayout: false}
    },
    {
        path: '/login',
        component: () => import('@/page/Login.vue'),
        meta: {noLayout: true}
    },
    {
        path: '/register',
        component: () => import('@/page/Register.vue'),
        meta: {noLayout: true}
    },
    {
        path: '/:pathMatch(.*)*',
        component: () => import('@/page/NotFound.vue'),
        meta: {noLayout: true}
    },
]

const router = createRouter({
    // history: createWebHistory(import.meta.env.VITE_APP_CONTEXT_PATH),
    history: createWebHistory('/'),
    routes: constantRoutes
})

export default router