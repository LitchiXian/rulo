import request from '@/util/request'

export interface ChatMessage {
  role: 'user' | 'assistant' | 'system'
  content: string
}

export interface ChatResponse {
  content: string
}

/** 非流式 AI 对话 */
export function chatComplete(messages: ChatMessage[]): Promise<ChatResponse> {
  return request({ url: '/ai/chat', method: 'post', data: { messages } }) as Promise<ChatResponse>
}

/**
 * 流式 AI 对话 (SSE)
 */
export async function chatStream(
  messages: ChatMessage[],
  onChunk: (content: string) => void,
  onDone: () => void,
  onError: (err: string) => void,
) {
  const token = getToken()
  const baseURL = request.defaults.baseURL || ''

  const resp = await fetch(`${baseURL}/ai/chat/stream`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { authorization: token } : {}),
    },
    body: JSON.stringify({ messages }),
  })

  if (!resp.ok) {
    onError(`请求失败: ${resp.status}`)
    return
  }

  const reader = resp.body?.getReader()
  if (!reader) {
    onError('无法读取响应流')
    return
  }

  const decoder = new TextDecoder()
  let buffer = ''

  while (true) {
    const { done, value } = await reader.read()
    if (done) break

    buffer += decoder.decode(value, { stream: true })

    const lines = buffer.split('\n')
    buffer = lines.pop() || ''

    for (const line of lines) {
      const trimmed = line.trim()
      if (!trimmed) continue

      if (trimmed.startsWith('data: ')) {
        const data = trimmed.slice(6)
        if (data === '[DONE]') {
          onDone()
          return
        }
        try {
          const parsed = JSON.parse(data)
          if (parsed.error) {
            onError(parsed.error)
            return
          }
          if (parsed.content) {
            onChunk(parsed.content)
          }
        } catch {
          // 忽略无法解析的行
        }
      }
    }
  }

  onDone()
}

function getToken(): string {
  try {
    const raw = localStorage.getItem('admin-user')
    if (!raw) return ''
    return JSON.parse(raw).token || ''
  } catch {
    return ''
  }
}
