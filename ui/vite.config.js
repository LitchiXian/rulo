import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  server: {
    port: 5173,
    // 允许访问的域名
    host: '0.0.0.0',
    // 若端口被占用，直接退出
    strictPort: true,
    // 配置正向代理，用于处理跨域
    proxy: {
      '/api': {
        target: 'http://localhost:8090',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
  plugins: [vue()],
})
