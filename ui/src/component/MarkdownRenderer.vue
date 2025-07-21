<template>
  <div class="markdown-body" v-html="htmlContent"></div>
</template>

<script setup lang="ts">
import { defineProps, ref, onMounted } from 'vue'
import markdownParser from '@/util/markdownParser'
import '@/style/markdown.css'

const props = defineProps({
  content: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['toc-generated'])
const htmlContent = ref('')

onMounted(() => {
  const md = (markdownParser as (options: any) => any)({
    anchorOptions: {
      permalink: true,
      permalinkBefore: true,
      permalinkSymbol: '#'
    }
  });
  htmlContent.value = md.render(props.content)

  // 提取标题生成目录
  generateToc()
})

// 生成文章目录
const generateToc = () => {
  // 等待DOM更新
  setTimeout(() => {
    const tocItems = []
    const headings = document.querySelectorAll('.markdown-body h2, .markdown-body h3, .markdown-body h4')

    headings.forEach((heading, index) => {
      const level = parseInt(heading.tagName.substring(1))
      const id = `heading-${index}`
      heading.id = id

      tocItems.push({
        id: id,
        text: heading.innerText.replace('#', ''),
        level: level
      })
    })

    // 发送生成的目录
    emit('toc-generated', tocItems)
  }, 100)
}
</script>