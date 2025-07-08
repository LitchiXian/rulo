<template>
  <MainLayout>
    <div v-if="post">
      <h1 class="text-3xl font-bold mb-4">{{ post.title }}</h1>
      <div class="flex items-center mb-6">
        <span class="text-gray-500 mr-4">{{ post.date }}</span>
        <div class="flex gap-2">
          <span v-for="tag in post.tags" :key="tag"
                class="bg-blue-100 text-blue-800 px-2 py-1 rounded text-xs">
            {{ tag }}
          </span>
        </div>
      </div>

      <MarkdownRenderer :content="post.content" />
    </div>

    <div v-else class="text-center py-20">
      <p class="text-2xl">文章不存在或已被移除</p>
      <router-link to="/" class="text-blue-500 mt-4 inline-block">返回首页</router-link>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { useBlogStore } from '@/stores/blogStore';
import MarkdownRenderer from '@/components/MarkdownRenderer.vue';
import MainLayout from '@/layouts/MainLayout.vue';

const route = useRoute();
const blogStore = useBlogStore();

const post = computed(() =>
    blogStore.getPostById(Number(route.params.id))
);
</script>