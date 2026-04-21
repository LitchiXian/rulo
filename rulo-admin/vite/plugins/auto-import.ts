import autoImport from 'unplugin-auto-import/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

// 自动导入 vue/vue-router/pinia API 以及 Element Plus 函数（ElMessage 等）
// 效果：组件中可以直接用 ref、computed、useRouter、ElMessage 等，无需手动 import
export default (path: any) => {
  return autoImport({
    imports: ['vue', 'vue-router', 'pinia'],
    resolvers: [ElementPlusResolver()],
    dts: path.resolve(__dirname, '../../src/auto-imports.d.ts'),
  })
}
