import {createRouter, createWebHistory} from 'vue-router';
import type {RouteRecordRaw} from 'vue-router';

export const constantRoutes: RouteRecordRaw[] = [
    {
        path: '/',
        // 在 App.vue 中定义了 MainLayout.vue，所以这里不用再设置了，不然会嵌套 layout
        // component: () => import('@/layouts/MainLayout.vue'),
        children: [
            {path: '', name: 'Home', component: () => import('@/view/Home.vue')},
            {path: 'post/:id', name: 'Post', component: () => import('@/view/Post.vue')},
            {path: 'saveArticle', name: 'SaveArticle', component: () => import('@/view/SaveArticle.vue')},
        ],
        meta: {noLayout: false}
    },
    {
        path: '/login',
        name: 'Login',
        component: () => import('@/view/Login.vue'),
        meta: {noLayout: true}
    },
    {
        path: '/register',
        name: 'Register',
        component: () => import('@/view/Register.vue'),
        meta: {noLayout: true}
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        component: () => import('@/view/NotFound.vue'),
        meta: {noLayout: true}
    },
]

const router = createRouter({
    // history: createWebHistory(import.meta.env.VITE_APP_CONTEXT_PATH),
    history: createWebHistory('/'),
    routes: constantRoutes
})

// 全局路由守卫
router.beforeEach(async (to, from) => {
    const {useUserStore} = await import("@/store/user.ts");

    const userStore = useUserStore();
    if (!userStore.userInfo && userStore.isLoggedIn) {
        await userStore.initUser();
    }

    if (to.meta.requiresAuth && !userStore.isLoggedIn) {
        return {name: 'Login', query: {redirect: to.fullPath}}
    }

    if (to.name === 'Login' && userStore.isLoggedIn) {
        return from.path || '/'
    }
})

export default router;