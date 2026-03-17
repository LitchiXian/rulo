<script setup lang="ts" name="About">
</script>

<template>
  <div class="about-container">
    <el-card shadow="hover">
      <template #header>
        <div class="card-header">
          <img src="/rulo.ico" class="about-logo" alt="logo" />
          <span class="about-title">Rulo Admin</span>
        </div>
      </template>

      <el-descriptions :column="1" border>
        <el-descriptions-item label="项目名称">Rulo</el-descriptions-item>
        <el-descriptions-item label="项目描述">基于 Vue 3 + Element Plus + Rust (Axum) 的后台管理系统</el-descriptions-item>
        <el-descriptions-item label="前端技术栈">Vue 3.5 / TypeScript / Vite 7 / Element Plus / Pinia</el-descriptions-item>
        <el-descriptions-item label="后端技术栈">Rust / Axum / PostgreSQL / Redis</el-descriptions-item>
        <el-descriptions-item label="项目地址">
          <el-link type="primary" href="https://github.com/LitchiXian/rulo" target="_blank">
            https://github.com/LitchiXian/rulo
          </el-link>
        </el-descriptions-item>
      </el-descriptions>
    </el-card>
  </div>
</template>

<style scoped>
.about-container {
  max-width: 700px;
  margin: 0 auto;
  padding: 20px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.about-logo {
  width: 36px;
  height: 36px;
  object-fit: contain;
}

.about-title {
  font-size: 22px;
  font-weight: 700;
  color: var(--rulo-text-primary);
}
</style>
