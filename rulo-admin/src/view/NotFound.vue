<script setup lang="ts" name="NotFound">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="not-found">
    <div class="not-found-code">404</div>
    <p class="not-found-msg">页面不存在</p>
    <el-button type="primary" @click="router.push('/dashboard')">返回首页</el-button>
  </div>
</template>

<style scoped>
.not-found {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  background: #f1f5f9;
}
.not-found-code {
  font-size: 96px;
  font-weight: 700;
  color: #e2e8f0;
  line-height: 1;
}
.not-found-msg {
  font-size: 16px;
  color: #94a3b8;
}
</style>
