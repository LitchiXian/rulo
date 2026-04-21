import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw, RouteLocationNormalized } from 'vue-router'
import { useUserStore } from '@/store/user'
import type { MenuTreeNode } from '@/type/user'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

NProgress.configure({ showSpinner: false })

// ---------- 组件懒加载映射（Vite glob import）----------
// DB 存储格式：view/system/user/index.vue → glob key：../view/system/user/index.vue
const viewModules = import.meta.glob('../view/**/*.vue')

function resolveComponent(component: string) {
  return viewModules[`../${component}`]
}

// ---------- 静态路由（不依赖权限，始终存在）----------
export const constantRoutes: RouteRecordRaw[] = [
  { path: '/', redirect: '/dashboard' },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('@/view/Home.vue'),
    meta: { requiresAuth: true, title: '首页' },
  },
  {
    path: '/profile',
    name: 'Profile',
    component: () => import('@/view/profile/index.vue'),
    meta: { requiresAuth: true, title: '个人中心' },
  },
  {
    path: '/changelog',
    name: 'Changelog',
    component: () => import('@/view/changelog/index.vue'),
    meta: { requiresAuth: true, title: '更新日志' },
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

// ---------- 动态路由 ----------
const staticPaths = new Set(['/', '/dashboard', '/profile', '/changelog', '/login'])
let dynamicRouteNames: string[] = []
let dynamicRoutesLoaded = false

/** 递归扁平化菜单树 */
function flattenMenus(menus: MenuTreeNode[]): MenuTreeNode[] {
  const result: MenuTreeNode[] = []
  for (const menu of menus) {
    result.push(menu)
    if (menu.children?.length) result.push(...flattenMenus(menu.children))
  }
  return result
}

/** 从菜单树生成并注册动态路由 */
function addDynamicRoutes(menus: MenuTreeNode[]) {
  for (const name of dynamicRouteNames) router.removeRoute(name)
  dynamicRouteNames = []

  for (const menu of flattenMenus(menus)) {
    if (!menu.path || staticPaths.has(menu.path)) continue

    const routeName = `Dynamic_${menu.id}`

    if (menu.menu_type === 2 && menu.component) {
      const comp = resolveComponent(menu.component)
      if (comp) {
        router.addRoute({
          path: menu.path,
          name: routeName,
          component: comp,
          meta: { requiresAuth: true, title: menu.name },
        })
        dynamicRouteNames.push(routeName)
      }
    } else if (menu.menu_type === 1 && menu.children?.length) {
      const firstChild = menu.children.find(c => c.path && !c.is_hidden)
      if (firstChild?.path) {
        router.addRoute({
          path: menu.path,
          name: routeName,
          redirect: firstChild.path,
          meta: { requiresAuth: true, title: menu.name },
        })
        dynamicRouteNames.push(routeName)
      }
    }
  }
}

function resetDynamicRoutes() {
  for (const name of dynamicRouteNames) router.removeRoute(name)
  dynamicRouteNames = []
  dynamicRoutesLoaded = false
}

// ---------- 路由守卫 ----------
router.beforeEach(async (to: RouteLocationNormalized) => {
  NProgress.start()
  const userStore = useUserStore()

  // 登出后清理动态路由
  if (!userStore.isLoggedIn && dynamicRoutesLoaded) {
    resetDynamicRoutes()
  }

  // token 存在但用户信息未加载 → 初始化
  if (!userStore.userInfo && userStore.isLoggedIn) {
    await userStore.initUser()
  }

  if (to.meta.requiresAuth && !userStore.isLoggedIn) {
    return { name: 'Login', query: { redirect: to.fullPath } }
  }

  if (to.name === 'Login' && userStore.isLoggedIn) {
    return { name: 'Dashboard' }
  }

  // 动态路由未加载 → 从 userStore.menus 生成并重新导航
  if (userStore.isLoggedIn && !dynamicRoutesLoaded) {
    addDynamicRoutes(userStore.menus)
    dynamicRoutesLoaded = true
    return to.fullPath
  }
})

router.afterEach((to) => {
  const title = to.meta.title ? `${to.meta.title} - Rulo Admin` : 'Rulo Admin'
  document.title = title
  NProgress.done()
})

export default router
