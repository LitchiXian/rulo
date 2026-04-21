import { ElMessage } from 'element-plus'

type MessageType = 'success' | 'warning' | 'error' | 'info'

export function showMessage(msg: string, type: MessageType = 'info') {
  ElMessage({ message: msg, type })
}
