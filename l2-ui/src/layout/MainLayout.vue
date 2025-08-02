<template>
  <div class="layout-container">
    <!-- 左侧导航栏 -->
    <div class="left-sidebar">
      <div class="author-info">
        <div v-if="!userStore.isLoggedIn" class="login-prompt" @click.prevent="goToLogin">
          去登录
        </div>
        <div v-else @click.prevent="goToUser(userStore.userInfo?.id)">
          <img src="@/asset/avatar.jpg" alt="博主头像" class="author-avatar"/>
        </div>
        <!--        <img src="@/asset/avatar.jpg" alt="博主头像" class="author-avatar"/>-->
        <h2 class="author-name">Litchi 的博客</h2>
      </div>

      <nav class="sidebar-nav">
        <router-link to="/">
          <span class="icon">🏠</span> Home
        </router-link>
        <a v-if="!userStore.isLoggedIn" href="#" @click.prevent="goToLogin" class="api-link">
          <span class="icon">🏷️</span> 登录
        </a>
        <a v-if="userStore.isLoggedIn" href="#" @click="handleLogoutClick" class="api-link">
          <span class="icon">👤</span> 注销
        </a>
        <!-- 修改 API 链接 -->
        <a href="#" @click="openApiDoc" class="api-link">
          <span class="icon">🔗</span> API
        </a>
        <router-link v-if="userStore.isLoggedIn" to="/saveArticle">
          <span class="icon">🛠️</span> 新增
        </router-link>

        <div class="dark-mode-toggle" @click="toggleDarkMode">
          <span class="icon">{{ darkMode ? '🌞' : '🌙' }}</span>
          {{ darkMode ? 'Light Mode' : 'Dark Mode' }}
        </div>
      </nav>
    </div>

    <!-- 中间内容区域 -->
    <div class="content-area">
      <router-view/>
    </div>

    <!-- 右侧目录栏 -->
    <div class="right-sidebar" v-if="showToc">
      <div class="toc-container">
        <h3 class="toc-title">文章目录</h3>
        <ul class="toc-list">
          <li v-for="(item, index) in tocItems" :key="index" :class="`toc-level-${item.level}`">
            <a :href="`#${item.id}`" @click.prevent="scrollToAnchor(item.id)">
              {{ item.text }}
            </a>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useRouter} from 'vue-router'
import {onMounted, ref} from 'vue';
import {useUserStore} from "@/store/user.ts";

const userStore = useUserStore();
const darkMode = ref(true);
const showToc = ref(true);
const router = useRouter();
const tocItems = ref([]); // 从文章内容提取的目录项

onMounted(() => {
  userStore.initUser();
})

const toggleDarkMode = () => {
  darkMode.value = !darkMode.value;
  document.body.classList.toggle('dark-mode', darkMode.value);
};

const scrollToAnchor = (id) => {
  const el = document.getElementById(id);
  if (el) {
    el.scrollIntoView({behavior: 'smooth'});
  }
};

const openApiDoc = () => {
  // 在新窗口打开 API 文档
  window.open('http://localhost:8090/doc.html', '_blank');

  // 添加点击反馈效果
  const apiLink = document.querySelector('.api-link');
  if (apiLink && 'style' in apiLink) {
    apiLink.style.transform = 'scale(0.95)';
    setTimeout(() => {
      apiLink.style.transform = '';
    }, 300);
  }
};

const goToLogin  = () => {
    // 跳转到登录页，并记录来源页面
    router.push({
      path: '/login',
      query: {redirect: router.currentRoute.value.fullPath}
    })
}

const handleLogoutClick = async () => {
  try {
    // 调用登出API（假设logout是异步函数）
    await userStore.logout();
  } finally {
    localStorage.removeItem('satoken');
    sessionStorage.removeItem('satoken');
  }
};

const goToUser = (id?:string) => {
  if (!id) {
    // 处理ID缺失的情况
    console.warn("跳转用户页失败：缺少用户ID");
    return;
  }
  router.push('/user/' + id);
}

// 根据文章内容生成目录项的逻辑可以放在这里
// 或者在文章页面加载时设置
defineExpose({
  setTocItems: (items: any) => {
    tocItems.value = items;
    showToc.value = items.length > 0;
  }
});
</script>

<style scoped>
.layout-container {
  display: flex;
  min-height: 100vh;
  background-color: var(--body-bg-color); /* 48,48,48 */
}

.left-sidebar {
  width: 250px;
  color: #e2e2ec; /* 226,226,236 - 标题颜色 */
  position: fixed;
  left: 0;
  top: 0;
  bottom: 0;
  overflow-y: auto;
  padding: 20px;
  margin-left: 5%;
  margin-top: 30px;
  border-right: 0px solid #424242; /* 66,66,66 - 框颜色 */
}

.author-info {
  text-align: center;
  margin-bottom: 30px;
}

.login-prompt {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background-color: #4abbb5; /* 青色背景 */
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  font-size: 1.2rem;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-bottom: 15px;
}

.author-avatar {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  object-fit: cover;
  margin-bottom: 15px;
}

.author-name {
  font-size: 1.5rem;
  margin-bottom: 5px;
}

.sidebar-nav {
  margin-left: 20%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.sidebar-nav a {
  display: flex;
  align-items: center;
  padding: 10px 15px;
  color: #ece2c0; /* 236,226,192 - 字体颜色 */
  text-decoration: none;
  border-radius: 5px;
  transition: background-color 0.3s;
}

.sidebar-nav a:hover {
  background-color: #424242; /* 66,66,66 */
}

.icon {
  margin-right: 10px;
  font-size: 1.2rem;
}

.dark-mode-toggle {
  display: flex;
  align-items: center;
  padding: 10px 15px;
  cursor: pointer;
  border-radius: 5px;
  margin-top: 20px;
  transition: background-color 0.3s;
}

.dark-mode-toggle:hover {
  background-color: #555;
}

.content-area {
  width: 43%;
  margin: 50px 0 0 24%; /* 上右下左边距匹配侧边栏宽度 */
  padding: 10px;
  background-color: var(--card-bg-color); /*  66,66,66 - 内容背景色 */
  color: #ece2c0; /* 236,226,192 - 字体颜色 */
  border-top-left-radius: 10px; /* 左上角圆角 */
  border-top-right-radius: 10px; /* 右上角圆角 */
}

.right-sidebar {
  width: 350px;
  color: #e2e2ec; /* 226,226,236 - 标题颜色 */
  position: fixed;
  right: 0;
  top: 0;
  bottom: 0;
  overflow-y: auto;
  padding: 20px;
  margin-right: 200px;
  margin-top: 30px;
  border-left: 0px solid #424242; /* 66,66,66 - 框颜色 */
}

.toc-container {
  background-color: #424242; /* 66,66,66 */
  border-radius: 8px;
  padding: 15px;
}

.toc-title {
  font-size: 1.25rem;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 1px solid #555;
}

.toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.toc-list li {
  margin-bottom: 8px;
  line-height: 1.4;
}

.toc-list a {
  color: #ece2c0; /* 236,226,192 */
  text-decoration: none;
  transition: color 0.3s;
  display: block;
  padding: 5px 0;
}

.toc-list a:hover {
  color: #4abbb5; /* 青色高亮 */
}

.toc-level-2 {
  padding-left: 15px;
}

.toc-level-3 {
  padding-left: 30px;
}

.toc-level-4 {
  padding-left: 45px;
}
</style>