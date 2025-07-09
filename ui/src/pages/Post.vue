<template>
  <div v-if="currentPost" class="post-container">
    <article class="post">
      <header class="post-header">
        <h1 class="post-title">{{ currentPost.title }}</h1>
        <div class="post-subtitle">{{ currentPost.subtitle }}</div>

        <div class="post-meta">
          <span class="post-date">{{ formatDate(currentPost.date) }}</span>
          <span class="post-read-time">⏱️ {{ currentPost.readTime }} 分钟阅读</span>
        </div>
      </header>

      <div class="post-content">
        <MarkdownRenderer
            :content="currentPost.content"
            @toc-generated="handleTocGenerated"
        />
      </div>
    </article>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import MarkdownRenderer from '@/components/MarkdownRenderer.vue'
import posts from '@/data/posts.json'

const route = useRoute()
const currentPost = computed(() => {
  return posts.find(post => post.id === route.params.id)
})

const handleTocGenerated = (items) => {
  // 设置TOC目录项
  window.dispatchEvent(new CustomEvent('set-toc', { detail: items }));
}

const formatDate = (dateString) => {
  const options = { year: 'numeric', month: 'long', day: 'numeric' }
  return new Date(dateString).toLocaleDateString('zh-CN', options)
}

onMounted(() => {
  // 监听设置TOC事件
  window.addEventListener('set-toc', (event) => {
    if (window.$mainLayout) {
      window.$mainLayout.setTocItems(event.detail);
    }
  });

  // 保存布局组件的引用以便访问其方法
  if (window.$mainLayout) {
    window.$mainLayout = null;
  }
});
</script>

<style scoped>
.post-container {
  max-width: 900px;
  margin: 0 auto;
}

.post-header {
  margin-bottom: 40px;
}

.post-title {
  font-size: 2.25rem;
  color: #e2e2ec; /* 226,226,236 - 标题颜色 */
  margin-bottom: 15px;
  line-height: 1.2;
}

.post-subtitle {
  font-size: 1.25rem;
  color: #a0a0a0;
  margin-bottom: 25px;
}

.post-meta {
  display: flex;
  gap: 20px;
  color: #8e8e8e;
  font-size: 0.95rem;
}

.post-date, .post-read-time {
  display: flex;
  align-items: center;
}

.post-content {
  font-size: 1.1rem;
  line-height: 1.8;
}
</style>