import type { Directive } from 'vue'
import { useUserStore } from '@/store/user'

/**
 * v-auth 权限指令
 *
 * 用法：
 *   v-auth="'sys:user:list'"                     — 单个权限码
 *   v-auth="['sys:user:add','sys:user:edit']"    — 任一匹配即显示（OR）
 *   v-auth.all="['sys:user:list-bind-roles','sys:user:update-bind-roles']"
 *                                                 — 必须全部匹配才显示（AND）
 */
const auth: Directive<HTMLElement, string | string[]> = {
  mounted(el, binding) {
    check(el, binding.value, !!binding.modifiers.all)
  },
  updated(el, binding) {
    check(el, binding.value, !!binding.modifiers.all)
  },
}

function check(el: HTMLElement, value: string | string[], all: boolean) {
  const userStore = useUserStore()
  const codes = Array.isArray(value) ? value : [value]
  const hasAuth = all
    ? codes.every((code) => userStore.hasPerm(code))
    : codes.some((code) => userStore.hasPerm(code))
  el.style.display = hasAuth ? '' : 'none'
}

export default auth
