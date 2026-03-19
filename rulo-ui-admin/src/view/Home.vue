<script setup lang="ts" name="Home">
import { TrendCharts, User, Document, Calendar, ChatDotRound, Star, Warning } from '@element-plus/icons-vue'
import { useUserStore } from '@/store/user'
import { ref } from 'vue'

const userStore = useUserStore()

const stats = [
  { label: '今日访问', value: '1,234', icon: TrendCharts, color: '#409eff', trend: '+12%' },
  { label: '注册用户', value: '5,678', icon: User,         color: '#67c23a', trend: '+5.3%' },
  { label: '文章总数', value: '256',   icon: Document,      color: '#e6a23c', trend: '+8' },
  { label: '待处理工单', value: '13',  icon: ChatDotRound,  color: '#f56c6c', trend: '-2' },
]

// 快捷入口（perm 为空则无需权限）
const shortcuts = [
  { name: '用户管理', icon: User, path: '/system/user', color: '#409eff', perm: 'sys:user:list' },
  { name: '角色管理', icon: Star, path: '/system/role', color: '#67c23a', perm: 'sys:role:list' },
  { name: '菜单管理', icon: Document, path: '/system/menu', color: '#e6a23c', perm: 'sys:menu:list' },
  { name: '更新日志', icon: TrendCharts, path: '/changelog', color: '#36A98F', perm: '' },
]

// 最近操作日志
const recentLogs = ref([
  { user: 'litchi', action: '修改了用户 lzx333 的邮箱', time: '5 分钟前', type: 'primary' },
  { user: 'lzx111', action: '新增了角色「运营管理员」', time: '23 分钟前', type: 'success' },
  { user: 'litchi', action: '删除了菜单「测试菜单」', time: '1 小时前', type: 'danger' },
  { user: 'lzx222', action: '登录了系统', time: '2 小时前', type: 'info' },
  { user: 'litchi', action: '更新了权限「article:publish」', time: '3 小时前', type: 'warning' },
  { user: 'lzx444', action: '注册了新账号', time: '5 小时前', type: 'success' },
])

// 待办事项
const todos = ref([
  { id: 1, text: '完善动态路由权限控制', done: false },
  { id: 2, text: '完成角色-菜单关联接口', done: false },
  { id: 3, text: '优化暗色模式下的表格样式', done: true },
  { id: 4, text: '接入文章管理模块', done: false },
  { id: 5, text: '编写接口文档', done: false },
])

// 系统信息
const sysInfo = [
  { label: '前端框架', value: 'Vue 3.5 + TypeScript' },
  { label: 'UI 组件库', value: 'Element Plus 2.10' },
  { label: '构建工具', value: 'Vite 7.x' },
  { label: '后端框架', value: 'Axum + Tokio (Rust)' },
  { label: '数据库', value: 'PostgreSQL 16' },
  { label: '缓存', value: 'Redis 7' },
]

// 访问趋势（最近7天假数据）
const visitDays = ['周一', '周二', '周三', '周四', '周五', '周六', '周日']
const visitData = [820, 932, 901, 1234, 1090, 1330, 1120]
const maxVisit = Math.max(...visitData)

// 内容占比
const contentParts = [
  { label: '技术文章', value: 128, color: '#409eff' },
  { label: '生活随笔', value: 56, color: '#67c23a' },
  { label: '项目记录', value: 42, color: '#e6a23c' },
  { label: '学习笔记', value: 30, color: '#f56c6c' },
]
const contentTotal = contentParts.reduce((a, b) => a + b.value, 0)
</script>

<template>
  <div class="dashboard">
    <!-- 欢迎卡片 -->
    <div class="welcome-card">
      <div class="welcome-left">
        <h2>欢迎回来，{{ userStore.userName || '管理员' }} 👋</h2>
        <p>
          今天是
          {{ new Date().toLocaleDateString('zh-CN', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' }) }}，祝你工作愉快！
        </p>
      </div>
      <div class="welcome-right">
        <div class="quick-entry" v-for="s in shortcuts" :key="s.name" v-show="!s.perm || userStore.hasPerm(s.perm)">
          <router-link :to="s.path" class="quick-link">
            <el-icon :size="20" :style="{ color: s.color }"><component :is="s.icon" /></el-icon>
            <span>{{ s.name }}</span>
          </router-link>
        </div>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stat-grid">
      <div v-for="stat in stats" :key="stat.label" class="stat-card">
        <div class="stat-icon" :style="{ background: stat.color + '18', color: stat.color }">
          <el-icon :size="28"><component :is="stat.icon" /></el-icon>
        </div>
        <div class="stat-info">
          <div class="stat-value">{{ stat.value }}</div>
          <div class="stat-label">
            {{ stat.label }}
            <span class="stat-trend" :class="stat.trend.startsWith('+') ? 'up' : 'down'">{{ stat.trend }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 中间区域：访问趋势 + 内容占比 -->
    <div class="mid-grid">
      <!-- 访问趋势（纯 CSS 柱状图） -->
      <el-card shadow="never" class="chart-card">
        <template #header>
          <div class="section-header">
            <span class="section-title">访问趋势</span>
            <span class="section-sub">最近 7 天</span>
          </div>
        </template>
        <div class="bar-chart">
          <div v-for="(v, i) in visitData" :key="i" class="bar-col">
            <div class="bar-value">{{ v }}</div>
            <div class="bar-track">
              <div class="bar-fill" :style="{ height: (v / maxVisit * 100) + '%' }" />
            </div>
            <div class="bar-label">{{ visitDays[i] }}</div>
          </div>
        </div>
      </el-card>

      <!-- 内容占比 -->
      <el-card shadow="never" class="chart-card">
        <template #header>
          <div class="section-header">
            <span class="section-title">内容分布</span>
            <span class="section-sub">共 {{ contentTotal }} 篇</span>
          </div>
        </template>
        <div class="content-list">
          <div v-for="p in contentParts" :key="p.label" class="content-item">
            <div class="content-label">
              <span class="content-dot" :style="{ background: p.color }" />
              <span>{{ p.label }}</span>
            </div>
            <div class="content-bar-wrap">
              <div class="content-bar" :style="{ width: (p.value / contentTotal * 100) + '%', background: p.color }" />
            </div>
            <span class="content-num">{{ p.value }}</span>
          </div>
        </div>
      </el-card>
    </div>

    <!-- 下方区域：操作日志 + 待办 + 系统信息 -->
    <div class="bottom-grid">
      <!-- 最近操作 -->
      <el-card shadow="never">
        <template #header>
          <div class="section-header">
            <span class="section-title">最近操作</span>
          </div>
        </template>
        <div class="log-list">
          <div v-for="(log, i) in recentLogs" :key="i" class="log-item">
            <el-tag :type="log.type as any" size="small" effect="plain" round>{{ log.user }}</el-tag>
            <span class="log-action">{{ log.action }}</span>
            <span class="log-time">{{ log.time }}</span>
          </div>
        </div>
      </el-card>

      <!-- 待办事项 -->
      <el-card shadow="never">
        <template #header>
          <div class="section-header">
            <span class="section-title">待办事项</span>
            <el-tag type="danger" size="small" round effect="dark">{{ todos.filter(t => !t.done).length }} 项待完成</el-tag>
          </div>
        </template>
        <div class="todo-list">
          <div v-for="todo in todos" :key="todo.id" class="todo-item">
            <el-checkbox v-model="todo.done" />
            <span class="todo-text" :class="{ done: todo.done }">{{ todo.text }}</span>
          </div>
        </div>
      </el-card>

      <!-- 系统信息 -->
      <el-card shadow="never">
        <template #header>
          <div class="section-header">
            <span class="section-title">技术栈</span>
          </div>
        </template>
        <div class="sys-list">
          <div v-for="s in sysInfo" :key="s.label" class="sys-item">
            <span class="sys-label">{{ s.label }}</span>
            <span class="sys-value">{{ s.value }}</span>
          </div>
        </div>
      </el-card>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* ── 欢迎卡片 ── */
.welcome-card {
  background: linear-gradient(135deg, var(--rulo-primary), #2d8a75);
  border-radius: 12px;
  padding: 28px 32px;
  color: #fff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 24px;
}
.welcome-left h2 {
  font-size: 22px;
  font-weight: 600;
  margin-bottom: 6px;
}
.welcome-left p {
  font-size: 14px;
  opacity: 0.85;
}
.welcome-right {
  display: flex;
  gap: 12px;
  flex-shrink: 0;
}
.quick-link {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 10px 14px;
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.18);
  backdrop-filter: blur(4px);
  color: #fff;
  text-decoration: none;
  font-size: 12px;
  transition: background 0.2s;
}
.quick-link:hover {
  background: rgba(255, 255, 255, 0.3);
}
.quick-link .el-icon {
  color: #fff !important;
}

/* ── 统计卡片 ── */
.stat-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}
@media (max-width: 1200px) {
  .stat-grid { grid-template-columns: repeat(2, 1fr); }
}

.stat-card {
  background: var(--el-bg-color);
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 12px;
  padding: 22px 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  transition: box-shadow 0.2s;
}
.stat-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
}
.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--el-text-color-primary);
  line-height: 1.2;
}
.stat-label {
  font-size: 13px;
  color: var(--el-text-color-secondary);
  margin-top: 4px;
  display: flex;
  align-items: center;
  gap: 6px;
}
.stat-trend {
  font-size: 12px;
  font-weight: 600;
}
.stat-trend.up { color: #67c23a; }
.stat-trend.down { color: #f56c6c; }

/* ── 中间区域 ── */
.mid-grid {
  display: grid;
  grid-template-columns: 1.5fr 1fr;
  gap: 16px;
}
@media (max-width: 1000px) {
  .mid-grid { grid-template-columns: 1fr; }
}

.chart-card {
  border-radius: 12px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.section-title {
  font-size: 15px;
  font-weight: 600;
}
.section-sub {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

/* ── 柱状图 ── */
.bar-chart {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  height: 200px;
  gap: 8px;
  padding-top: 8px;
}
.bar-col {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  height: 100%;
}
.bar-value {
  font-size: 11px;
  color: var(--el-text-color-secondary);
  flex-shrink: 0;
}
.bar-track {
  flex: 1;
  width: 100%;
  max-width: 40px;
  background: var(--el-fill-color-light);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  overflow: hidden;
}
.bar-fill {
  background: linear-gradient(180deg, var(--rulo-primary), #2d8a75);
  border-radius: 6px;
  transition: height 0.6s ease;
  min-height: 4px;
}
.bar-label {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  flex-shrink: 0;
}

/* ── 内容分布 ── */
.content-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 8px 0;
}
.content-item {
  display: flex;
  align-items: center;
  gap: 12px;
}
.content-label {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 90px;
  font-size: 13px;
  color: var(--el-text-color-regular);
  flex-shrink: 0;
}
.content-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}
.content-bar-wrap {
  flex: 1;
  height: 8px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  overflow: hidden;
}
.content-bar {
  height: 100%;
  border-radius: 4px;
  transition: width 0.6s ease;
}
.content-num {
  font-size: 13px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  width: 36px;
  text-align: right;
}

/* ── 下方三列 ── */
.bottom-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 16px;
}
@media (max-width: 1200px) {
  .bottom-grid { grid-template-columns: 1fr; }
}

.bottom-grid :deep(.el-card) {
  border-radius: 12px;
}

/* ── 操作日志 ── */
.log-list {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.log-item {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 13px;
}
.log-action {
  flex: 1;
  color: var(--el-text-color-regular);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.log-time {
  color: var(--el-text-color-secondary);
  font-size: 12px;
  flex-shrink: 0;
}

/* ── 待办 ── */
.todo-list {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.todo-item {
  display: flex;
  align-items: center;
  gap: 10px;
}
.todo-text {
  font-size: 13px;
  color: var(--el-text-color-regular);
}
.todo-text.done {
  text-decoration: line-through;
  color: var(--el-text-color-secondary);
}

/* ── 系统信息 ── */
.sys-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.sys-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
}
.sys-label {
  color: var(--el-text-color-secondary);
}
.sys-value {
  color: var(--el-text-color-primary);
  font-weight: 500;
}
</style>
