<script setup lang="ts" name="AdminLayout">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Fold, Expand, Odometer, User, SwitchButton, Setting, UserFilled, Key, Menu as MenuIcon, Lock, MoreFilled, InfoFilled, Link, Sunny, Moon, FullScreen, ScaleToOriginal } from '@element-plus/icons-vue'

import { useUserStore } from '@/store/user'
import { useLayoutStore } from '@/store/layout'
import LayoutSetting from '@/component/LayoutSetting.vue'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const layoutStore = useLayoutStore()

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

const handleCommand = async (cmd: string) => {
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
onMounted(() => document.addEventListener('fullscreenchange', onFullscreenChange))
onUnmounted(() => document.removeEventListener('fullscreenchange', onFullscreenChange))

// 混合模式：点击顶部一级菜单，切换侧边子菜单
const handleTopMenuSelect = (index: string) => {
  activeTopKey.value = index
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
      :width="isCollapsed ? '64px' : '220px'"
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
        <el-sub-menu index="/system">
          <template #title>
            <el-icon><Setting /></el-icon>
            <span>系统管理</span>
          </template>
          <el-menu-item index="/system/user">
            <el-icon><UserFilled /></el-icon>
            <template #title>用户管理</template>
          </el-menu-item>
          <el-menu-item index="/system/role">
            <el-icon><Key /></el-icon>
            <template #title>角色管理</template>
          </el-menu-item>
          <el-menu-item index="/system/menu">
            <el-icon><MenuIcon /></el-icon>
            <template #title>菜单管理</template>
          </el-menu-item>
          <el-menu-item index="/system/permission">
            <el-icon><Lock /></el-icon>
            <template #title>权限管理</template>
          </el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="/other">
          <template #title>
            <el-icon><MoreFilled /></el-icon>
            <span>其他</span>
          </template>
          <el-menu-item index="/other/about">
            <el-icon><InfoFilled /></el-icon>
            <template #title>关于我们</template>
          </el-menu-item>
          <el-menu-item @click="openProjectUrl">
            <el-icon><Link /></el-icon>
            <template #title>项目地址</template>
          </el-menu-item>
        </el-sub-menu>
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
        <!-- /dashboard 没有子菜单，显示首页本身 -->
        <template v-if="activeTopKey === '/dashboard'">
          <el-menu-item index="/dashboard">
            <el-icon><Odometer /></el-icon>
            <template #title>首页</template>
          </el-menu-item>
        </template>
        <!-- /system 子菜单 -->
        <template v-if="activeTopKey === '/system'">
          <el-menu-item index="/system/user">
            <el-icon><UserFilled /></el-icon>
            <template #title>用户管理</template>
          </el-menu-item>
          <el-menu-item index="/system/role">
            <el-icon><Key /></el-icon>
            <template #title>角色管理</template>
          </el-menu-item>
          <el-menu-item index="/system/menu">
            <el-icon><MenuIcon /></el-icon>
            <template #title>菜单管理</template>
          </el-menu-item>
          <el-menu-item index="/system/permission">
            <el-icon><Lock /></el-icon>
            <template #title>权限管理</template>
          </el-menu-item>
        </template>
        <!-- /other 子菜单 -->
        <template v-if="activeTopKey === '/other'">
          <el-menu-item index="/other/about">
            <el-icon><InfoFilled /></el-icon>
            <template #title>关于我们</template>
          </el-menu-item>
          <el-menu-item @click="openProjectUrl">
            <el-icon><Link /></el-icon>
            <template #title>项目地址</template>
          </el-menu-item>
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
            <el-menu-item index="/system">
              <el-icon><Setting /></el-icon>
              <span>系统管理</span>
            </el-menu-item>
            <el-menu-item index="/other">
              <el-icon><MoreFilled /></el-icon>
              <span>其他</span>
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
            <el-sub-menu index="/system">
              <template #title>
                <el-icon><Setting /></el-icon>
                <span>系统管理</span>
              </template>
              <el-menu-item index="/system/user">
                <el-icon><UserFilled /></el-icon>
                <span>用户管理</span>
              </el-menu-item>
              <el-menu-item index="/system/role">
                <el-icon><Key /></el-icon>
                <span>角色管理</span>
              </el-menu-item>
              <el-menu-item index="/system/menu">
                <el-icon><MenuIcon /></el-icon>
                <span>菜单管理</span>
              </el-menu-item>
              <el-menu-item index="/system/permission">
                <el-icon><Lock /></el-icon>
                <span>权限管理</span>
              </el-menu-item>
            </el-sub-menu>
            <el-sub-menu index="/other">
              <template #title>
                <el-icon><MoreFilled /></el-icon>
                <span>其他</span>
              </template>
              <el-menu-item index="/other/about">
                <el-icon><InfoFilled /></el-icon>
                <span>关于我们</span>
              </el-menu-item>
              <el-menu-item @click="openProjectUrl">
                <el-icon><Link /></el-icon>
                <span>项目地址</span>
              </el-menu-item>
            </el-sub-menu>
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
              <el-icon :size="18">
                <ScaleToOriginal v-if="isFullscreen" />
                <FullScreen v-else />
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
              <el-avatar :size="30" style="background: #409eff; flex-shrink: 0;">
                <el-icon><User /></el-icon>
              </el-avatar>
              <span class="user-name">{{ userStore.userName || '管理员' }}</span>
            </div>
            <template #dropdown>
              <el-dropdown-menu>
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
        <el-aside :width="isCollapsed ? '64px' : '220px'" class="admin-aside">
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
            <el-sub-menu index="/system">
              <template #title>
                <el-icon><Setting /></el-icon>
                <span>系统管理</span>
              </template>
              <el-menu-item index="/system/user">
                <el-icon><UserFilled /></el-icon>
                <template #title>用户管理</template>
              </el-menu-item>
              <el-menu-item index="/system/role">
                <el-icon><Key /></el-icon>
                <template #title>角色管理</template>
              </el-menu-item>
              <el-menu-item index="/system/menu">
                <el-icon><MenuIcon /></el-icon>
                <template #title>菜单管理</template>
              </el-menu-item>
              <el-menu-item index="/system/permission">
                <el-icon><Lock /></el-icon>
                <template #title>权限管理</template>
              </el-menu-item>
            </el-sub-menu>
            <el-sub-menu index="/other">
              <template #title>
                <el-icon><MoreFilled /></el-icon>
                <span>其他</span>
              </template>
              <el-menu-item index="/other/about">
                <el-icon><InfoFilled /></el-icon>
                <template #title>关于我们</template>
              </el-menu-item>
              <el-menu-item @click="openProjectUrl">
                <el-icon><Link /></el-icon>
                <template #title>项目地址</template>
              </el-menu-item>
            </el-sub-menu>
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
            <template v-if="activeTopKey === '/system'">
              <el-menu-item index="/system/user">
                <el-icon><UserFilled /></el-icon>
                <template #title>用户管理</template>
              </el-menu-item>
              <el-menu-item index="/system/role">
                <el-icon><Key /></el-icon>
                <template #title>角色管理</template>
              </el-menu-item>
              <el-menu-item index="/system/menu">
                <el-icon><MenuIcon /></el-icon>
                <template #title>菜单管理</template>
              </el-menu-item>
              <el-menu-item index="/system/permission">
                <el-icon><Lock /></el-icon>
                <template #title>权限管理</template>
              </el-menu-item>
            </template>
            <template v-if="activeTopKey === '/other'">
              <el-menu-item index="/other/about">
                <el-icon><InfoFilled /></el-icon>
                <template #title>关于我们</template>
              </el-menu-item>
              <el-menu-item @click="openProjectUrl">
                <el-icon><Link /></el-icon>
                <template #title>项目地址</template>
              </el-menu-item>
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
  padding: 0 16px;
}

.logo-icon {
  width: 28px;
  height: 28px;
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
  --el-menu-item-height: 50px;
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
