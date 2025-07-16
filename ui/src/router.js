import {createRouter, createWebHistory} from 'vue-router'

const routes = [
    {
        path: '/',
        component: () => import('@/layouts/MainLayout.vue'),
        children: [
            {path: '', name: 'Home', component: () => import('@/pages/Home.vue')},
            {path: 'post/:id', name: 'Post', component: () => import('@/pages/Post.vue')},
            {path: 'saveArticle', name: 'SaveArticle', component: () => import('@/pages/SaveArticle.vue')},
        ]
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