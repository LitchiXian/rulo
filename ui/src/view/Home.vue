<template>
  <div class="home-content">
    <section class="hero">
      <h1>Welcome to Vue Blog</h1>
      <p>A simple markdown-based blog built with Vue 3 + Vite</p>
    </section>

    <hr>

    <section class="featured-posts">
      <h2>Latest Posts</h2>
      <div v-for="post in featuredPosts" :key="post.id" class="post-card">
        <router-link :to="`/post/${post.id}`">
          <h3>{{ post.title }}</h3>
        </router-link>
        <div class="meta">
          <span class="date">{{ smartFormatDate(post.createTime) }}</span>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import {ref, onMounted} from 'vue'
import {useRouter} from 'vue-router'
import {list} from "@/api/web/blogArticle.ts";
import type  {Article} from "@/type/article";

const router = useRouter()
const featuredPosts = ref<Article[]>([])
const loading = ref(true)
const error = ref<string>('')

onMounted(async () => {
  try {
    const response = await list();

    // 类型转换和验证
    const data = Array.isArray(response) ? response : [];
    featuredPosts.value = data.map(item => ({
      id: Number(item.id) || 0,
      title: item.title || '',
      content: '',
      createTime: Number(item.createTime) || Date.now(),
    })).slice(0, 5); // 取前5条作为特色文章

  } catch (err) {
    console.error('Error fetching posts:', err);
    error.value = 'Failed to load posts. Please try again later.';
  } finally {
    loading.value = false;
  }
});

/**
 * 智能格式化时间戳为易读格式
 * @param {number} timestamp - 时间戳（毫秒）
 * @returns {string} 格式化后的时间字符串
 */
const smartFormatDate = (timestamp: number) => {
  const now = new Date();
  const date = new Date(timestamp);

  // 计算今天的开始时间（0点0分0秒）
  const todayStart = new Date(now).setHours(0, 0, 0, 0);

  // 计算目标日期的开始时间（0点0分0秒）
  const targetDateStart = new Date(date).setHours(0, 0, 0, 0);

  // 计算日历天数差（基于天数的计算，而不是24小时周期）
  const dayDiff = Math.round((todayStart - targetDateStart) / (1000 * 60 * 60 * 24));

  // 获取时间组件
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');

  // 判断时间范围并返回对应格式
  if (dayDiff === 0) {
    // 今天：显示"今天 HH:mm"
    return `今天 ${hours}:${minutes}`;
  } else if (dayDiff === 1) {
    // 昨天：显示"昨天 HH:mm"
    return `昨天 ${hours}:${minutes}`;
  } else if (dayDiff <= 7) {
    // 7天内：显示"X天前"
    return `${dayDiff}天前`;
  } else if (date.getFullYear() === now.getFullYear()) {
    // 今年内：显示"MM-DD HH:mm"
    return `${month}-${day} ${hours}:${minutes}`;
  } else {
    // 往年：显示完整时间
    return `${year}-${month}-${day} ${hours}:${minutes}`;
  }
};
</script>

<style scoped>
.home-content {
  padding: 0 20px;
}

.hero {
  text-align: center;
  margin-bottom: 3rem;
  padding: 2rem;
  border-radius: 8px;
}

.hero h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  color: var(--title-color);
}

.hero p {
  font-size: 1.2rem;
  color: rgba(236, 226, 192, 0.8);
}

.featured-posts {
  border-radius: 8px;
  padding: 2rem;
}

.post-card {
  padding: 1.5rem 0;
  border-bottom: 1px solid var(--border-color);
}

.post-card:last-child {
  border-bottom: none;
}

.post-card h3 {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
  color: var(--title-color);
}

.excerpt {
  color: rgba(236, 226, 192, 0.8);
  margin-bottom: 0.8rem;
}

.meta {
  font-size: 0.9rem;
  color: rgba(236, 226, 192, 0.6);
}

.date {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
}

.date::before {
  content: '📅';
  margin-right: 0.3rem;
}
</style>