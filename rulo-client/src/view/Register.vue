<script setup lang="ts">
import { ref, onUnmounted } from 'vue';
import { Hide, View } from '@element-plus/icons-vue';
import { useRouter } from "vue-router";
import authApi from "@/api/web/auth.ts";
import { showMessage } from "@/util/message.ts";

// 表单数据
const userName = ref('');
const email = ref('');
const password = ref('');
const confirmPassword = ref('');
const code = ref('');
const agreeTerms = ref(false);

// UI状态
const isPasswordVisible = ref(false);
const isConfirmPasswordVisible = ref(false);
const showCaptchaDialog = ref(false);
const captchaVerified = ref(false);
const countdown = ref(0);
const loading = ref(false);

let timer: ReturnType<typeof setInterval> | null = null;
const router = useRouter();

// 清理定时器
onUnmounted(() => {
  if (timer) clearInterval(timer);
});

// 切换密码可见性
const togglePasswordVisibility = () => {
  isPasswordVisible.value = !isPasswordVisible.value;
};

const toggleConfirmPasswordVisibility = () => {
  isConfirmPasswordVisible.value = !isConfirmPasswordVisible.value;
};

// 打开验证码弹窗
const openCaptchaDialog = () => {
  if (!email.value) {
    showMessage('请先输入邮箱', 'warning');
    return;
  }
  showCaptchaDialog.value = true;
};

// 验证码完成处理
const handleCaptchaComplete = () => {
  captchaVerified.value = true;
  showMessage('验证成功', 'success');

  setTimeout(() => {
    showCaptchaDialog.value = false;
    getVerificationCode();
  }, 800);
};

// 获取验证码
const getVerificationCode = async () => {
  try {
    await authApi.getRegisterCode({ email: email.value });
    showMessage('验证码已发送', 'success');
    startCountdown();
  } catch (err) {
    console.error('获取验证码失败:', err);
  }
};

// 开始倒计时
const startCountdown = () => {
  countdown.value = 60;
  timer = setInterval(() => {
    countdown.value--;
    if (countdown.value <= 0 && timer) {
      clearInterval(timer);
      timer = null;
    }
  }, 1000);
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
  if (!code.value.trim()) {
    showMessage('请输入验证码', 'warning');
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
      userName: userName.value,
      email: email.value,
      password: password.value,
      code: code.value
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

        <!-- 验证码输入框 -->
        <div class="auth-input-group with-action">
          <i class="ri-shield-keyhole-line auth-icon"></i>
          <div class="auth-input-wrapper">
            <input
              type="text"
              required
              class="auth-input"
              id="register-code"
              placeholder=" "
              v-model="code"
              autocomplete="one-time-code"
            >
            <label for="register-code" class="auth-label">验证码</label>
          </div>
          <button
            type="button"
            class="auth-code-btn"
            :disabled="countdown > 0"
            @click="openCaptchaDialog"
          >
            {{ countdown > 0 ? `${countdown}s` : '获取验证码' }}
          </button>
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

    <!-- 验证码弹窗 -->
    <div v-if="showCaptchaDialog" class="captcha-modal">
      <div class="captcha-content">
        <div class="captcha-header">
          <h3>请完成安全验证</h3>
          <button @click="showCaptchaDialog = false" class="captcha-close">&times;</button>
        </div>
        <div class="captcha-body">
          <p>请拖动滑块完成验证</p>
          <div class="captcha-slider" @mouseup="handleCaptchaComplete">
            <div class="slider-track"></div>
            <div class="slider-button">&rarr;</div>
          </div>
          <div v-if="captchaVerified" class="captcha-success">
            <span>✓</span> 验证成功
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 页面特定样式（如有需要） */
</style>