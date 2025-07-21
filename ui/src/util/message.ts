import { ElMessage } from "element-plus";

type MessageType = 'success' | 'warning' | 'error';

interface MessageOptions {
    duration?: number;
    position?: string;
}

const message = (type: MessageType, content: string, options: MessageOptions = {}): void => {
    const { duration = 2000, position = 'top' } = options;

    ElMessage({
        type,
        message: content,
        duration,
        position
    });
};

// 导出快捷方法
export const msg = {
    success: (content: string, options: MessageOptions = {}) => message('success', content, options),
    warning: (content: string, options: MessageOptions = {}) => message('warning', content, options),
    error: (content: string, options: MessageOptions = {}) => message('error', content, options)
};
