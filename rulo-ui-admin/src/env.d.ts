/// <reference types="vite/client" />

// 让 TypeScript 能识别 .vue 文件导入（vue-tsc 会自动补全，这里是 IDE 兜底）
declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent
  export default component
}

// 扩展 import.meta.env 的类型
interface ImportMetaEnv {
  readonly VITE_APP_ENV: string
  readonly MODE: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}

// ⚠️ 必须有 export {} 使这个文件成为 "模块"
// 否则 declare module 'vue-router' 会变成 ambient 声明，替换掉整个 vue-router！
export {}

// 扩展 Vue Router 的 meta 类型，避免访问时需要 as any
declare module 'vue-router' {
  interface RouteMeta {
    /** 是否需要登录 */
    requiresAuth?: boolean
    /** 是否不使用布局（登录页/404 等） */
    noLayout?: boolean
    /** 页面标题 */
    title?: string
  }
}
