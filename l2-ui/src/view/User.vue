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
import {ref, onMounted} from 'vue'
import {useRoute} from 'vue-router'
import blogArticleApi from "@/api/web/blogArticle.ts";
import type {Article, Tag} from "@/type/article";
import {blogTagApi} from "@/api/web/blogTag.ts";

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
  margin-left: 10px;
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
}

.date::before {
  content: '📅';
  margin-right: 0.3rem;
}

.tags {
  margin-top: 0.8rem;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag {
  background-color: rgba(74, 187, 181, 0.15);
  color: #4abbb5;
  padding: 0.2rem 0.6rem;
  border-radius: 12px;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.tag:hover {
  background-color: rgba(74, 187, 181, 0.3);
  transform: translateY(-2px);
}

.tag.ellipsis {
  background-color: rgba(150, 150, 150, 0.15);
  color: #969696;
  cursor: default;
}

.tag.ellipsis:hover {
  background-color: rgba(150, 150, 150, 0.15);
  transform: none;
}

.name {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
}

.name::before {
  content: '👤';
  margin-right: 0.3rem;
}
</style>