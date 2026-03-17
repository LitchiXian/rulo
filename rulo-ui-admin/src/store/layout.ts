import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

/** 布局模式 */
export type LayoutMode = 'left' | 'left-top-mix' | 'top' | 'right' | 'right-top-mix'

export const useLayoutStore = defineStore(
  'admin-layout',
  () => {
    const mode = ref<LayoutMode>('left')
    const isDark = ref(false)

    const setMode = (m: LayoutMode) => {
      mode.value = m
    }

    const applyDark = (dark: boolean) => {
      document.documentElement.classList.toggle('dark', dark)
    }

    const toggleDark = () => {
      isDark.value = !isDark.value
      applyDark(isDark.value)
    }

    // 同步 dark class 到 html 元素
    watch(isDark, (dark) => {
      applyDark(dark)
    }, { immediate: true })

    return { mode, isDark, setMode, toggleDark }
  },
  {
    persist: {
      key: 'admin-layout',
      pick: ['mode', 'isDark'],
    },
  },
)
