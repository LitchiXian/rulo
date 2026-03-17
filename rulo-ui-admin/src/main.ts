import { createApp } from 'vue'
import App from './App.vue'
import router from './router/router'
import pinia from './store'
import { loadEnvConfig } from './util/envConfig'

// Element Plus 全量 CSS（组件由 unplugin-vue-components 按需注入，但样式需全量引入）
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import './style/base.css'

async function initApp() {
  // 先加载运行时配置，再启动应用，确保 baseURL 已设置
  await loadEnvConfig()

  const app = createApp(App)
  app.use(pinia)
  app.use(router)
  app.mount('#app')
}

initApp()
