<template>
  <div class="markdown-body" v-html="compiledMarkdown" />
</template>

<script setup lang="ts">
import { marked } from 'marked';
import { computed } from 'vue';
import hljs from 'highlight.js';

const props = defineProps({
  content: {
    type: String,
    required: true
  }
});

const compiledMarkdown = computed(() => {
  return marked(props.content, {
    highlight: (code) => hljs.highlightAuto(code).value
  });
});
</script>

<style>
/* 基础Markdown样式 */
.markdown-body {
  line-height: 1.6;
  h1, h2, h3 {
    @apply mt-8 mb-4 font-bold;
  }
  h1 { @apply text-3xl; }
  h2 { @apply text-2xl; }
  h3 { @apply text-xl; }
  code { @apply bg-gray-100 px-1 py-0.5 rounded; }
  pre { @apply bg-gray-100 p-4 rounded overflow-auto; }
}
</style>