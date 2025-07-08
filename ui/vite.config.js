import { defineConfig, loadEnv } from 'vite'
import path from "path"
import createVitePlugins from './vite/plugins'

// https://vite.dev/config/
export default defineConfig(({mode, command}) => {
    const env = loadEnv(mode, process.cwd())
    const {VITE_APP_ENV} = env
    return {
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
        plugins: createVitePlugins(env, command === 'build'),
        resolve: {
            // https://cn.vitejs.dev/config/#resolve-alias
            alias: {
                // 设置路径
                '~': path.resolve(__dirname, './'),
                // 设置别名
                '@': path.resolve(__dirname, './src')
            }
        }
    }
})
