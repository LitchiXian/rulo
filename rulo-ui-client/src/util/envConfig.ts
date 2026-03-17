import request from '@/util/request'

const envConfig = {
  service: {
    apiBaseUrl: '',
    debugMode: false,
  },
}

export default envConfig

export async function loadEnvConfig() {
  try {
    const response = await fetch(`/config.json?ts=${Date.now()}`)
    if (!response.ok) throw new Error(`配置加载失败: ${response.status}`)

    const configData = await response.json()
    const env = import.meta.env.MODE
    const configName = env === 'development' ? 'dev' : 'prod'

    Object.assign(envConfig.service, configData[configName])

    // dev 模式走 Vite 正向代理 /api → 后端地址，避免跨域
    // prod 模式直接用 config.json 中配置的地址（由 nginx 等处理）
    request.defaults.baseURL = env === 'development' ? '/api' : envConfig.service.apiBaseUrl

    console.log(`[envConfig] 已加载 ${configName} 环境:`, envConfig.service)
  } catch (error) {
    console.error('[envConfig] 加载失败:', error)
  }
}