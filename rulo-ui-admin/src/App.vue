<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed } from 'vue'

const route = useRoute()
// noLayout: true 的路由（登录页、404 等）使用 EmptyLayout，其余使用 AdminLayout
const useLayout = computed(() => !route.meta.noLayout)
</script>

<template>
  <AdminLayout v-if="useLayout">
    <router-view />
  </AdminLayout>
  <EmptyLayout v-else>
    <router-view />
  </EmptyLayout>
</template>
