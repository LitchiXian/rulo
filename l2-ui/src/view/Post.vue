<template>
  <div class="post-page">
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-container">
      <div class="loading">加载中...</div>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-container">
      <p>⚠️ {{ error }}</p>
      <button class="btn btn-primary" @click="retryFetch">重试</button>
    </div>

    <!-- 文章内容 -->
    <article v-else-if="currentPost" class="post-container">
      <header class="post-header">
        <h1 class="post-title">{{ currentPost.title }}</h1>
        <div class="post-meta">
          <span class="meta-item">
            <span>⏱️</span>
            {{ formatDate(currentPost.createTime, 'YYYY-MM-DD HH:mm') }}
          </span>
        </div>
      </header>

      <div class="post-content">
        <MarkdownRenderer
          :content="currentPost.content"
          @toc-generated="handleTocGenerated"
        />
      </div>

      <footer class="post-footer">
        <div class="tags">
          <span v-for="tag in getPostTags(currentPost.tags)" :key="tag.id" class="tag">
            {{ tag.name }}
          </span>
        </div>
      </footer>
    </article>

    <!-- 文章不存在 -->
    <div v-else class="not-found">
      <h2>文章不存在</h2>
      <p>抱歉，找不到您要查看的文章</p>
      <router-link to="/" class="btn btn-primary">返回首页</router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import MarkdownRenderer from '@/component/MarkdownRenderer.vue'
import blogArticleApi from "@/api/web/blogArticle.ts";
import type { Article, Tag } from "@/type/article.ts";
import { blogTagApi } from "@/api/web/blogTag.ts";
import { formatDate } from "@/util/dateFormat.ts";

const route = useRoute()
const currentPost = ref<Article>()
const loading = ref(true)
const error = ref<string | null>(null)
const tags = ref<Tag[]>([])

// 获取文章详情
const fetchArticle = async () => {
  try {
    loading.value = true
    error.value = null

    const id = Array.isArray(route.params.id) ? route.params.id[0] : route.params.id;
    currentPost.value = await blogArticleApi.getInfo({ id });
  } catch (err) {
    console.error('获取文章失败:', err);
    error.value = '加载文章失败，请稍后重试';
  } finally {
    loading.value = false
  }
}

const handleTocGenerated = (items: any[]) => {
  window.dispatchEvent(new CustomEvent('set-toc', { detail: items }));
}

// 监听路由参数变化
watch(() => route.params.id, (newId) => {
  if (newId) fetchArticle()
})

const retryFetch = () => {
  fetchArticle()
}

onMounted(async () => {
  await fetchArticle();
  await getTagList();

  window.addEventListener('set-toc', ((event: CustomEvent) => {
    if ((window as any).$mainLayout) {
      (window as any).$mainLayout.setTocItems(event.detail);
    }
  }) as EventListener);
});

const getTagList = async () => {
  tags.value = await blogTagApi.list();
}

const getPostTags = (tagIds: string) => {
  if (!tagIds) return [];
  const idArray = tagIds.split(',');
  return tags.value.filter(tag => idArray.includes(tag.id));
}
</script>

<style scoped>
.post-page {
  max-width: 100%;
}

.post-container {
  padding: var(--spacing-md);
}

.post-header {
  margin-bottom: var(--spacing-xl);
  padding-bottom: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
}

.post-title {
  font-size: 2rem;
  color: var(--text-title);
  margin: 0 0 var(--spacing-md);
  line-height: 1.3;
}

.post-meta {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
  color: var(--text-muted);
  font-size: 0.9rem;
}

.meta-item {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.post-content {
  font-size: 1.05rem;
  line-height: 1.8;
  color: var(--text-primary);
}

.post-footer {
  margin-top: var(--spacing-xl);
  padding-top: var(--spacing-lg);
  border-top: 1px solid var(--border-color);
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-sm);
}

/* 加载和错误状态 */
.loading-container {
  text-align: center;
  padding: var(--spacing-2xl) 0;
}

.error-container {
  text-align: center;
  padding: var(--spacing-xl);
  background-color: rgba(245, 108, 108, 0.1);
  border-radius: var(--radius-md);
  color: var(--color-danger);
}

.error-container p {
  margin-bottom: var(--spacing-md);
}

.not-found {
  text-align: center;
  padding: var(--spacing-2xl);
}

.not-found h2 {
  color: var(--text-title);
  margin-bottom: var(--spacing-md);
}

.not-found p {
  color: var(--text-secondary);
  margin-bottom: var(--spacing-lg);
}

/* 响应式 */
@media (max-width: 600px) {
  .post-title {
    font-size: 1.5rem;
  }
  
  .post-content {
    font-size: 1rem;
  }
}
</style>