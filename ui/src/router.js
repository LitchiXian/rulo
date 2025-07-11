import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        name: 'Home',
        component: () => import('@/pages/Home.vue')
    },
    {
        path: '/post/:id',
        name: 'Post',
        component: () => import('@/pages/Post.vue'),
        props: true
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        component: () => import('@/pages/NotFound.vue')
    },
    {
        path: '/saveArticle',
        name: 'SaveArticle',
        component: () => import('@/pages/SaveArticle.vue')
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
})

export default router