import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw, RouteLocationNormalized } from 'vue-router'
import { useUserStore } from '@/store/user'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

// 关闭 NProgress 的转圈图标，只保留顶部进度条
NProgress.configure({ showSpinner: false })

export const constantRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    children: [
      { path: '', name: 'Home', component: () => import('@/view/Home.vue'), meta: { title: '首页' } },
      { path: 'post/:id', name: 'Post', component: () => import('@/view/Post.vue'), meta: { title: '文章' } },
      { path: 'saveArticle', name: 'SaveArticle', component: () => import('@/view/SaveArticle.vue'), meta: { requiresAuth: true, title: '编辑文章' } },
      { path: 'user/:id', name: 'User', component: () => import('@/view/User.vue'), meta: { title: '用户' } },
    ],
    meta: { noLayout: false },
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/view/Login.vue'),
    meta: { noLayout: true },
  },
  {
    path: '/register',
    name: 'Register',
    component: () => import('@/view/Register.vue'),
    meta: { noLayout: true },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: () => import('@/view/NotFound.vue'),
    meta: { noLayout: true },
  },
]

const router = createRouter({
  history: createWebHistory('/'),
  routes: constantRoutes,
})

// 路由守卫：进度条 + 权限
router.beforeEach(async (to: RouteLocationNormalized) => {
  NProgress.start()
  const userStore = useUserStore()

  // token 存在但用户信息未加载时，初始化用户信息
  if (!userStore.userInfo && userStore.isLoggedIn) {
    await userStore.initUser()
  }

  if (to.meta.requiresAuth && !userStore.isLoggedIn) {
    return { name: 'Login', query: { redirect: to.fullPath } }
  }

  if (to.name === 'Login' && userStore.isLoggedIn) {
    return '/'
  }
})

router.afterEach((to) => {
  // 动态页面标题
  const title = to.meta.title ? `${to.meta.title} - Rulo` : 'Rulo'
  document.title = title
  NProgress.done()
})

export default router