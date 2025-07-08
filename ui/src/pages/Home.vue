<template>
  <MainLayout>
    <div class="container mx-auto px-4 py-8">
      <h1 class="text-3xl font-bold text-gray-800 mb-8">最新博客文章</h1>

      <!-- 搜索框 -->
      <div class="mb-6 flex">
        <input
            v-model="searchTerm"
            placeholder="搜索文章..."
            class="px-4 py-2 border border-gray-300 rounded-l-md w-full focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
            @click="searchPosts"
            class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-r-md"
        >
          搜索
        </button>
      </div>

      <!-- 标签筛选 -->
      <div class="mb-6 flex flex-wrap gap-2">
        <button
            v-for="tag in allTags"
            :key="tag"
            @click="toggleTag(tag)"
            :class="{'bg-blue-500 text-white': selectedTags.includes(tag), 'bg-gray-200 hover:bg-gray-300': !selectedTags.includes(tag)}"
            class="px-3 py-1 rounded-full text-sm transition-colors"
        >
          {{ tag }}
        </button>
        <button
            v-if="selectedTags.length"
            @click="clearFilters"
            class="text-sm text-blue-500 hover:text-blue-700 flex items-center ml-2"
        >
          清除筛选
        </button>
      </div>

      <!-- 文章列表 -->
      <div class="space-y-8">
        <div
            v-for="post in filteredPosts"
            :key="post.id"
            class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow p-6"
        >
          <div class="flex flex-col md:flex-row md:items-start">
            <!-- 文章信息 -->
            <div class="flex-1">
              <h2 class="text-xl font-bold text-gray-800 mb-2">
                <router-link :to="`/post/${post.id}`" class="hover:text-blue-600 transition-colors">
                  {{ post.title }}
                </router-link>
              </h2>

              <div class="text-gray-500 text-sm mb-4">
                <span>{{ formatDate(post.date) }}</span>
              </div>

              <p class="text-gray-600 mb-4 line-clamp-2">
                {{ getPreviewText(post.content) }}
              </p>

              <div class="flex flex-wrap gap-2">
                <span
                    v-for="tag in post.tags"
                    :key="tag"
                    class="bg-gray-100 text-gray-700 px-2 py-1 rounded text-xs"
                >
                  {{ tag }}
                </span>
              </div>
            </div>

            <!-- 阅读按钮 -->
            <div class="mt-4 md:mt-0 flex justify-end">
              <router-link
                  :to="`/post/${post.id}`"
                  class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition-colors"
              >
                阅读全文
              </router-link>
            </div>
          </div>
        </div>

        <!-- 无文章提示 -->
        <div v-if="filteredPosts.length === 0" class="text-center py-10">
          <p class="text-xl text-gray-500">没有找到匹配的文章</p>
          <button
              @click="clearFilters"
              class="mt-4 text-blue-500 hover:text-blue-700"
          >
            清除筛选条件
          </button>
        </div>
      </div>

      <!-- 回到顶部按钮 -->
      <button
          v-show="showScrollTop"
          @click="scrollToTop"
          class="fixed bottom-6 right-6 bg-blue-500 hover:bg-blue-600 text-white p-3 rounded-full shadow-lg transition-opacity"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import MainLayout from '@/layouts/MainLayout.vue';
import { useBlogStore } from '@/stores/blogStore';
import type { Post } from '@/stores/blogStore';

const blogStore = useBlogStore();
const searchTerm = ref('');
const selectedTags = ref<string[]>([]);
const showScrollTop = ref(false);

// 获取所有不重复的标签
const allTags = computed(() => {
  const tagsSet = new Set<string>();
  blogStore.posts.forEach(post => {
    post.tags.forEach(tag => tagsSet.add(tag));
  });
  return Array.from(tagsSet).sort();
});

// 获取过滤后的文章
const filteredPosts = computed((): Post[] => {
  if (!searchTerm.value && selectedTags.value.length === 0) {
    return [...blogStore.posts].sort((a, b) =>
        new Date(b.date).getTime() - new Date(a.date).getTime()
    );
  }

  return blogStore.posts.filter(post => {
    const matchesSearch =
        !searchTerm.value ||
        post.title.toLowerCase().includes(searchTerm.value.toLowerCase()) ||
        post.content.toLowerCase().includes(searchTerm.value.toLowerCase()) ||
        post.tags.some(tag => tag.toLowerCase().includes(searchTerm.value.toLowerCase()));

    const matchesTags =
        selectedTags.value.length === 0 ||
        selectedTags.value.every(tag => post.tags.includes(tag));

    return matchesSearch && matchesTags;
  }).sort((a, b) =>
      new Date(b.date).getTime() - new Date(a.date).getTime()
  );
});

// 日期格式化
const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });
};

// 获取文章预览文本（去掉 Markdown 标记）
const getPreviewText = (markdown: string): string => {
  // 移除标题标记
  const noHeaders = markdown.replace(/#+\s*/g, '');
  // 移除代码块
  const noCode = noHeaders.replace(/```[\s\S]*?```/g, '');
  // 移除图片
  const noImages = noCode.replace(/!\[.*?\]\(.*?\)/g, '');
  // 移除格式标记
  const cleanText = noImages.replace(/[*_~]/g, '');
  return cleanText.substring(0, 200) + '...';
};

// 标签筛选切换
const toggleTag = (tag: string) => {
  const index = selectedTags.value.indexOf(tag);
  if (index === -1) {
    selectedTags.value.push(tag);
  } else {
    selectedTags.value.splice(index, 1);
  }
};

// 清除所有筛选
const clearFilters = () => {
  searchTerm.value = '';
  selectedTags.value = [];
};

// 搜索文章
const searchPosts = () => {
  // 这里不需要额外的逻辑，computed 属性会自动更新
};

// 回到顶部功能
const scrollToTop = () => {
  window.scrollTo({
    top: 0,
    behavior: 'smooth'
  });
};

// 监听滚动
const handleScroll = () => {
  showScrollTop.value = window.scrollY > 300;
};

// 生命周期钩子
onMounted(() => {
  window.addEventListener('scroll', handleScroll);
});

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll);
});
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>