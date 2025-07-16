import {createRouter, createWebHistory} from 'vue-router'

const routes = [
    {
        path: '/',
        // 在 App.vue 中定义了 MainLayout.vue，所以这里不用再设置了，不然会嵌套 layout
        // component: () => import('@/layouts/MainLayout.vue'),
        children: [
            {path: '', name: 'Home', component: () => import('@/pages/Home.vue')},
            {path: 'post/:id', name: 'Post', component: () => import('@/pages/Post.vue')},
            {path: 'saveArticle', name: 'SaveArticle', component: () => import('@/pages/SaveArticle.vue')},
        ],
        meta: {noLayout: false}
    },
    {
        path: '/login',
        component: () => import('@/pages/Login.vue'),
        meta: {noLayout: true}
    },
    {
        path: '/:pathMatch(.*)*',
        component: () => import('@/pages/NotFound.vue'),
        meta: {noLayout: true}
    },
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
})

export default router