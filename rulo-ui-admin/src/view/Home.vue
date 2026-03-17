<script setup lang="ts" name="Home">
import { TrendCharts, User, Document, Calendar } from '@element-plus/icons-vue'
import { useUserStore } from '@/store/user'

const userStore = useUserStore()

const stats = [
  { label: '今日访问', value: '1,234', icon: TrendCharts, color: '#409eff' },
  { label: '注册用户', value: '5,678', icon: User,         color: '#67c23a' },
  { label: '文章总数', value: '256',   icon: Document,      color: '#e6a23c' },
  {
    label: '今日日期',
    value: new Date().toLocaleDateString('zh-CN'),
    icon: Calendar,
    color: '#f56c6c',
  },
]
</script>

<template>
  <div class="dashboard">
    <!-- 欢迎卡片 -->
    <div class="welcome-card">
      <h2>欢迎回来，{{ userStore.userName || '管理员' }} 👋</h2>
      <p>
        今天是
        {{
          new Date().toLocaleDateString('zh-CN', {
            weekday: 'long',
            year: 'numeric',
            month: 'long',
            day: 'numeric',
          })
        }}
      </p>
    </div>

    <!-- 统计卡片 -->
    <div class="stat-grid">
      <div v-for="stat in stats" :key="stat.label" class="stat-card">
        <div class="stat-icon" :style="{ background: stat.color + '20', color: stat.color }">
          <el-icon :size="28"><component :is="stat.icon" /></el-icon>
        </div>
        <div class="stat-info">
          <div class="stat-value">{{ stat.value }}</div>
          <div class="stat-label">{{ stat.label }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.welcome-card {
  background: linear-gradient(135deg, #409eff, #1e88e5);
  border-radius: 12px;
  padding: 28px 32px;
  color: #fff;
}
.welcome-card h2 {
  font-size: 22px;
  font-weight: 600;
  margin-bottom: 6px;
}
.welcome-card p {
  font-size: 14px;
  opacity: 0.85;
}

.stat-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 16px;
}

.stat-card {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.06);
  transition: box-shadow 0.2s;
}
.stat-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
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
  font-size: 22px;
  font-weight: 700;
  color: #1e293b;
  line-height: 1.2;
}
.stat-label {
  font-size: 13px;
  color: #94a3b8;
  margin-top: 4px;
}
</style>
