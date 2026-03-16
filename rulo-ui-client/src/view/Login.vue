<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { Hide, View } from '@element-plus/icons-vue';
import { useUserStore } from "@/store/user.ts";
import { showMessage } from "@/util/message.ts";

// 响应式状态
const email = ref('');
const password = ref('');
const isPasswordVisible = ref(false);
const rememberMe = ref(false);
const loading = ref(false);

const router = useRouter();
const route = useRoute();

// 切换密码可见性
const togglePasswordVisibility = () => {
  isPasswordVisible.value = !isPasswordVisible.value;
};

// 表单提交
const handleSubmit = async (e: Event) => {
  e.preventDefault();

  if (!email.value || !password.value) {
    showMessage('请填写账号和密码', 'warning');
    return;
  }

  loading.value = true;

  try {
    const userStore = useUserStore();
    await userStore.login({
      userName: email.value,
      password: password.value,
      redirect: route.query.redirect as string
    });
    showMessage('登录成功', 'success');
  } catch (err) {
    console.error('登录失败:', err);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="auth-container">
    <img src="@/asset/image/login-bg.png" alt="背景" class="auth-bg">

    <form @submit="handleSubmit" class="auth-form">
      <h1 class="auth-title">登录</h1>

      <div class="auth-content">
        <!-- 账号输入框 -->
        <div class="auth-input-group">
          <i class="ri-user-3-line auth-icon"></i>
          <div class="auth-input-wrapper">
            <input
              type="text"
              required
              class="auth-input"
              id="login-email"
              placeholder=" "
              v-model="email"
              autocomplete="username"
            >
            <label for="login-email" class="auth-label">账号</label>
          </div>
        </div>

        <!-- 密码输入框 -->
        <div class="auth-input-group">
          <i class="ri-lock-2-line auth-icon"></i>
          <div class="auth-input-wrapper">
            <input
              :type="isPasswordVisible ? 'text' : 'password'"
              required
              class="auth-input"
              id="login-pass"
              placeholder=" "
              v-model="password"
              autocomplete="current-password"
            >
            <label for="login-pass" class="auth-label">密码</label>
            <el-icon class="auth-eye-btn" @click="togglePasswordVisibility">
              <Hide v-if="isPasswordVisible"/>
              <View v-else/>
            </el-icon>
          </div>
        </div>
      </div>

      <div class="auth-check">
        <div class="auth-check-group">
          <input 
            type="checkbox" 
            class="auth-check-input" 
            id="login-remember"
            v-model="rememberMe"
          >
          <label for="login-remember" class="auth-check-label">记住我</label>
        </div>
      </div>

      <button 
        type="submit" 
        class="auth-submit-btn"
        :disabled="loading"
      >
        {{ loading ? '登录中...' : '登录' }}
      </button>

      <p class="auth-switch">
        没有账号?
        <router-link to="/register" class="auth-switch-link">注册</router-link>
      </p>
    </form>
  </div>
</template>

<style scoped>
/* 页面特定样式（如有需要） */
</style>