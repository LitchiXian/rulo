<script setup lang="ts">
import { useLayoutStore } from '@/store/layout'
import type { LayoutMode, ThemeMode } from '@/store/layout'
import { Sunny, Moon, Monitor } from '@element-plus/icons-vue'

const layoutStore = useLayoutStore()

const visible = defineModel<boolean>({ default: false })

const layouts: { mode: LayoutMode; label: string }[] = [
  { mode: 'left', label: '左侧菜单' },
  { mode: 'top', label: '顶部菜单' },
  { mode: 'right', label: '右侧菜单' },
  { mode: 'left-top-mix', label: '左侧顶部混合' },
  { mode: 'right-top-mix', label: '右侧顶部混合' },
]

const themes: { mode: ThemeMode; label: string; icon: typeof Sunny }[] = [
  { mode: 'light', label: '浅色', icon: Sunny },
  { mode: 'dark', label: '深色', icon: Moon },
  { mode: 'system', label: '系统', icon: Monitor },
]

const selectLayout = (mode: LayoutMode) => {
  layoutStore.setMode(mode)
}

const selectTheme = (mode: ThemeMode, e: MouseEvent) => {
  layoutStore.setThemeMode(mode, e)
}
</script>

<template>
  <el-drawer v-model="visible" title="布局设置" size="320px" :with-header="true">
    <div class="layout-section">
      <div class="section-title">菜单布局</div>
      <div class="layout-grid">
        <div
          v-for="item in layouts"
          :key="item.mode"
          class="layout-item"
          :class="{ active: layoutStore.mode === item.mode }"
          @click="selectLayout(item.mode)"
        >
          <!-- layout thumbnail -->
          <div class="layout-thumb" :class="'thumb-' + item.mode">
            <!-- left -->
            <template v-if="item.mode === 'left'">
              <div class="t-aside" />
              <div class="t-right">
                <div class="t-header" />
                <div class="t-main" />
              </div>
            </template>
            <!-- left-top-mix -->
            <template v-else-if="item.mode === 'left-top-mix'">
              <div class="t-aside" />
              <div class="t-right">
                <div class="t-header has-menu" />
                <div class="t-main" />
              </div>
            </template>
            <!-- top -->
            <template v-else-if="item.mode === 'top'">
              <div class="t-top-only">
                <div class="t-header-full" />
                <div class="t-main-full" />
              </div>
            </template>
            <!-- right -->
            <template v-else-if="item.mode === 'right'">
              <div class="t-left-area">
                <div class="t-header" />
                <div class="t-main" />
              </div>
              <div class="t-aside" />
            </template>
            <!-- right-top-mix -->
            <template v-else-if="item.mode === 'right-top-mix'">
              <div class="t-left-area">
                <div class="t-header has-menu" />
                <div class="t-main" />
              </div>
              <div class="t-aside" />
            </template>
          </div>
          <span class="layout-label">{{ item.label }}</span>
        </div>
      </div>
    </div>

    <!-- 主题风格 -->
    <div class="layout-section" style="margin-top: 24px;">
      <div class="section-title">主题风格</div>
      <div class="theme-grid">
        <div
          v-for="item in themes"
          :key="item.mode"
          class="theme-item"
          :class="{ active: layoutStore.themeMode === item.mode }"
          @click="selectTheme(item.mode, $event)"
        >
          <el-icon :size="20"><component :is="item.icon" /></el-icon>
          <span>{{ item.label }}</span>
        </div>
      </div>
    </div>
  </el-drawer>
</template>

<style scoped>
.layout-section {
  padding: 0 4px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--rulo-text-primary);
  margin-bottom: 16px;
}

.layout-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 12px;
}

.layout-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  padding: 6px;
  border-radius: 8px;
  border: 2px solid transparent;
  transition: all 0.2s;
}

.layout-item:hover {
  background: var(--rulo-sidebar-hover-bg);
}

.layout-item.active {
  border-color: var(--rulo-primary);
  background: var(--rulo-sidebar-active-bg);
}

.layout-label {
  font-size: 11px;
  color: var(--rulo-text-secondary);
  white-space: nowrap;
}

.layout-item.active .layout-label {
  color: var(--rulo-primary);
  font-weight: 600;
}

/* ── Layout Thumbnails ── */
.layout-thumb {
  width: 68px;
  height: 48px;
  border-radius: 4px;
  overflow: hidden;
  display: flex;
  background: var(--rulo-main-bg);
  border: 1px solid var(--rulo-border);
}

.t-aside {
  width: 16px;
  background: var(--rulo-sidebar-bg);
  border-right: 1px solid var(--rulo-border);
  flex-shrink: 0;
}

.t-right,
.t-left-area {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.t-header {
  height: 10px;
  background: var(--rulo-header-bg);
  border-bottom: 1px solid var(--rulo-border);
}

.t-header.has-menu {
  background: var(--rulo-header-bg);
  position: relative;
}

.t-header.has-menu::after {
  content: '';
  position: absolute;
  top: 4px;
  left: 4px;
  width: 60%;
  height: 4px;
  background: var(--rulo-primary);
  border-radius: 2px;
  opacity: 0.5;
}

.t-main {
  flex: 1;
  background: var(--rulo-main-bg);
}

/* top-only layout */
.t-top-only {
  width: 100%;
  display: flex;
  flex-direction: column;
}

.t-header-full {
  height: 10px;
  background: var(--rulo-header-bg);
  border-bottom: 1px solid var(--rulo-border);
}

.t-main-full {
  flex: 1;
  background: var(--rulo-main-bg);
}

/* ── Theme Style Selector ── */
.theme-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 12px;
}

.theme-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 14px 8px;
  border-radius: 8px;
  border: 2px solid transparent;
  cursor: pointer;
  color: var(--rulo-text-secondary);
  font-size: 13px;
  transition: all 0.2s;
}

.theme-item:hover {
  background: var(--rulo-sidebar-hover-bg);
  color: var(--rulo-text-primary);
}

.theme-item.active {
  border-color: var(--rulo-primary);
  background: var(--rulo-sidebar-active-bg);
  color: var(--rulo-primary);
}
</style>
