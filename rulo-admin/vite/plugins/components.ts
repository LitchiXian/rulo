import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

// 自动导入组件：Element Plus 组件 + src/components/ + src/layout/ 目录下的本地组件
// 效果：模板中可以直接用 <ElButton>、<AdminLayout> 等，无需手动 import
export default () => {
  return Components({
    resolvers: [ElementPlusResolver({ importStyle: false })],
    dirs: ['src/components', 'src/layout'],
    dts: 'src/components.d.ts',
  })
}
