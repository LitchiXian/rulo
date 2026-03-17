import { defineStore } from 'pinia'
import { ref, computed, watch, onUnmounted } from 'vue'

/** 布局模式 */
export type LayoutMode = 'left' | 'left-top-mix' | 'top' | 'right' | 'right-top-mix'
/** 主题模式 */
export type ThemeMode = 'light' | 'dark' | 'system'

export const useLayoutStore = defineStore(
  'admin-layout',
  () => {
    const mode = ref<LayoutMode>('left')
    const themeMode = ref<ThemeMode>('light')

    // 系统偏好监听
    const systemDarkQuery = window.matchMedia('(prefers-color-scheme: dark)')
    const systemPrefersDark = ref(systemDarkQuery.matches)
    const onSystemChange = (e: MediaQueryListEvent) => { systemPrefersDark.value = e.matches }
    systemDarkQuery.addEventListener('change', onSystemChange)
    onUnmounted(() => systemDarkQuery.removeEventListener('change', onSystemChange))

    const isDark = computed(() => {
      if (themeMode.value === 'system') return systemPrefersDark.value
      return themeMode.value === 'dark'
    })

    const setMode = (m: LayoutMode) => {
      mode.value = m
    }

    const setThemeMode = (m: ThemeMode) => {
      themeMode.value = m
    }

    const toggleDark = () => {
      // 循环切换: light -> dark -> light
      themeMode.value = isDark.value ? 'light' : 'dark'
    }

    // 同步 dark class 到 html 元素
    watch(isDark, (dark) => {
      document.documentElement.classList.toggle('dark', dark)
    }, { immediate: true })

    return { mode, themeMode, isDark, setMode, setThemeMode, toggleDark }
  },
  {
    persist: {
      key: 'admin-layout',
      pick: ['mode', 'themeMode'],
    },
  },
)
