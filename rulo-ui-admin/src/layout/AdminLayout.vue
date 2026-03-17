<script setup lang="ts" name="AdminLayout">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Fold, Expand, Odometer, User, SwitchButton, Setting, UserFilled, Key, Menu as MenuIcon, Lock, MoreFilled, InfoFilled, Link } from '@element-plus/icons-vue'

import { useUserStore } from '@/store/user'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

const isCollapsed = ref(false)
const toggleCollapse = () => (isCollapsed.value = !isCollapsed.value)

// 当前高亮菜单项
const activeMenu = computed(() => route.path)

const handleCommand = async (cmd: string) => {
  if (cmd === 'logout') {
    await userStore.logout()
    router.push('/login')
  }
}

const openProjectUrl = () => {
  window.open('https://github.com/LitchiXian/rulo', '_blank')
}
</script>

<template>
  <el-container class="admin-container">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapsed ? '64px' : '220px'" class="admin-aside">
      <div class="admin-logo" :class="{ collapsed: isCollapsed }">
        <img src="/rulo.ico" class="logo-icon" alt="logo" />
        <span v-show="!isCollapsed" class="logo-text">Rulo Admin</span>
      </div>

      <el-menu
        :default-active="activeMenu"
        :collapse="isCollapsed"
        :collapse-transition="false"
        unique-opened
        router
        class="admin-menu"
      >
        <el-menu-item index="/dashboard">
          <el-icon><Odometer /></el-icon>
          <template #title>首页</template>
        </el-menu-item>

        <!-- TODO: 侧边栏菜单目前是写死的，后续改为动态渲染：
             从后端获取当前用户的菜单权限数据，递归生成 el-sub-menu / el-menu-item -->
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
    </el-aside>

    <el-container class="admin-right">
      <!-- 顶栏 -->
      <el-header class="admin-header">
        <div class="header-left">
          <el-icon class="collapse-btn" :size="20" @click="toggleCollapse">
            <Fold v-if="!isCollapsed" />
            <Expand v-else />
          </el-icon>
          <el-breadcrumb separator="/">
            <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
          </el-breadcrumb>
        </div>

        <div class="header-right">
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

      <!-- 主内容区域 -->
      <el-main class="admin-main">
        <slot />
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.admin-container {
  height: 100vh;
  overflow: hidden;
}

/* ── 侧边栏 ── */
.admin-aside {
  background: #1e293b;
  transition: width 0.28s ease;
  overflow: hidden;
  flex-shrink: 0;
}

.admin-logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  color: #f1f5f9;
  font-size: 18px;
  font-weight: 700;
  letter-spacing: 1px;
  border-bottom: 1px solid #2d3f55;
  white-space: nowrap;
  overflow: hidden;
  transition: padding 0.28s ease;
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
  background: #1e293b;
  --el-menu-text-color: #94a3b8;
  --el-menu-hover-text-color: #f1f5f9;
  --el-menu-active-color: #60a5fa;
  --el-menu-bg-color: #1e293b;
  --el-menu-hover-bg-color: #263349;
  --el-menu-item-height: 50px;
}

/* ── 右侧区域 ── */
.admin-right {
  flex-direction: column;
  overflow: hidden;
}

.admin-header {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #fff;
  border-bottom: 1px solid #e2e8f0;
  padding: 0 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.06);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.collapse-btn {
  cursor: pointer;
  color: #64748b;
  transition: color 0.2s;
}
.collapse-btn:hover {
  color: #409eff;
}

.header-right {
  display: flex;
  align-items: center;
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
  background: #f1f5f9;
}

.user-name {
  font-size: 14px;
  color: #374151;
}

.admin-main {
  background: #f1f5f9;
  padding: 20px;
  overflow-y: auto;
}
</style>
