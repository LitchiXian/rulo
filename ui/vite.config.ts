import {defineConfig, loadEnv} from 'vite'
import createVitePlugins from './vite/plugins'
import path from "path";

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
      port: 10086,
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
