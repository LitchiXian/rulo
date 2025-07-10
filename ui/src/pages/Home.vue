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
        <p class="excerpt">{{ post.excerpt }}</p>
        <div class="meta">
          <span class="date">{{ formatDate(post.date) }}</span>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import posts from '@/data/posts.json'

const router = useRouter()
const featuredPosts = ref([])

onMounted(() => {
  featuredPosts.value = posts.slice(0, 5)
})

const formatDate = (dateString) => {
  const options = { year: 'numeric', month: 'long', day: 'numeric' }
  return new Date(dateString).toLocaleDateString(undefined, options)
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