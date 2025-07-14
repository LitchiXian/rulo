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
          <span class="date">{{  formatDate(post.createTime) }}</span>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {list} from "@/api/web/bBlogArticle.js";

const router = useRouter()
const featuredPosts = ref([])
const loading = ref(true)
const error = ref(null)

// onMounted(() => {
//   featuredPosts.value = posts.slice(0, 5)
// })

onMounted(async () => {
  try {
    // 调用API获取文章列表
    const response = await list()

    // 从响应中提取data字段
    if (response.data && Array.isArray(response.data)) {
      featuredPosts.value = response.data
    } else {
      throw new Error('Invalid response format')
    }
  } catch (err) {
    console.error('Error fetching posts:', err)
    error.value = 'Failed to load posts. Please try again later.'
  } finally {
    loading.value = false
  }
})

/**
 * 将时间戳转换为格式化的日期时间字符串 (YYYY-MM-DD HH:mm)
 * @param {number} timestamp - 时间戳（毫秒）
 * @returns {string} 格式化后的日期时间字符串
 */
const formatDate = (timestamp) => {
  // 创建日期对象
  const date = new Date(timestamp);

  // 获取日期组件
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0'); // 月份从0开始
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');

  // 组合成格式化字符串
  return `${year}-${month}-${day} ${hours}:${minutes}`;
}
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