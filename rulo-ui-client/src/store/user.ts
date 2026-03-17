import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { UserInfo, LoginDto } from '@/type/user'
import authApi from '@/api/web/auth'

export const useUserStore = defineStore(
  'client-user',
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
      try {
        userInfo.value = await authApi.getLoginInfo()
      } catch {
        token.value = ''
        userInfo.value = null
      }
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
      key: 'client-user',
      pick: ['token'],
    },
  },
)