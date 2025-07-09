<template>
  <div class="hello">

    <!-- 主要欢迎信息 -->
    <h1 :class="{ 'pulse': isHighlighted }">
      {{ greetingMessage }}
    </h1>

    <!-- 计数器控制区域 -->
    <div class="counter-container">
      <button @click="decrementCounter" class="counter-btn" :disabled="counter <= 0">-</button>
      <span class="counter-value">{{ counter }}</span>
      <button @click="incrementCounter" class="counter-btn">+</button>
    </div>

    <!-- 功能按钮组 -->
    <div class="buttons">
      <button
          v-for="(button, index) in functionalButtons"
          :key="index"
          @click="executeAction(button.action)"
          :class="button.style"
      >
        {{ button.label }}
      </button>
    </div>

    <!-- 文档链接 -->
    <div class="links">
      <h3>推荐资源</h3>
      <ul>
        <li v-for="(link, index) in usefulLinks" :key="index">
          <a :href="link.url" target="_blank" rel="noopener">
            {{ link.title }}
          </a>
          <span class="link-desc">{{ link.description }}</span>
        </li>
      </ul>
    </div>

    <!-- 自定义问候语 -->
    <div class="custom-greeting">
      <input
          v-model="customGreeting"
          type="text"
          placeholder="输入自定义欢迎语"
          @keyup.enter="updateGreeting"
      >
      <button @click="updateGreeting">更新</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, reactive, watch, onMounted } from 'vue';

export default defineComponent({
  name: 'HelloWorld',
  props: {
    msg: {
      type: String,
      required: true,
      default: 'Welcome to Your Vue.js App'
    },
    showLogo: {
      type: Boolean,
      default: true
    },
    initialCounter: {
      type: Number,
      default: 0
    }
  },
  setup(props) {
    // 响应式状态
    const counter = ref(props.initialCounter);
    const customGreeting = ref('');
    const animationActive = ref(true);
    const isHighlighted = ref(false);

    // 计算属性
    const greetingMessage = computed(() => {
      return customGreeting.value || props.msg;
    });

    // 响应式对象
    const functionalButtons = reactive([
      { label: '切换动画', action: 'toggleAnimation', style: 'toggle-btn' },
      { label: '重置计数器', action: 'resetCounter', style: 'reset-btn' },
      { label: '高亮欢迎', action: 'highlightGreeting', style: 'highlight-btn' }
    ]);

    const usefulLinks = reactive([
      {
        title: 'Vue 3 官方文档',
        url: 'https://vuejs.org',
        description: '学习 Vue 核心概念'
      },
      {
        title: 'Vue Router',
        url: 'https://router.vuejs.org',
        description: '单页应用路由解决方案'
      },
      {
        title: 'Pinia',
        url: 'https://pinia.vuejs.org',
        description: 'Vue 官方状态管理库'
      },
      {
        title: 'Vite',
        url: 'https://vitejs.dev',
        description: '下一代前端工具链'
      }
    ]);

    // 方法
    const incrementCounter = () => {
      counter.value++;
    };

    const decrementCounter = () => {
      if (counter.value > 0) {
        counter.value--;
      }
    };

    const toggleAnimation = () => {
      animationActive.value = !animationActive.value;
    };

    const resetCounter = () => {
      counter.value = props.initialCounter;
    };

    const highlightGreeting = () => {
      isHighlighted.value = true;
      setTimeout(() => {
        isHighlighted.value = false;
      }, 1000);
    };

    const updateGreeting = () => {
      if (customGreeting.value.trim()) {
        customGreeting.value = customGreeting.value;
      } else {
        customGreeting.value = '';
      }
    };

    // 执行按钮对应的操作
    const executeAction = (action: string) => {
      switch(action) {
        case 'toggleAnimation':
          toggleAnimation();
          break;
        case 'resetCounter':
          resetCounter();
          break;
        case 'highlightGreeting':
          highlightGreeting();
          break;
        default:
          console.log('Unknown action');
      }
    };

    // 生命周期钩子
    onMounted(() => {
      console.log('HelloWorld component mounted');
    });

    // 监听器
    watch(counter, (newValue) => {
      console.log(`Counter updated to ${newValue}`);
      if (newValue >= 5) {
        highlightGreeting();
      }
    });

    return {
      // 状态
      counter,
      customGreeting,
      animationActive,
      isHighlighted,

      // 计算属性
      greetingMessage,

      // 数据
      functionalButtons,
      usefulLinks,

      // 方法
      incrementCounter,
      decrementCounter,
      executeAction,
      updateGreeting
    };
  }
});
</script>

<style scoped>
.hello {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
  text-align: center;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  color: #2c3e50;
}

.logo {
  height: 150px;
  width: 150px;
  margin-bottom: 2rem;
  transition: transform 0.3s ease;
}

.logo-animation {
  animation: bounce 2s infinite;
}

h1 {
  font-size: 2.2rem;
  margin-bottom: 1.5rem;
  transition: color 0.3s ease;
}

h1.pulse {
  animation: pulse 1s;
  color: #42b983;
}

.counter-container {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 1.5rem 0;
  gap: 15px;
}

.counter-btn {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 50%;
  background-color: #42b983;
  color: white;
  font-size: 1.2rem;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.counter-btn:hover {
  background-color: #3aa876;
}

.counter-btn:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.counter-value {
  font-size: 1.5rem;
  min-width: 50px;
  display: inline-block;
  font-weight: bold;
}

.buttons {
  display: flex;
  gap: 10px;
  margin-bottom: 2rem;
}

button {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.toggle-btn {
  background-color: #f0ad4e;
  color: #fff;
}

.reset-btn {
  background-color: #d9534f;
  color: #fff;
}

.highlight-btn {
  background-color: #5bc0de;
  color: #fff;
}

button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}

.links {
  text-align: left;
  margin-top: 2rem;
  padding: 1rem;
  border-top: 1px solid #eee;
  width: 100%;
}

.links h3 {
  text-align: center;
  margin-bottom: 1rem;
}

.links ul {
  list-style-type: none;
  padding: 0;
}

.links li {
  padding: 8px 0;
  border-bottom: 1px dashed #eee;
}

.links a {
  color: #42b983;
  text-decoration: none;
  font-weight: 500;
}

.links a:hover {
  text-decoration: underline;
}

.link-desc {
  display: block;
  font-size: 0.85rem;
  color: #7f8c8d;
  margin-top: 4px;
}

.custom-greeting {
  display: flex;
  gap: 10px;
  margin-top: 2rem;
  width: 100%;
  max-width: 500px;
}

.custom-greeting input {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
}

.custom-greeting button {
  background-color: #42b983;
  color: white;
}

@keyframes bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-20px); }
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.05); }
}
</style>