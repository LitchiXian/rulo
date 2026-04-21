import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { UserInfoVo, MenuTreeNode, LoginDto } from '@/type/user'
import authApi from '@/api/admin/auth'

export const useUserStore = defineStore(
  'admin-user',
  () => {
    const token = ref<string>('')
    const userInfo = ref<UserInfoVo | null>(null)
    const perms = ref<string[]>([])
    const menus = ref<MenuTreeNode[]>([])
    const loading = ref(false)

    const isLoggedIn = computed(() => !!token.value)
    const userName = computed(() => userInfo.value?.nick_name || userInfo.value?.user_name || '')

    /** 判断当前用户是否拥有指定权限码 */
    const hasPerm = (code: string) => perms.value.includes(code)

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
      const info = await authApi.getLoginInfo()
      userInfo.value = info.user
      perms.value = info.perms
      menus.value = info.menus
    }

    const logout = async () => {
      try {
        if (isLoggedIn.value) await authApi.logout()
      } finally {
        token.value = ''
        userInfo.value = null
        perms.value = []
        menus.value = []
      }
    }

    return { token, userInfo, perms, menus, loading, isLoggedIn, userName, hasPerm, login, initUser, logout }
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
