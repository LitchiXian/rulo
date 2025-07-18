import {ElMessage} from "element-plus";

const message = (type, content, options = {}) => {
    const {duration = 2000, position = 'top'} = options;

    ElMessage({
        type,
        message: content,
        duration,
        position
    });
}

// 导出快捷方法
export const msg = {
    success: (content, options = {}) => message('success', content, options),
    warning: (content, options = {}) => message('warning', content, options),
    error: (content, options = {}) => message('error', content, options)
};