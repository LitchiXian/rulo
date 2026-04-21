<script setup lang="ts" name="Changelog">
const logs = [
  {
    version: 'v0.5.0',
    date: '2026-03-17',
    tag: '主题与布局',
    items: [
      '新增 5 种布局模式：左侧菜单、顶部菜单、右侧菜单、左侧顶部混合、右侧顶部混合',
      '新增深色 / 浅色 / 跟随系统三种主题模式切换',
      '基于 CSS 变量实现全站主题系统，统一覆盖 Element Plus 暗色变量',
      '新增布局设置抽屉（LayoutSetting），支持布局与主题选择',
      '新增全屏切换功能',
      '确立 Rulo 品牌色 #36A98F（青绿色），暗色模式采用中性深灰色系',
    ],
  },
  {
    version: 'v0.4.0',
    date: '2026-03-15',
    tag: '管理前端',
    items: [
      '搭建 rulo-ui-admin 前端项目（Vue 3 + TypeScript + Vite + Element Plus）',
      '实现登录页 / 首页 / 404 页面',
      '对接后端登录、登出、用户信息接口',
      '新增 NProgress 路由进度条与动态页面标题',
      '新增 Vite 开发代理，通过 config.json 配置后端地址',
      '新增系统管理模块：用户、角色、菜单、权限的 CRUD 页面',
    ],
  },
  {
    version: 'v0.3.0',
    date: '2026-03-12',
    tag: '系统管理',
    items: [
      '新增 sys_user / sys_role / sys_menu / sys_permission 增删改查接口',
      '新增 RBAC 数据库迁移（角色权限、角色菜单关联表）',
      '使用 sqlx 宏实现编译期 SQL 检查',
      '使用 sqlx migrate 管理数据库版本',
      '代码结构拆分：handler / service / model 分层',
    ],
  },
  {
    version: 'v0.2.0',
    date: '2026-03-05',
    tag: '认证鉴权',
    items: [
      '实现 JWT 认证中间件，区分公开路由与受保护路由',
      '使用 argon2 加密用户密码',
      '使用 jsonwebtoken 签发与校验 Token',
      '新增 Redis Token 存储，支持登出时令牌失效',
      '新增用户注册与登录接口',
      '后端添加 CORS 支持',
    ],
  },
  {
    version: 'v0.1.0',
    date: '2026-02-28',
    tag: '项目初始化',
    items: [
      '初始化 Rust 后端项目（Axum + Tokio），Cargo Workspace 结构',
      '搭建 app（应用入口）与 rulo-common（公共模块）',
      '接入 PostgreSQL 连接池（sqlx）',
      '接入 Redis（bb8-redis）',
      '实现统一响应体 ApiResult 与统一错误处理',
      '配置文件日志与控制台日志输出',
    ],
  },
]

const tagColor = (tag: string) => {
  const map: Record<string, string> = {
    '主题与布局': '#36A98F',
    '管理前端': '#409eff',
    '系统管理': '#e6a23c',
    '认证鉴权': '#f56c6c',
    '项目初始化': '#909399',
  }
  return map[tag] || '#909399'
}
</script>

<template>
  <div class="changelog-page">
    <el-card shadow="never">
      <template #header>
        <div class="card-header">
          <span class="card-title">更新日志</span>
          <span class="card-desc">Rulo 项目各版本更新记录</span>
        </div>
      </template>

      <el-timeline>
        <el-timeline-item
          v-for="log in logs"
          :key="log.version"
          :timestamp="log.date"
          placement="top"
          :color="tagColor(log.tag)"
        >
          <div class="log-header">
            <span class="log-version">{{ log.version }}</span>
            <el-tag :color="tagColor(log.tag)" effect="dark" size="small" style="border: none; color: #fff;">
              {{ log.tag }}
            </el-tag>
          </div>
          <ul class="log-list">
            <li v-for="(item, i) in log.items" :key="i">{{ item }}</li>
          </ul>
        </el-timeline-item>
      </el-timeline>
    </el-card>
  </div>
</template>

<style scoped>
.changelog-page {
  max-width: 900px;
}

.card-header {
  display: flex;
  align-items: baseline;
  gap: 12px;
}

.card-title {
  font-size: 18px;
  font-weight: 600;
}

.card-desc {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.log-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
}

.log-version {
  font-size: 16px;
  font-weight: 600;
}

.log-list {
  margin: 0;
  padding-left: 18px;
  line-height: 2;
  color: var(--el-text-color-regular);
  font-size: 14px;
}
</style>
