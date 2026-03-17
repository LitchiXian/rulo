<script setup lang="ts">
import { useLayoutStore } from '@/store/layout'
import type { LayoutMode } from '@/store/layout'

const layoutStore = useLayoutStore()

const visible = defineModel<boolean>({ default: false })

const layouts: { mode: LayoutMode; label: string }[] = [
  { mode: 'left', label: '左侧菜单' },
  { mode: 'left-top-mix', label: '左侧顶部混合' },
  { mode: 'top', label: '顶部菜单' },
  { mode: 'right', label: '右侧菜单' },
  { mode: 'right-top-mix', label: '右侧顶部混合' },
]

const selectLayout = (mode: LayoutMode) => {
  layoutStore.setMode(mode)
}
</script>

<template>
  <el-drawer v-model="visible" title="布局设置" size="280px" :with-header="true">
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
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.layout-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 8px;
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
  font-size: 12px;
  color: var(--rulo-text-secondary);
  white-space: nowrap;
}

.layout-item.active .layout-label {
  color: var(--rulo-primary);
  font-weight: 600;
}

/* ── Layout Thumbnails ── */
.layout-thumb {
  width: 80px;
  height: 56px;
  border-radius: 4px;
  overflow: hidden;
  display: flex;
  background: var(--rulo-main-bg);
  border: 1px solid var(--rulo-border);
}

.t-aside {
  width: 20px;
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
  height: 12px;
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
  height: 12px;
  background: var(--rulo-header-bg);
  border-bottom: 1px solid var(--rulo-border);
}

.t-main-full {
  flex: 1;
  background: var(--rulo-main-bg);
}
</style>
