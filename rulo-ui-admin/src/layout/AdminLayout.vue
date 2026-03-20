<script setup lang="ts" name="AdminLayout">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Fold, Expand, Odometer, User, SwitchButton, Setting, UserFilled, Key, Menu as MenuIcon, Lock, MoreFilled, InfoFilled, Link, Sunny, Moon, FullScreen, Notebook, Monitor, DataLine } from '@element-plus/icons-vue'
import type { Component } from 'vue'
import type { MenuTreeNode } from '@/type/user'

import { useUserStore } from '@/store/user'
import { useLayoutStore } from '@/store/layout'
import { PROFILE_DECOR_CHANGE_EVENT, loadProfileDecor } from '@/util/profileDecor'
import LayoutSetting from '@/component/LayoutSetting.vue'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const layoutStore = useLayoutStore()

// 图标名 → 组件映射
const iconMap: Record<string, Component> = {
  Odometer, User, Setting, UserFilled, Key, Menu: MenuIcon, Lock,
  MoreFilled, InfoFilled, Link, Notebook, Monitor, DataLine, SwitchButton, Expand, Fold, Sunny, Moon, FullScreen,
}

// 可见菜单树（过滤 is_hidden）
const visibleMenus = computed(() => filterHidden(userStore.menus))

function filterHidden(nodes: MenuTreeNode[]): MenuTreeNode[] {
  return nodes
    .filter(n => !n.is_hidden)
    .map(n => ({ ...n, children: filterHidden(n.children ?? []) }))
}

// 混合模式：顶部只展示一级菜单；侧边展示当前选中一级的子菜单
const topLevelMenus = computed(() => visibleMenus.value)

const mixSidebarMenus = computed(() => {
  if (activeTopKey.value === '/dashboard') return []
  const found = visibleMenus.value.find(m => m.path === activeTopKey.value)
  return found?.children ?? []
})

// 判断是否外链
const isExternal = (path?: string | null) => !!path && /^https?:\/\//.test(path)

// 点击菜单项时判断是否外链
const handleMenuClick = (item: MenuTreeNode) => {
  if (item.path && isExternal(item.path)) {
    window.open(item.path, '_blank')
  }
}

const isCollapsed = ref(false)
const toggleCollapse = () => (isCollapsed.value = !isCollapsed.value)
const showSetting = ref(false)

// 当前高亮菜单项
const activeMenu = computed(() => route.path)

// 混合模式下：顶部菜单高亮的一级路径
const activeTopMenu = computed(() => {
  const parts = route.path.split('/')
  return parts.length >= 2 ? `/${parts[1]}` : '/'
})

// 混合模式下：当前选中的一级菜单对应的子菜单
const activeTopKey = ref(activeTopMenu.value)
watch(activeTopMenu, (v) => { activeTopKey.value = v })

const headerAvatar = ref(loadProfileDecor().avatar)

const handleCommand = async (cmd: string) => {
  if (cmd === 'profile') {
    router.push('/profile')
    return
  }
  if (cmd === 'logout') {
    await userStore.logout()
    router.push('/login')
  }
}

const openProjectUrl = () => {
  window.open('https://github.com/LitchiXian/rulo', '_blank')
}

// 全屏切换
const isFullscreen = ref(!!document.fullscreenElement)
const toggleFullscreen = () => {
  if (document.fullscreenElement) {
    document.exitFullscreen()
  } else {
    document.documentElement.requestFullscreen()
  }
}
const onFullscreenChange = () => {
  isFullscreen.value = !!document.fullscreenElement
}
const syncHeaderAvatar = () => {
  headerAvatar.value = loadProfileDecor().avatar
}

onMounted(() => {
  document.addEventListener('fullscreenchange', onFullscreenChange)
  window.addEventListener(PROFILE_DECOR_CHANGE_EVENT, syncHeaderAvatar)
  syncHeaderAvatar()
})

onUnmounted(() => {
  document.removeEventListener('fullscreenchange', onFullscreenChange)
  window.removeEventListener(PROFILE_DECOR_CHANGE_EVENT, syncHeaderAvatar)
})

// 混合模式：点击顶部一级菜单，切换侧边子菜单；同时跳转到第一个子页面
const handleTopMenuSelect = (index: string) => {
  activeTopKey.value = index
  const found = visibleMenus.value.find(m => m.path === index)
  if (found?.children?.length) {
    const first = found.children[0]
    if (first.path && !isExternal(first.path)) {
      router.push(first.path)
    }
  }
}

// 布局模式判断
const hasSidebar = computed(() => ['left', 'left-top-mix', 'right', 'right-top-mix'].includes(layoutStore.mode))
const hasTopMenu = computed(() => ['left-top-mix', 'top', 'right-top-mix'].includes(layoutStore.mode))
const isMixMode = computed(() => ['left-top-mix', 'right-top-mix'].includes(layoutStore.mode))
const isRight = computed(() => ['right', 'right-top-mix'].includes(layoutStore.mode))
const isTopOnly = computed(() => layoutStore.mode === 'top')

// 侧边菜单模式（水平 or 垂直）
const sidebarMenuMode = computed(() => 'vertical' as const)
</script>

<template>
  <el-container class="admin-container" :class="'layout-' + layoutStore.mode">

    <!-- ========= 左侧侧边栏 (left / left-top-mix) ========= -->
    <el-aside
      v-if="hasSidebar && !isRight"
      :width="isCollapsed ? '64px' : '230px'"
      class="admin-aside"
    >
      <div class="admin-logo" :class="{ collapsed: isCollapsed }">
        <img src="/rulo.ico" class="logo-icon" alt="logo" />
        <span v-show="!isCollapsed" class="logo-text">Rulo Admin</span>
      </div>

      <!-- 非混合：完整菜单 -->
      <el-menu
        v-if="!isMixMode"
        :default-active="activeMenu"
        :collapse="isCollapsed"
        :collapse-transition="false"
        unique-opened
        router
        :mode="sidebarMenuMode"
        class="admin-menu"
      >
        <el-menu-item index="/dashboard">
          <el-icon><Odometer /></el-icon>
          <template #title>首页</template>
        </el-menu-item>
        <template v-for="item in visibleMenus" :key="item.id">
          <el-sub-menu v-if="item.children?.length" :index="item.path ?? ''">
            <template #title>
              <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
              <span>{{ item.name }}</span>
            </template>
            <template v-for="child in item.children" :key="child.id">
              <el-menu-item
                v-if="!isExternal(child.path)"
                :index="child.path ?? ''"
              >
                <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                <template #title>{{ child.name }}</template>
              </el-menu-item>
              <el-menu-item v-else :key="'ext-' + child.id" @click="handleMenuClick(child)">
                <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                <template #title>{{ child.name }}</template>
              </el-menu-item>
            </template>
          </el-sub-menu>
          <el-menu-item
            v-else-if="!isExternal(item.path)"
            :index="item.path ?? ''"
          >
            <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
            <template #title>{{ item.name }}</template>
          </el-menu-item>
          <el-menu-item v-else @click="handleMenuClick(item)">
            <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
            <template #title>{{ item.name }}</template>
          </el-menu-item>
        </template>
      </el-menu>

      <!-- 混合：只显示当前一级菜单的子菜单 -->
      <el-menu
        v-else
        :default-active="activeMenu"
        :collapse="isCollapsed"
        :collapse-transition="false"
        unique-opened
        router
        :mode="sidebarMenuMode"
        class="admin-menu"
      >
        <template v-if="activeTopKey === '/dashboard'">
          <el-menu-item index="/dashboard">
            <el-icon><Odometer /></el-icon>
            <template #title>首页</template>
          </el-menu-item>
        </template>
        <template v-else>
          <template v-for="child in mixSidebarMenus" :key="child.id">
            <el-menu-item
              v-if="!isExternal(child.path)"
              :index="child.path ?? ''"
            >
              <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
              <template #title>{{ child.name }}</template>
            </el-menu-item>
            <el-menu-item v-else @click="handleMenuClick(child)">
              <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
              <template #title>{{ child.name }}</template>
            </el-menu-item>
          </template>
          <!-- 无子菜单的一级页面，显示自身 -->
          <template v-if="!mixSidebarMenus.length">
            <el-menu-item
              v-for="item in visibleMenus.filter(m => m.path === activeTopKey && !m.children?.length)"
              :key="item.id"
              :index="item.path ?? ''"
            >
              <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
              <template #title>{{ item.name }}</template>
            </el-menu-item>
          </template>
        </template>
      </el-menu>
    </el-aside>

    <el-container class="admin-right" :class="{ 'flex-row-reverse': isRight && hasSidebar }">
      <!-- ========= 顶部 Header ========= -->
      <el-header class="admin-header" :class="{ 'has-top-menu': hasTopMenu }">
        <div class="header-left">
          <!-- 折叠按钮（有侧边栏时显示） -->
          <el-icon v-if="hasSidebar" class="collapse-btn" :size="20" @click="toggleCollapse">
            <Fold v-if="!isCollapsed" />
            <Expand v-else />
          </el-icon>

          <!-- 顶部菜单模式 logo -->
          <div v-if="isTopOnly" class="top-logo">
            <img src="/rulo.ico" class="logo-icon" alt="logo" />
            <span class="logo-text-dark">Rulo Admin</span>
          </div>

          <!-- 顶部菜单：混合模式显示一级菜单，纯顶部模式显示完整菜单 -->
          <el-menu
            v-if="hasTopMenu && isMixMode"
            :default-active="activeTopKey"
            mode="horizontal"
            class="top-menu"
            @select="handleTopMenuSelect"
          >
            <el-menu-item index="/dashboard">
              <el-icon><Odometer /></el-icon>
              <span>首页</span>
            </el-menu-item>
            <el-menu-item v-for="item in topLevelMenus" :key="item.id" :index="item.path ?? ''">
              <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
              <span>{{ item.name }}</span>
            </el-menu-item>
          </el-menu>

          <el-menu
            v-if="isTopOnly"
            :default-active="activeMenu"
            mode="horizontal"
            router
            class="top-menu"
          >
            <el-menu-item index="/dashboard">
              <el-icon><Odometer /></el-icon>
              <span>首页</span>
            </el-menu-item>
            <template v-for="item in visibleMenus" :key="item.id">
              <el-sub-menu v-if="item.children?.length" :index="item.path ?? ''">
                <template #title>
                  <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
                  <span>{{ item.name }}</span>
                </template>
                <template v-for="child in item.children" :key="child.id">
                  <el-menu-item
                    v-if="!isExternal(child.path)"
                    :index="child.path ?? ''"
                  >
                    <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                    <span>{{ child.name }}</span>
                  </el-menu-item>
                  <el-menu-item v-else @click="handleMenuClick(child)">
                    <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                    <span>{{ child.name }}</span>
                  </el-menu-item>
                </template>
              </el-sub-menu>
              <el-menu-item
                v-else-if="!isExternal(item.path)"
                :index="item.path ?? ''"
              >
                <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
                <span>{{ item.name }}</span>
              </el-menu-item>
              <el-menu-item v-else @click="handleMenuClick(item)">
                <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
                <span>{{ item.name }}</span>
              </el-menu-item>
            </template>
          </el-menu>

          <!-- 非顶部模式的面包屑 -->
          <el-breadcrumb v-if="!hasTopMenu" separator="/">
            <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
          </el-breadcrumb>
        </div>

        <div class="header-right">
          <!-- GitHub 图标 -->
          <el-tooltip content="项目地址" placement="bottom">
            <span class="header-icon-btn" @click="openProjectUrl">
              <svg class="github-icon" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0016 8c0-4.42-3.58-8-8-8z"/>
              </svg>
            </span>
          </el-tooltip>

          <!-- 全屏切换 -->
          <el-tooltip :content="isFullscreen ? '退出全屏' : '全屏'" placement="bottom">
            <span class="header-icon-btn" @click="toggleFullscreen">
              <svg
                v-if="isFullscreen"
                class="fullscreen-inset-icon"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path d="M10 4V9H5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M14 4V9H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M10 20V15H5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M14 20V15H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
              <el-icon v-else :size="18">
                <FullScreen />
              </el-icon>
            </span>
          </el-tooltip>

          <!-- 深色/浅色切换 -->
          <el-tooltip :content="layoutStore.isDark ? '切换浅色' : '切换深色'" placement="bottom">
            <span class="header-icon-btn" @click="layoutStore.toggleDark()">
              <el-icon :size="18">
                <Moon v-if="!layoutStore.isDark" />
                <Sunny v-else />
              </el-icon>
            </span>
          </el-tooltip>

          <!-- 布局设置 -->
          <el-tooltip content="布局设置" placement="bottom">
            <span class="header-icon-btn" @click="showSetting = true">
              <el-icon :size="18"><Setting /></el-icon>
            </span>
          </el-tooltip>

          <!-- 用户下拉 -->
          <el-dropdown trigger="click" @command="handleCommand">
            <div class="user-trigger">
              <el-avatar :size="30" class="header-avatar">
                <img v-if="headerAvatar" :src="headerAvatar" alt="avatar" />
                <el-icon v-else><User /></el-icon>
              </el-avatar>
              <span class="user-name">{{ userStore.userName || '管理员' }}</span>
            </div>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="profile" :icon="UserFilled">
                  个人中心
                </el-dropdown-item>
                <el-dropdown-item command="logout" :icon="SwitchButton">
                  退出登录
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <!-- ========= 主内容区（可能包含右侧侧边栏） ========= -->
      <el-container v-if="isRight && hasSidebar" class="admin-body">
        <el-main class="admin-main">
          <slot />
        </el-main>

        <!-- 右侧侧边栏 -->
        <el-aside :width="isCollapsed ? '64px' : '230px'" class="admin-aside">
          <div class="admin-logo" :class="{ collapsed: isCollapsed }">
            <img src="/rulo.ico" class="logo-icon" alt="logo" />
            <span v-show="!isCollapsed" class="logo-text">Rulo Admin</span>
          </div>

          <!-- 非混合：完整菜单 -->
          <el-menu
            v-if="!isMixMode"
            :default-active="activeMenu"
            :collapse="isCollapsed"
            :collapse-transition="false"
            unique-opened
            router
            :mode="sidebarMenuMode"
            class="admin-menu"
          >
            <el-menu-item index="/dashboard">
              <el-icon><Odometer /></el-icon>
              <template #title>首页</template>
            </el-menu-item>
            <template v-for="item in visibleMenus" :key="item.id">
              <el-sub-menu v-if="item.children?.length" :index="item.path ?? ''">
                <template #title>
                  <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
                  <span>{{ item.name }}</span>
                </template>
                <template v-for="child in item.children" :key="child.id">
                  <el-menu-item
                    v-if="!isExternal(child.path)"
                    :index="child.path ?? ''"
                  >
                    <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                    <template #title>{{ child.name }}</template>
                  </el-menu-item>
                  <el-menu-item v-else @click="handleMenuClick(child)">
                    <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                    <template #title>{{ child.name }}</template>
                  </el-menu-item>
                </template>
              </el-sub-menu>
              <el-menu-item
                v-else-if="!isExternal(item.path)"
                :index="item.path ?? ''"
              >
                <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
                <template #title>{{ item.name }}</template>
              </el-menu-item>
              <el-menu-item v-else @click="handleMenuClick(item)">
                <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
                <template #title>{{ item.name }}</template>
              </el-menu-item>
            </template>
          </el-menu>

          <!-- 混合：只显示子菜单 -->
          <el-menu
            v-else
            :default-active="activeMenu"
            :collapse="isCollapsed"
            :collapse-transition="false"
            unique-opened
            router
            :mode="sidebarMenuMode"
            class="admin-menu"
          >
            <template v-if="activeTopKey === '/dashboard'">
              <el-menu-item index="/dashboard">
                <el-icon><Odometer /></el-icon>
                <template #title>首页</template>
              </el-menu-item>
            </template>
            <template v-else>
              <template v-for="child in mixSidebarMenus" :key="child.id">
                <el-menu-item
                  v-if="!isExternal(child.path)"
                  :index="child.path ?? ''"
                >
                  <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                  <template #title>{{ child.name }}</template>
                </el-menu-item>
                <el-menu-item v-else @click="handleMenuClick(child)">
                  <el-icon><component :is="iconMap[child.icon ?? '']" /></el-icon>
                  <template #title>{{ child.name }}</template>
                </el-menu-item>
              </template>
              <template v-if="!mixSidebarMenus.length">
                <el-menu-item
                  v-for="item in visibleMenus.filter(m => m.path === activeTopKey && !m.children?.length)"
                  :key="item.id"
                  :index="item.path ?? ''"
                >
                  <el-icon><component :is="iconMap[item.icon ?? '']" /></el-icon>
                  <template #title>{{ item.name }}</template>
                </el-menu-item>
              </template>
            </template>
          </el-menu>
        </el-aside>
      </el-container>

      <!-- 无右侧栏时直接展示 main -->
      <el-main v-else class="admin-main">
        <slot />
      </el-main>
    </el-container>
  </el-container>

  <!-- 布局设置抽屉 -->
  <LayoutSetting v-model="showSetting" />
</template>

<style scoped>
.admin-container {
  height: 100vh;
  overflow: hidden;
}

/* ── 侧边栏 ── */
.admin-aside {
  background: var(--rulo-sidebar-bg);
  border-right: 1px solid var(--rulo-sidebar-border);
  transition: width 0.28s ease, background 0.3s ease;
  overflow: hidden;
  flex-shrink: 0;
}

.admin-logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  color: var(--rulo-sidebar-logo-color);
  font-size: 18px;
  font-weight: 700;
  letter-spacing: 1px;
  border-bottom: 1px solid var(--rulo-sidebar-border);
  white-space: nowrap;
  overflow: hidden;
  transition: padding 0.28s ease, color 0.3s ease;
  padding: 0 20px;
}

.logo-icon {
  width: 34px;
  height: 34px;
  flex-shrink: 0;
  object-fit: contain;
}

.logo-text {
  transition: opacity 0.2s ease;
}

.admin-menu {
  border-right: none;
  background: var(--rulo-sidebar-bg);
  --el-menu-text-color: var(--rulo-sidebar-text);
  --el-menu-hover-text-color: var(--rulo-sidebar-text-hover);
  --el-menu-active-color: var(--rulo-sidebar-text-active);
  --el-menu-bg-color: var(--rulo-sidebar-bg);
  --el-menu-hover-bg-color: var(--rulo-sidebar-hover-bg);
  --el-menu-item-height: 52px;
  transition: background 0.3s ease;
}

/* ── 右侧区域 ── */
.admin-right {
  flex-direction: column;
  overflow: hidden;
}

/* 右侧布局：主容器横向排列（main + right aside） */
.admin-body {
  flex: 1;
  overflow: hidden;
}

.admin-header {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--rulo-header-bg);
  border-bottom: 1px solid var(--rulo-header-border);
  padding: 0 20px;
  box-shadow: 0 1px 4px var(--rulo-header-shadow);
  flex-shrink: 0;
  transition: background 0.3s ease, border-color 0.3s ease;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
  flex: 1;
  min-width: 0;
}

.collapse-btn {
  cursor: pointer;
  color: var(--rulo-header-icon);
  transition: color 0.2s;
  flex-shrink: 0;
}
.collapse-btn:hover {
  color: var(--rulo-primary);
}

/* ── 顶部 logo（纯顶部模式） ── */
.top-logo {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  margin-right: 8px;
}

.logo-text-dark {
  font-size: 16px;
  font-weight: 700;
  color: var(--rulo-text-primary);
  letter-spacing: 1px;
}

/* ── 顶部菜单 ── */
.top-menu {
  border-bottom: none !important;
  flex: 1;
  min-width: 0;
  --el-menu-item-height: 60px;
  --el-menu-bg-color: var(--rulo-header-bg);
  --el-menu-text-color: var(--rulo-header-text);
  --el-menu-hover-text-color: var(--rulo-text-primary);
  --el-menu-active-color: var(--rulo-primary);
  --el-menu-hover-bg-color: var(--rulo-header-icon-hover-bg);
}

.top-menu :deep(.el-menu-item),
.top-menu :deep(.el-sub-menu__title) {
  border-bottom: 2px solid transparent !important;
}

.top-menu :deep(.el-menu-item.is-active) {
  border-bottom: 2px solid var(--rulo-primary) !important;
}

/* ── Header 右侧区域 ── */
.header-right {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
}

.header-icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  cursor: pointer;
  color: var(--rulo-header-icon);
  transition: all 0.2s;
}

.header-icon-btn:hover {
  background: var(--rulo-header-icon-hover-bg);
  color: var(--rulo-primary);
}

.github-icon {
  width: 18px;
  height: 18px;
}

.fullscreen-inset-icon {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.header-avatar {
  background: linear-gradient(135deg, var(--rulo-primary), color-mix(in srgb, var(--rulo-primary) 68%, #ffffff 32%));
  flex-shrink: 0;
}

.user-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 4px 10px;
  border-radius: 8px;
  transition: background 0.2s;
}
.user-trigger:hover {
  background: var(--rulo-header-icon-hover-bg);
}

.user-name {
  font-size: 14px;
  color: var(--rulo-header-text);
}

.admin-main {
  background: var(--rulo-main-bg);
  padding: 20px;
  overflow-y: auto;
  transition: background 0.3s ease;
}
</style>
