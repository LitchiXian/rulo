<script setup lang="ts">
import {ref} from 'vue';
import {Hide, View} from '@element-plus/icons-vue';
import {useRouter} from "vue-router";
import {getRegisterCode, register} from "@/api/web/login";
import {showMessage} from "@/util/message.ts";

/*--------------- 响应式状态声明 ---------------*/
const userName = ref<string>('');       // 用户名输入值
const email = ref<string>('');         // 邮箱输入值
const password = ref<string>('');      // 密码输入值
const code = ref<string>('');      // 密码输入值
const confirmPassword = ref<string>(''); // 确认密码输入值
const isPasswordVisible = ref<boolean>(false); // 密码可见状态
const isConfirmPasswordVisible = ref<boolean>(false); // 确认密码可见状态

// 验证码相关状态
const showCaptchaDialog = ref<boolean>(false); // 控制验证码弹窗显示
const captchaVerified = ref<boolean>(false);    // 是否通过验证码验证
const countdown = ref<number>(0);              // 倒计时秒数
const timer = ref<NodeJS.Timeout | null>(null); // 倒计时定时器
const agreeTerms = ref<boolean>(false);       // 是否同意条款

const sliderPosition = ref(0);
const sliderPercentage = ref(0);
const sliderComplete = ref(false);
const errorMessage = ref('');


// 是否正在拖动
const isDragging = ref(false);
const dragStartX = ref(0);
const dragStartLeft = ref(0);
const maxLeft = ref(0);

const router = useRouter()
const loginButtonRef = ref<HTMLButtonElement | null>(null);
const loading = ref<boolean>(false);

/*--------------- 密码可见性切换方法 ---------------*/
const togglePasswordVisibility = () => {
  isPasswordVisible.value = !isPasswordVisible.value;
};

const toggleConfirmPasswordVisibility = () => {
  isConfirmPasswordVisible.value = !isConfirmPasswordVisible.value;
};

/*--------------- 验证码相关方法 ---------------*/
// 打开验证码弹窗
const openCaptchaDialog = () => {
  showCaptchaDialog.value = true;
};

// 验证码拖动完成处理
const handleCaptchaComplete = () => {
  captchaVerified.value = true;
  showMessage('验证成功', 'success');

  // 关闭弹窗
  setTimeout(() => {
    showCaptchaDialog.value = false;
    getVerificationCode();
  }, 1000);
};

// 获取验证码
const getVerificationCode = async () => {
  if (!email.value) {
    showMessage('请先输入邮箱', 'error');
    return;
  }

  try {
    const res = await getRegisterCode({email: email.value});
    if (res.code === 200) {
      showMessage('验证码已发送', 'success');
      startCountdown();
    } else {
      showMessage(res.result.msg || '验证码发送失败', 'error');
    }
  } catch (error) {
    showMessage('验证码发送失败', 'error');
  }
};

// 开始倒计时
const startCountdown = () => {
  countdown.value = 60;
  timer.value = setInterval(() => {
    countdown.value--;
    if (countdown.value <= 0 && timer.value) {
      clearInterval(timer.value);
      timer.value = null;
    }
  }, 1000);
};

/*--------------- 表单提交处理（示例） ---------------*/
const handleSubmit = async (e: Event) => {
  e.preventDefault(); // 阻止表单默认提交
  console.log('注册信息:', {
    userName: userName.value,
    email: email.value,
    password: password.value,
    confirmPassword: confirmPassword.value
  });
  // 这里添加实际注册逻辑（如调用API）

  loading.value = true;
  if (loginButtonRef.value) {
    loginButtonRef.value.disabled = true;
    loginButtonRef.value.textContent = '注册中...';
  }

  try {
    const res = await register({
      userName: userName.value,
      email: email.value,
      password: password.value,
      code: code.value
    });

    if (res.code === 200) {
      // 注册成功
      console.log('注册成功');
      showMessage('注册成功', 'success');
      // 跳转到首页
      router.push('/login');
    } else {
      // 注册失败
      console.log('注册失败');
      showMessage(res.result.msg, 'error');

    }
  } finally {
    loading.value = false;
    if (loginButtonRef.value) {
      loginButtonRef.value.disabled = false;
      loginButtonRef.value.textContent = 'Register';
    }
  }
};
</script>

<template>
  <div class="login-container">
    <!-- 背景图（取消注释启用） -->
    <img src="@/asset/image/login-bg.png" alt="登录背景" class="login-bg">

    <form @submit="handleSubmit" class="login-form">
      <h1 class="login-title">Register</h1>

      <div class="login-content">
        <!-- 用户名输入框 -->
        <div class="login-box">
          <i class="ri-user-3-line login-icon"></i>
          <div class="login-box-input">
            <input
                type="text"
                required
                class="login-input"
                id="register-username"
                placeholder=" "
                v-model="userName"
            >
            <label for="register-username" class="login-label">UserName</label>
          </div>
        </div>

        <!-- 邮箱输入框 -->
        <div class="login-box">
          <i class="ri-user-3-line login-icon"></i>
          <div class="login-box-input">
            <input
                type="text"
                required
                class="login-input"
                id="register-email"
                placeholder=" "
                v-model="email"
            >
            <label for="register-email" class="login-label">Email</label>
          </div>
        </div>

        <!-- 密码输入框 -->
        <div class="login-box">
          <i class="ri-lock-2-line login-icon"></i>
          <div class="login-box-input">
            <input
                :type="isPasswordVisible ? 'text' : 'password'"
                required
                class="login-input"
                id="register-pass"
                placeholder=" "
                v-model="password"
            >
            <label for="register-pass" class="login-label">Password</label>
            <!-- 密码可见切换图标 -->
            <el-icon
                :class="['login-eye']"
                @click="togglePasswordVisibility"
            >
              <Hide v-if="isPasswordVisible"/>
              <View v-else/>
            </el-icon>
          </div>
        </div>

        <!-- 确认密码输入框 -->
        <div class="login-box">
          <i class="ri-lock-2-line login-icon"></i>
          <div class="login-box-input">
            <input
                :type="isConfirmPasswordVisible ? 'text' : 'password'"
                required
                class="login-input"
                id="register-confirm-pass"
                placeholder=" "
                v-model="confirmPassword"
            >
            <label for="register-confirm-pass" class="login-label">Confirm Password</label>
            <!-- 密码可见切换图标 -->
            <el-icon
                :class="['login-eye']"
                @click="toggleConfirmPasswordVisibility"
            >
              <Hide v-if="isConfirmPasswordVisible"/>
              <View v-else/>
            </el-icon>
          </div>
        </div>
      </div>

      <!-- 验证码输入框 -->
      <div class="login-box">
        <i class="ri-shield-keyhole-line login-icon"></i>
        <div class="login-box-input">
          <input
              type="text"
              required
              class="login-input"
              id="register-code"
              placeholder=" "
              v-model="code"
          >
          <label for="register-code" class="login-label">Verification Code</label>
        </div>
        <button
            type="button"
            class="get-code-btn"
            :disabled="countdown > 0"
            @click="openCaptchaDialog"
        >
          {{ countdown > 0 ? `${countdown}s` : 'Get Code' }}
        </button>
      </div>

      <div class="login-check">
        <div class="login-check-group">
          <input type="checkbox" class="login-check-input" id="register-check">
          <label for="register-check" class="login-check-label">I agree to the terms and conditions</label>
        </div>
      </div>

      <button type="submit" class="login-button">Register</button>

      <!-- 添加登录链接，与登录页保持一致 -->
      <p class="login-register">
        Already have an account?
        <router-link to="/login" class="login-register-link">Login</router-link>
      </p>
    </form>

    <!-- 验证码弹窗 -->
    <div v-if="showCaptchaDialog" class="captcha-modal">
      <div class="captcha-content">
        <div class="captcha-header">
          <h3>请完成安全验证</h3>
          <button @click="showCaptchaDialog = false" class="captcha-close">×</button>
        </div>
        <div class="captcha-body">
          <p>请拖动滑块完成验证</p>
          <div class="captcha-slider" @mouseup="handleCaptchaComplete">
            <div class="slider-track"></div>
            <div class="slider-button">→</div>
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
/* 复用登录页的所有样式 */
.login-container {
  position: relative;
  height: 100vh;
  display: grid;
  align-items: center;
  justify-content: center;
  /* 防止子元素溢出 */
  overflow: hidden;
}

.login-bg {
  position: absolute;
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center;
  opacity: 0.9; /* 背景图透明度 */
}

.login-form {
  position: relative;
  background: rgba(255, 255, 255, 0.05); /* 半透明背景 */
  backdrop-filter: blur(10px); /* 毛玻璃效果 */
  border: 1px solid rgba(255, 255, 255, 0.1); /* 边框 */
  border-radius: 15px;
  padding: 2.5rem;
  width: 90%;
  max-width: 400px; /* 最大宽度 */
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  /* 限制组件内样式作用范围 */
  z-index: 1; /* 确保表单在背景图上方 */
}

.login-title {
  text-align: center;
  color: #ffffff;
  font-size: 1.8rem;
  margin-bottom: 2rem;
  font-weight: 500;
}

.login-content {
  display: grid;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.login-box {
  display: grid;
  grid-template-columns: 30px 1fr; /* 图标固定宽度 */
  align-items: center;
  gap: 0.75rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
}

.login-icon {
  color: #ffffff;
  font-size: 1.25rem;
}

.login-input {
  width: 100%;
  padding: 0.8rem 0;
  background: transparent;
  color: #ffffff;
  font-size: 1rem;
  border: none;
  outline: none;
  transition: 0.3s;
}

.login-box-input {
  position: relative;
}

.login-label {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  color: rgba(255, 255, 255, 0.6);
  pointer-events: none;
  transition: 0.3s;
}

.login-input:focus ~ .login-label,
.login-input:not(:placeholder-shown) ~ .login-label {
  top: -5px;
  font-size: 0.8rem;
  color: #ffffff;
}

.login-eye {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  cursor: pointer;
  color: rgba(255, 255, 255, 0.6);
  transition: 0.3s;
}

.login-eye:hover {
  color: #ffffff;
}

.login-check {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.login-check-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.login-check-input {
  width: 16px;
  height: 16px;
  accent-color: #ffffff; /* 复选框选中颜色 */
}

.login-forgot {
  color: rgba(255, 255, 255, 0.6);
  transition: 0.3s;
}

.login-forgot:hover {
  color: #ffffff;
  text-decoration: underline;
}

.login-button {
  width: 100%;
  padding: 1rem;
  background: #ffffff;
  color: #764ba2;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: 0.3s;
}

.login-button:hover {
  background: #f0f0f0;
  transform: translateY(-2px);
}

.login-register {
  text-align: center;
  color: rgba(255, 255, 255, 0.6);
  margin-top: 1rem;
}

.login-register-link {
  color: #ffffff;
  font-weight: 500;
  transition: 0.3s;
}

.login-register-link:hover {
  text-decoration: underline;
}

/* 响应式设计 */
@media (min-width: 768px) {
  .login-form {
    padding: 3rem;
    max-width: 450px;
  }

  .login-title {
    font-size: 2rem;
  }
}

/* 验证码相关样式 */
.captcha-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.captcha-content {
  background-color: white;
  border-radius: 8px;
  width: 350px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.captcha-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #eee;
}

.captcha-close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #999;
}

.captcha-body {
  padding: 20px;
  text-align: center;
}

.captcha-slider {
  position: relative;
  height: 40px;
  background: #f5f5f5;
  border-radius: 20px;
  margin: 20px 0;
  cursor: pointer;
}

.slider-track {
  position: absolute;
  height: 100%;
  width: 100%;
  background: #e0e0e0;
  border-radius: 20px;
}

.slider-button {
  position: absolute;
  height: 40px;
  width: 40px;
  background: white;
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  left: 0;
  top: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  z-index: 2;
}

.captcha-success {
  color: #4CAF50;
  font-weight: bold;
  margin-top: 10px;
}

/* 验证码按钮样式 */
.get-code-btn {
  position: absolute;
  right: 30px;
  transform: translateY(-50%);
  height: 30px;
  width: 90px;
  cursor: pointer;
  border: none;
  background-color: #4b4b4b;
  color: white;
  border-radius: 4px;
  font-size: 14px;
  transition: all 0.3s;
}

.get-code-btn:hover {
  background-color: #656565;
}

.get-code-btn:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

/* 原有样式基础上增加邮箱图标 */
.login-box .ri-mail-line {
  color: #666;
}

.login-box .ri-shield-keyhole-line {
  color: #666;
}

/* 调整验证码输入框位置 */
.login-box:has(.get-code-btn) .login-box-input {
  padding-right: 100px;
}
</style>