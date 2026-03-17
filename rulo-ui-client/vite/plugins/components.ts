import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

// 自动导入组件：Element Plus 组件 + src/component/ + src/layout/ 目录下的本地组件
// 效果：模板中可以直接用 <ElButton>、<MainLayout> 等，无需手动 import
export default () => {
  return Components({
    resolvers: [ElementPlusResolver({ importStyle: false })],
    dirs: ['src/component', 'src/layout'],
    dts: 'src/components.d.ts',
  })
}
