<template>
  <div class="layout-container">
    <!-- 左侧导航栏 -->
    <aside class="left-sidebar">
      <div class="author-info">
        <div v-if="!userStore.isLoggedIn" class="login-prompt" @click.prevent="goToLogin">
          去登录
        </div>
        <div v-else @click.prevent="goToUser(userStore.userInfo?.id)" class="avatar-wrapper">
          <img src="@/asset/avatar.jpg" alt="头像" class="author-avatar"/>
        </div>
        <h2 class="author-name">Litchi 的博客</h2>
      </div>

      <nav class="sidebar-nav">
        <router-link to="/" class="nav-item">
          <span class="nav-icon">🏠</span>
          <span class="nav-text">首页</span>
        </router-link>
        
        <a v-if="!userStore.isLoggedIn" href="#" @click.prevent="goToLogin" class="nav-item">
          <span class="nav-icon">🏷️</span>
          <span class="nav-text">登录</span>
        </a>
        
        <a v-if="userStore.isLoggedIn" href="#" @click="handleLogoutClick" class="nav-item">
          <span class="nav-icon">👤</span>
          <span class="nav-text">注销</span>
        </a>
        
        <a href="#" @click="openApiDoc" class="nav-item">
          <span class="nav-icon">🔗</span>
          <span class="nav-text">API</span>
        </a>
        
        <router-link v-if="userStore.isLoggedIn" to="/saveArticle" class="nav-item">
          <span class="nav-icon">🛠️</span>
          <span class="nav-text">新增</span>
        </router-link>

        <div class="nav-item dark-mode-toggle" @click="toggleDarkMode">
          <span class="nav-icon">{{ darkMode ? '🌞' : '🌙' }}</span>
          <span class="nav-text">{{ darkMode ? '浅色' : '深色' }}</span>
        </div>
      </nav>
    </aside>

    <!-- 中间内容区域 -->
    <main class="content-area">
      <router-view/>
    </main>

    <!-- 右侧目录栏 -->
    <aside class="right-sidebar toc-sidebar" v-if="showToc && tocItems.length > 0">
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
    </aside>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted, ref } from 'vue';
import { useUserStore } from "@/store/user.ts";

interface TocItem {
  id: string;
  text: string;
  level: number;
}

const userStore = useUserStore();
const darkMode = ref(true);
const showToc = ref(true);
const router = useRouter();
const tocItems = ref<TocItem[]>([]);

onMounted(() => {
  userStore.initUser();
});

const toggleDarkMode = () => {
  darkMode.value = !darkMode.value;
  document.body.classList.toggle('dark-mode', darkMode.value);
};

const scrollToAnchor = (id: string) => {
  const el = document.getElementById(id);
  if (el) {
    el.scrollIntoView({ behavior: 'smooth' });
  }
};

const openApiDoc = () => {
  window.open('http://localhost:8090/doc.html', '_blank');
};

const goToLogin = () => {
  router.push({
    path: '/login',
    query: { redirect: router.currentRoute.value.fullPath }
  });
};

const handleLogoutClick = async () => {
  try {
    await userStore.logout();
  } finally {
    localStorage.removeItem('satoken');
    sessionStorage.removeItem('satoken');
  }
};

const goToUser = (id?: string) => {
  if (!id) {
    console.warn("跳转用户页失败：缺少用户ID");
    return;
  }
  router.push('/user/' + id);
};

// 暴露方法供子组件设置目录
defineExpose({
  setTocItems: (items: TocItem[]) => {
    tocItems.value = items;
    showToc.value = items.length > 0;
  }
});
</script>

<style scoped>
.layout-container {
  display: flex;
  min-height: 100vh;
  background-color: var(--bg-body);
}

/* ======== 左侧边栏 ======== */
.left-sidebar {
  width: var(--sidebar-width);
  color: var(--text-title);
  position: fixed;
  left: 5%;
  top: 0;
  bottom: 0;
  overflow-y: auto;
  padding: var(--spacing-lg);
  padding-top: var(--spacing-xl);
}

.author-info {
  text-align: center;
  margin-bottom: var(--spacing-xl);
}

.login-prompt {
  width: 100px;
  height: 100px;
  margin: 0 auto var(--spacing-md);
  border-radius: var(--radius-full);
  background-color: var(--color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-white);
  font-weight: 600;
  font-size: 1rem;
  cursor: pointer;
  transition: transform var(--transition-fast), box-shadow var(--transition-fast);
}

.login-prompt:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(74, 187, 181, 0.4);
}

.avatar-wrapper {
  cursor: pointer;
}

.author-avatar {
  width: 100px;
  height: 100px;
  border-radius: var(--radius-full);
  object-fit: cover;
  margin: 0 auto var(--spacing-md);
  display: block;
  transition: transform var(--transition-fast);
}

.author-avatar:hover {
  transform: scale(1.05);
}

.author-name {
  font-size: 1.2rem;
  margin: 0;
  color: var(--text-title);
}

/* ======== 导航菜单 ======== */
.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  padding-left: 10%;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  color: var(--text-primary);
  text-decoration: none;
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
  cursor: pointer;
}

.nav-item:hover {
  background-color: var(--bg-card);
  text-decoration: none;
}

.nav-item.router-link-active {
  background-color: var(--color-primary-light);
  color: var(--color-primary);
}

.nav-icon {
  margin-right: var(--spacing-sm);
  font-size: 1.1rem;
}

.dark-mode-toggle {
  margin-top: var(--spacing-lg);
}

/* ======== 内容区域 ======== */
.content-area {
  flex: 1;
  max-width: 800px;
  margin: var(--spacing-xl) auto;
  margin-left: calc(var(--sidebar-width) + 10%);
  padding: var(--spacing-lg);
  background-color: var(--bg-card);
  color: var(--text-primary);
  border-radius: var(--radius-md) var(--radius-md) 0 0;
  min-height: calc(100vh - var(--spacing-xl));
}

/* ======== 右侧目录栏 ======== */
.right-sidebar {
  width: var(--toc-width);
  color: var(--text-title);
  position: fixed;
  right: 5%;
  top: var(--spacing-xl);
  max-height: calc(100vh - var(--spacing-2xl));
  overflow-y: auto;
}

.toc-container {
  background-color: var(--bg-card);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
}

.toc-title {
  font-size: 1rem;
  margin: 0 0 var(--spacing-md);
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px solid var(--border-color);
  color: var(--text-title);
}

.toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.toc-list li {
  margin-bottom: var(--spacing-xs);
  line-height: 1.4;
}

.toc-list a {
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 0.9rem;
  display: block;
  padding: var(--spacing-xs) 0;
  transition: color var(--transition-fast);
}

.toc-list a:hover {
  color: var(--color-primary);
}

.toc-level-2 { padding-left: var(--spacing-md); }
.toc-level-3 { padding-left: calc(var(--spacing-md) * 2); }
.toc-level-4 { padding-left: calc(var(--spacing-md) * 3); }

/* ======== 响应式适配 ======== */
@media (max-width: 1200px) {
  .right-sidebar {
    display: none;
  }
  
  .content-area {
    margin-right: 5%;
  }
}

@media (max-width: 900px) {
  .left-sidebar {
    width: var(--sidebar-collapsed);
    left: 0;
    padding: var(--spacing-md);
  }
  
  .author-name,
  .nav-text {
    display: none;
  }
  
  .author-info {
    margin-bottom: var(--spacing-lg);
  }
  
  .login-prompt,
  .author-avatar {
    width: 40px;
    height: 40px;
    font-size: 0.7rem;
  }
  
  .sidebar-nav {
    padding-left: 0;
    align-items: center;
  }
  
  .nav-item {
    padding: var(--spacing-sm);
    justify-content: center;
  }
  
  .nav-icon {
    margin-right: 0;
  }
  
  .content-area {
    margin-left: calc(var(--sidebar-collapsed) + var(--spacing-lg));
  }
}

@media (max-width: 600px) {
  .left-sidebar {
    display: none;
  }
  
  .content-area {
    margin: 0;
    border-radius: 0;
    min-height: 100vh;
  }
}
</style>