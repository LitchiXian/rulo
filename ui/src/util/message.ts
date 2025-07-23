import { ElMessage } from "element-plus";

import type { MessageParams } from "element-plus";

export function showMessage(
    message: string = '提示内容',
    type: string = 'success',
    customClass: string = ''
) {
    return ElMessage({
        type: type,
        message: message,
        customClass: customClass,
    } as MessageParams);
}