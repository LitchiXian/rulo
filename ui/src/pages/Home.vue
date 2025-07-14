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
          <span class="date">{{  smartFormatDate(post.createTime) }}</span>
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
 * 智能格式化时间戳为易读格式
 * @param {number} timestamp - 时间戳（毫秒）
 * @returns {string} 格式化后的时间字符串
 */
const smartFormatDate = (timestamp) => {
  const now = new Date();
  const date = new Date(timestamp);

  // 时间差计算（毫秒）
  const diffMs = now - date;
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  // 获取时间组件
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');

  // 判断时间范围并返回对应格式
  if (diffMs < 0) {
    // 未来时间：显示完整时间
    return `${year}-${month}-${day} ${hours}:${minutes}`;
  } else if (diffDays === 0) {
    // 今天：显示"今天 HH:mm"
    return `今天 ${hours}:${minutes}`;
  } else if (diffDays === 1) {
    // 昨天：显示"昨天 HH:mm"
    return `昨天 ${hours}:${minutes}`;
  } else if (diffDays <= 7) {
    // 7天内：显示"X天前"
    return `${diffDays}天前`;
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