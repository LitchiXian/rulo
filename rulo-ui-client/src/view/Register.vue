<script setup lang="ts">
import { ref } from 'vue';
import { Hide, View } from '@element-plus/icons-vue';
import { useRouter } from "vue-router";
import authApi from '@/api/web/auth';
import { showMessage } from '@/util/message';

// 表单数据
const userName = ref('');
const email = ref('');
const password = ref('');
const confirmPassword = ref('');
const agreeTerms = ref(false);

// UI状态
const isPasswordVisible = ref(false);
const isConfirmPasswordVisible = ref(false);
const loading = ref(false);

const router = useRouter();

// 切换密码可见性
const togglePasswordVisibility = () => {
  isPasswordVisible.value = !isPasswordVisible.value;
};

const toggleConfirmPasswordVisibility = () => {
  isConfirmPasswordVisible.value = !isConfirmPasswordVisible.value;
};

// 表单验证
const validateForm = (): boolean => {
  if (!userName.value.trim()) {
    showMessage('请输入账号', 'warning');
    return false;
  }
  if (!email.value.trim()) {
    showMessage('请输入邮箱', 'warning');
    return false;
  }
  if (!password.value) {
    showMessage('请输入密码', 'warning');
    return false;
  }
  if (password.value !== confirmPassword.value) {
    showMessage('两次输入的密码不一致', 'warning');
    return false;
  }
  return true;
};

// 表单提交
const handleSubmit = async (e: Event) => {
  e.preventDefault();

  if (!validateForm()) return;

  loading.value = true;

  try {
    await authApi.register({
      username: userName.value,
      email: email.value,
      password: password.value,
    });

    showMessage('注册成功', 'success');
    router.push('/login');
  } catch (err) {
    console.error('注册失败:', err);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="auth-container">
    <img src="@/asset/image/login-bg.png" alt="背景" class="auth-bg">

    <form @submit="handleSubmit" class="auth-form">
      <h1 class="auth-title">注册</h1>

      <div class="auth-content">
        <!-- 用户名输入框 -->
        <div class="auth-input-group">
          <i class="ri-user-3-line auth-icon"></i>
          <div class="auth-input-wrapper">
            <input
              type="text"
              required
              class="auth-input"
              id="register-username"
              placeholder=" "
              v-model="userName"
              autocomplete="username"
            >
            <label for="register-username" class="auth-label">账号</label>
          </div>
        </div>

        <!-- 邮箱输入框 -->
        <div class="auth-input-group">
          <i class="ri-mail-line auth-icon"></i>
          <div class="auth-input-wrapper">
            <input
              type="email"
              required
              class="auth-input"
              id="register-email"
              placeholder=" "
              v-model="email"
              autocomplete="email"
            >
            <label for="register-email" class="auth-label">邮箱</label>
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
              id="register-pass"
              placeholder=" "
              v-model="password"
              autocomplete="new-password"
            >
            <label for="register-pass" class="auth-label">密码</label>
            <el-icon class="auth-eye-btn" @click="togglePasswordVisibility">
              <Hide v-if="isPasswordVisible"/>
              <View v-else/>
            </el-icon>
          </div>
        </div>

        <!-- 确认密码输入框 -->
        <div class="auth-input-group">
          <i class="ri-lock-2-line auth-icon"></i>
          <div class="auth-input-wrapper">
            <input
              :type="isConfirmPasswordVisible ? 'text' : 'password'"
              required
              class="auth-input"
              id="register-confirm"
              placeholder=" "
              v-model="confirmPassword"
              autocomplete="new-password"
            >
            <label for="register-confirm" class="auth-label">确认密码</label>
            <el-icon class="auth-eye-btn" @click="toggleConfirmPasswordVisibility">
              <Hide v-if="isConfirmPasswordVisible"/>
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
            id="register-terms"
            v-model="agreeTerms"
          >
          <label for="register-terms" class="auth-check-label">我已阅读并同意相关条款</label>
        </div>
      </div>

      <button 
        type="submit" 
        class="auth-submit-btn"
        :disabled="loading"
      >
        {{ loading ? '注册中...' : '注册' }}
      </button>

      <p class="auth-switch">
        已有账号?
        <router-link to="/login" class="auth-switch-link">登录</router-link>
      </p>
    </form>
  </div>
</template>

<style scoped>
/* 页面特定样式（如有需要） */
</style>