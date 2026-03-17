<script setup lang="ts" name="AdminLayout">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Fold, Expand, Odometer, User, SwitchButton } from '@element-plus/icons-vue'
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
</script>

<template>
  <el-container class="admin-container">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapsed ? '64px' : '220px'" class="admin-aside">
      <div class="admin-logo" :class="{ collapsed: isCollapsed }">
        <span v-show="!isCollapsed">Rulo Admin</span>
        <el-icon v-show="isCollapsed" :size="22"><Odometer /></el-icon>
      </div>

      <el-menu
        :default-active="activeMenu"
        :collapse="isCollapsed"
        :collapse-transition="false"
        router
        class="admin-menu"
      >
        <el-menu-item index="/dashboard">
          <el-icon><Odometer /></el-icon>
          <template #title>首页</template>
        </el-menu-item>
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
  color: #f1f5f9;
  font-size: 18px;
  font-weight: 700;
  letter-spacing: 1px;
  border-bottom: 1px solid #2d3f55;
  white-space: nowrap;
  overflow: hidden;
  transition: padding 0.28s ease;
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
