import { ElMessage } from "element-plus";

// 定义 ElMessage 支持的消息类型（根据 Element Plus 文档）
type MessageType = 'info' | 'success' | 'warning' | 'error';

/**
 * 通用消息提示函数
 * @param type 消息类型（info/success/warning/error）
 * @param content 消息内容
 */
const message = (type: MessageType, content: string): void => {
    ElMessage({
        type,       // 消息类型（对应 ElMessage 的 type 属性）
        message: content,  // 消息内容（对应 ElMessage 的 message 属性）
        // 可选：添加其他通用配置（如 duration、showClose 等）
        // duration: 3000,  // 自动关闭时间（毫秒）
        // showClose: true  // 显示关闭按钮
    });
};

// 导出快捷方法（类型安全）
export const msg = {
    success: (content: string) => message('success', content),  // 修复多余逗号
    warning: (content: string) => message('warning', content),
    error: (content: string) => message('error', content),
    // 可选：补充 info 类型（如果需要）
    info: (content: string) => message('info', content)
};