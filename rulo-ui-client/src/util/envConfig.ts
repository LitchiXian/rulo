import request from "@/util/request.ts";

const envConfig = {
    service: {
        apiBaseUrl: '',
        debugMode: false,
    },
};

// 统一批量导出
export default envConfig;

// 配置加载函数
export async function loadEnvConfig() {
    try {
        // 获取当前环境: 'development' 或 'production'
        // // @ts-ignore // 用于抑制报红
        // const env = import.meta.env.MODE;
        const env = (import.meta as any).env.MODE as string;
        const configName = env === 'development' ? 'dev' : 'prod';

        // 加载配置文件
        const response = await fetch(`/config.json?ts=${Date.now()}`);
        if (!response.ok) {
            throw new Error(`配置文件加载失败: ${response.status}`);
        }

        const configData = await response.json();

        // 更新envConfig对象
        Object.assign(envConfig.service, configData[configName]);

        console.log(`已加载 ${configName} 环境配置:`, envConfig.service);

        request.defaults.baseURL = envConfig.service.apiBaseUrl;
    } catch (error) {
        console.error('配置加载失败:', error);

        // 添加回退策略，如使用默认的配置 .env* 或 给出加载失败页面
    }
}