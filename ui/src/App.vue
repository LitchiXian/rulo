<script setup>
import { useRoute } from 'vue-router'
import { computed } from 'vue'
import MainLayout from "@/layouts/MainLayout.vue";

const route = useRoute()
// 直接根据meta信息判断是否使用布局
const useLayout = computed(() => !route.meta.noLayout)
</script>

<template>
  <MainLayout v-if="useLayout">
    <router-view />
  </MainLayout>

  <div v-else class="full-page">
    <router-view />
  </div>
</template>

<style scoped>
.full-page {
  min-height: 100vh;
}
</style>