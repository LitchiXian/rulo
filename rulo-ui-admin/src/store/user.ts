import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { UserInfo, LoginDto } from '@/type/user'
import authApi from '@/api/admin/auth'

export const useUserStore = defineStore(
  'admin-user',
  () => {
    const token = ref<string>('')
    const userInfo = ref<UserInfo | null>(null)
    const loading = ref(false)

    const isLoggedIn = computed(() => !!token.value)
    const userName = computed(() => userInfo.value?.nick_name || userInfo.value?.user_name || '')

    const login = async (credentials: LoginDto) => {
      loading.value = true
      try {
        token.value = (await authApi.login(credentials)) as unknown as string
      } finally {
        loading.value = false
      }
    }

    const initUser = async () => {
      if (!token.value) return
      userInfo.value = await authApi.getLoginInfo()
    }

    const logout = async () => {
      try {
        if (isLoggedIn.value) await authApi.logout()
      } finally {
        token.value = ''
        userInfo.value = null
      }
    }

    return { token, userInfo, loading, isLoggedIn, userName, login, initUser, logout }
  },
  {
    persist: {
      // 只持久化 token，key 与 request.ts 中的读取逻辑对应
      // v4 breaking change: paths → pick
      key: 'admin-user',
      pick: ['token'],
    },
  },
)
