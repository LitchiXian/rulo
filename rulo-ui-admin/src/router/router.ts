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
    redirect: '/dashboard',
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('@/view/Home.vue'),
    meta: { requiresAuth: true, title: '首页' },
  },
  // TODO: 系统管理菜单目前是写死的静态路由，后续改为动态路由：
  //   1. 登录后调用后端接口获取当前用户拥有的菜单权限（根据 sys_role_menu / sys_role_permission）
  //   2. 根据返回的菜单数据动态 addRoute()，而非在此处静态注册
  //   3. 侧边栏菜单也应从动态路由数据渲染，而非模板写死
  {
    path: '/system',
    name: 'System',
    redirect: '/system/user',
    meta: { requiresAuth: true, title: '系统管理' },
    children: [
      {
        path: 'user',
        name: 'SystemUser',
        component: () => import('@/view/system/user/index.vue'),
        meta: { requiresAuth: true, title: '用户管理' },
      },
      {
        path: 'role',
        name: 'SystemRole',
        component: () => import('@/view/system/role/index.vue'),
        meta: { requiresAuth: true, title: '角色管理' },
      },
      {
        path: 'menu',
        name: 'SystemMenu',
        component: () => import('@/view/system/menu/index.vue'),
        meta: { requiresAuth: true, title: '菜单管理' },
      },
      {
        path: 'permission',
        name: 'SystemPermission',
        component: () => import('@/view/system/permission/index.vue'),
        meta: { requiresAuth: true, title: '权限管理' },
      },
    ],
  },
  {
    path: '/monitor',
    name: 'Monitor',
    redirect: '/monitor/server',
    meta: { requiresAuth: true, title: '系统监控' },
    children: [
      {
        path: 'server',
        name: 'MonitorServer',
        component: () => import('@/view/monitor/server/index.vue'),
        meta: { requiresAuth: true, title: '服务监控' },
      },
    ],
  },
  {
    path: '/other',
    name: 'Other',
    redirect: '/other/about',
    meta: { requiresAuth: true, title: '其他' },
    children: [
      {
        path: 'about',
        name: 'About',
        component: () => import('@/view/other/about/index.vue'),
        meta: { requiresAuth: true, title: '关于我们' },
      },
    ],
  },
  {
    path: '/changelog',
    name: 'Changelog',
    component: () => import('@/view/changelog/index.vue'),
    meta: { requiresAuth: true, title: '更新日志' },
  },
  {
    path: '/profile',
    name: 'Profile',
    component: () => import('@/view/profile/index.vue'),
    meta: { requiresAuth: true, title: '个人中心' },
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/view/Login.vue'),
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

// 路由守卫：进度条 + 动态标题 + 权限
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
    return { name: 'Dashboard' }
  }
})

router.afterEach((to) => {
  // 动态页面标题
  const title = to.meta.title ? `${to.meta.title} - Rulo Admin` : 'Rulo Admin'
  document.title = title
  NProgress.done()
})

export default router
