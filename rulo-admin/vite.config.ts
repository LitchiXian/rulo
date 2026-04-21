import { defineConfig, loadEnv } from 'vite'
import createVitePlugins from './vite/plugins'
import path from 'path'
import fs from 'fs'

// 读取 config.json 中 dev 的后端地址，作为代理目标
function getProxyTarget(): string {
  try {
    const raw = fs.readFileSync(path.resolve(__dirname, 'public/config.json'), 'utf-8')
    return JSON.parse(raw).dev.apiBaseUrl || 'http://127.0.0.1:3000'
  } catch {
    return 'http://127.0.0.1:3000'
  }
}

export default defineConfig(({ mode, command }) => {
  const env = loadEnv(mode, process.cwd())
  const proxyTarget = getProxyTarget()
  return {
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './src'),
      },
      extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue'],
    },
    plugins: createVitePlugins(env, command === 'build'),
    build: {
      sourcemap: command === 'build' ? false : 'inline',
      chunkSizeWarningLimit: 2000,
      rollupOptions: {
        output: {
          // 按模块类型分到对应子目录，部署时更清晰
          chunkFileNames: 'static/js/[name]-[hash].js',
          entryFileNames: 'static/js/[name]-[hash].js',
          assetFileNames: 'static/[ext]/[name]-[hash].[ext]',
        },
      },
    },
    server: {
      port: 9999,
      host: '0.0.0.0',
      strictPort: true,
      proxy: {
        '/api': {
          target: proxyTarget,
          changeOrigin: true,
          rewrite: (p) => p.replace(/^\/api/, ''),
        },
      },
    },
  }
})
