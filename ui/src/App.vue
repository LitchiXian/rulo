<script setup>
import {useRoute} from 'vue-router'
import {computed} from 'vue'
import MainLayout from "@/layout/MainLayout.vue";
import EmptyLayout from "@/layout/EmptyLayout.vue";

const route = useRoute()
// 直接根据meta信息判断是否使用布局
const useLayout = computed(() => !route.meta.noLayout)
</script>

<template>
<!--  <router-view v-if="useLayout"/>-->

    <MainLayout v-if="useLayout">
      <router-view />
    </MainLayout>

    <EmptyLayout v-else>
      <router-view/>
    </EmptyLayout>

</template>

<style scoped>

</style>