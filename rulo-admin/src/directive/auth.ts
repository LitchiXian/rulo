import type { Directive } from 'vue'
import { useUserStore } from '@/store/user'

/**
 * v-auth 权限指令
 *
 * 用法：
 *   v-auth="'sys:user:list'"           — 单个权限码
 *   v-auth="['sys:user:add','sys:user:edit']"  — 任一匹配即显示
 */
const auth: Directive<HTMLElement, string | string[]> = {
  mounted(el, binding) {
    check(el, binding.value)
  },
  updated(el, binding) {
    check(el, binding.value)
  },
}

function check(el: HTMLElement, value: string | string[]) {
  const userStore = useUserStore()
  const codes = Array.isArray(value) ? value : [value]
  const hasAuth = codes.some((code) => userStore.hasPerm(code))
  el.style.display = hasAuth ? '' : 'none'
}

export default auth
