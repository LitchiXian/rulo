<template>
  <div class="home-content">
    <section class="hero">
      <h1>自嗨小角落</h1>
    </section>

    <hr>

    <section class="featured-posts">
      <h2>Latest Posts</h2>
      <div v-for="post in featuredPosts" :key="post.id" class="post-card">
        <router-link :to="`/post/${post.id}`">
          <h3>{{ post.title }}</h3>
        </router-link>
        <div class="meta">
          <span class="name">By {{ post.userName }}</span><span class="date">{{ smartFormatDate(post.createTime) }}</span>
        </div>
        <!-- 标签展示 -->
        <div class="tags">
          <span v-for="tag in getPostTags(post.tags).slice(0, 5)" :key="tag.id" class="tag">{{ tag.name }}</span>
          <span v-if="getPostTags(post.tags).length > 5" class="tag ellipsis">...</span>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import blogArticleApi from "@/api/web/blogArticle.ts";
import type { Article, Tag } from "@/type/article";
import { blogTagApi } from "@/api/web/blogTag.ts";
import { smartFormatDate } from "@/util/dateFormat.ts";

const route = useRoute()
const featuredPosts = ref<Article[]>([])
const loading = ref(true)
const error = ref<string>('')
const tags = ref<Tag[]>([])

onMounted(async() => {
  await getList();
  await getTagList();
});

const getList = async () => {
  try {

    loading.value = true;
    error.value = '';

    // 调用API接口获取数据
    const id = Array.isArray(route.params.id) ? route.params.id[0] : route.params.id;
    // currentPost.value = await blogArticleApi.get({id});

    featuredPosts.value = await blogArticleApi.getUserArticleList({id});

  } catch (err) {
    console.error('Error fetching posts:', err);
    error.value = 'Failed to load posts. Please try again later.';
  } finally {
    loading.value = false;
  }
}

const getTagList = async () => {
  tags.value = await blogTagApi.list();
}

// 根据文章的标签ID字符串获取标签对象
const getPostTags = (tagIds: string) => {
  if (!tagIds) return [];
  const idArray = tagIds.split(',');
  return tags.value.filter(tag => idArray.includes(tag.id));
}
</script>

<style scoped>
.home-content {
  padding: 0 var(--spacing-lg);
}

.hero {
  text-align: center;
  margin-bottom: var(--spacing-2xl);
  padding: var(--spacing-xl);
  border-radius: var(--radius-md);
}

.hero h1 {
  font-size: 2.5rem;
  margin-bottom: var(--spacing-md);
  color: var(--text-title);
}

.featured-posts {
  border-radius: var(--radius-md);
  padding: var(--spacing-xl);
}

.featured-posts h2 {
  color: var(--text-title);
  margin-bottom: var(--spacing-lg);
}

.post-card {
  padding: var(--spacing-lg) 0;
  border-bottom: 1px solid var(--border-color);
}

.post-card:last-child {
  border-bottom: none;
}

.post-card h3 {
  font-size: 1.5rem;
  margin-bottom: var(--spacing-sm);
  color: var(--text-title);
  transition: color var(--transition-fast);
}

.post-card h3:hover {
  color: var(--color-primary);
}

.meta {
  font-size: 0.9rem;
  color: var(--text-muted);
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
}

.name {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.name::before {
  content: '\1F464';
}

.date {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.date::before {
  content: '\1F4C5';
}

.tags {
  margin-top: var(--spacing-sm);
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-sm);
}

.tag.ellipsis {
  background-color: rgba(150, 150, 150, 0.15);
  color: var(--text-muted);
  cursor: default;
}

.tag.ellipsis:hover {
  background-color: rgba(150, 150, 150, 0.15);
  transform: none;
}

@media (max-width: 600px) {
  .hero h1 {
    font-size: 1.8rem;
  }
  
  .post-card h3 {
    font-size: 1.2rem;
  }
}
</style>