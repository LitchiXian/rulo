import {defineConfig, loadEnv} from 'vite'
import createVitePlugins from './vite/plugins'
import path from "path";

// vite.config.ts 是 Vite 构建工具的核心配置文件，用于定制 Vite 的行为，确保项目能按需求高效开发、构建和运行。

// https://vite.dev/config/
export default defineConfig(({ mode, command }) =>{
  const env = loadEnv(mode, process.cwd());
  return {
    resolve: {
      // https://cn.vitejs.dev/config/#resolve-alias
      alias: {
        '@': path.resolve(__dirname, './src')
      },
      extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue']
    },
    plugins: createVitePlugins(env, command === 'build'),
    server: {
      port: 80,
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
  }
});
