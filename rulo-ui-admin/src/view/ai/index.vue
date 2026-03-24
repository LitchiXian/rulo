<script setup lang="ts" name="AiChat">
import { ref, nextTick, computed } from 'vue'
import { Promotion, Delete, Switch } from '@element-plus/icons-vue'
import { chatComplete, chatStream } from '@/api/admin/ai'
import type { ChatMessage } from '@/api/admin/ai'
import { ElMessage } from 'element-plus'
import { marked } from 'marked'

// marked 配置：启用 GFM、换行识别
marked.setOptions({
  gfm: true,
  breaks: true,
})

/** 将 markdown 文本渲染为 HTML（对用户消息原样输出） */
function renderMarkdown(content: string, role: string): string {
  if (role === 'user') return content
  return marked.parse(content) as string
}

// 对话消息列表
const messages = ref<ChatMessage[]>([])
// 用户输入
const userInput = ref('')
// 是否正在请求
const loading = ref(false)
// 输出模式: stream / full
const outputMode = ref<'stream' | 'full'>('stream')

const chatBoxRef = ref<HTMLDivElement>()

const canSend = computed(() => userInput.value.trim() && !loading.value)

function scrollToBottom() {
  nextTick(() => {
    if (chatBoxRef.value) {
      chatBoxRef.value.scrollTop = chatBoxRef.value.scrollHeight
    }
  })
}

async function sendMessage() {
  const content = userInput.value.trim()
  if (!content || loading.value) return

  // 添加用户消息
  messages.value.push({ role: 'user', content })
  userInput.value = ''
  scrollToBottom()

  loading.value = true

  if (outputMode.value === 'full') {
    await sendFull(content)
  } else {
    await sendStream(content)
  }

  loading.value = false
  scrollToBottom()
}

/** 全量输出模式 */
async function sendFull(content: string) {
  // 构造发给 API 的 messages (保留历史上下文)
  const apiMessages = buildApiMessages()

  try {
    const resp = await chatComplete(apiMessages)
    messages.value.push({ role: 'assistant', content: resp.content })
  } catch (e: any) {
    messages.value.push({ role: 'assistant', content: `❌ 请求失败: ${e.message || e}` })
  }
}

/** 流式输出模式 */
async function sendStream(content: string) {
  const apiMessages = buildApiMessages()

  // 先添加一个空的 assistant 消息，逐步填充
  messages.value.push({ role: 'assistant', content: '' })
  const idx = messages.value.length - 1

  try {
    await chatStream(
      apiMessages,
      (chunk) => {
        messages.value[idx].content += chunk
        scrollToBottom()
      },
      () => {
        // done
      },
      (err) => {
        messages.value[idx].content += `\n❌ ${err}`
      },
    )
  } catch (e: any) {
    messages.value[idx].content += `\n❌ 请求异常: ${e.message || e}`
  }
}

function buildApiMessages(): ChatMessage[] {
  // 发送最近 20 条消息作为上下文
  return messages.value.slice(-20).map((m) => ({
    role: m.role,
    content: m.content,
  }))
}

function clearMessages() {
  messages.value = []
  ElMessage.success('对话已清空')
}

function handleKeydown(e: KeyboardEvent | Event) {
  if (!(e instanceof KeyboardEvent)) return
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    sendMessage()
  }
}
</script>

<template>
  <div class="ai-chat-container">
    <!-- 顶部工具栏 -->
    <div class="chat-toolbar">
      <div class="toolbar-left">
        <h3 style="margin: 0; font-size: 16px;">🤖 AI 助手</h3>
        <el-tag type="info" size="small" style="margin-left: 8px;">GLM-4-Flash</el-tag>
      </div>
      <div class="toolbar-right">
        <el-tooltip :content="outputMode === 'stream' ? '当前: 流式输出' : '当前: 全量输出'" placement="top">
          <el-switch
            v-model="outputMode"
            active-value="stream"
            inactive-value="full"
            active-text="流式"
            inactive-text="全量"
            :inline-prompt="true"
            style="margin-right: 12px;"
          />
        </el-tooltip>
        <el-button :icon="Delete" text @click="clearMessages" :disabled="loading">清空对话</el-button>
      </div>
    </div>

    <!-- 消息列表 -->
    <div class="chat-messages" ref="chatBoxRef">
      <div v-if="messages.length === 0" class="chat-empty">
        <div class="empty-icon">🤖</div>
        <p>你好！我是 AI 助手，有什么可以帮助你的吗？</p>
        <p style="font-size: 12px; color: var(--el-text-color-secondary);">支持流式/全量两种输出模式，右上角可切换</p>
      </div>
      <div
        v-for="(msg, index) in messages"
        :key="index"
        :class="['chat-message', msg.role === 'user' ? 'msg-user' : 'msg-assistant']"
      >
        <div class="msg-avatar">
          {{ msg.role === 'user' ? '🧑' : '🤖' }}
        </div>
        <div class="msg-bubble">
          <div
            class="msg-content"
            :class="{ 'md-body': msg.role === 'assistant' }"
            v-html="renderMarkdown(msg.content, msg.role)"
          />
        </div>
      </div>
      <!-- 加载指示器 -->
      <div v-if="loading && outputMode === 'full'" class="chat-message msg-assistant">
        <div class="msg-avatar">🤖</div>
        <div class="msg-bubble">
          <div class="msg-content typing-indicator">
            <span></span><span></span><span></span>
          </div>
        </div>
      </div>
    </div>

    <!-- 输入框 -->
    <div class="chat-input-area">
      <el-input
        v-model="userInput"
        type="textarea"
        :autosize="{ minRows: 1, maxRows: 5 }"
        placeholder="输入消息，Enter 发送，Shift+Enter 换行"
        @keydown="handleKeydown"
        :disabled="loading"
        resize="none"
      />
      <el-button
        type="primary"
        :icon="Promotion"
        :loading="loading"
        :disabled="!canSend"
        @click="sendMessage"
        class="send-btn"
      >
        发送
      </el-button>
    </div>
  </div>
</template>

<style scoped>
.ai-chat-container {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 120px);
  min-height: 400px;
  background: var(--el-bg-color);
  border-radius: 8px;
  border: 1px solid var(--el-border-color-lighter);
  overflow: hidden;
}

.chat-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  border-bottom: 1px solid var(--el-border-color-lighter);
  background: var(--el-bg-color-overlay);
}

.toolbar-left,
.toolbar-right {
  display: flex;
  align-items: center;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.chat-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  color: var(--el-text-color-secondary);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.chat-message {
  display: flex;
  gap: 10px;
  max-width: 80%;
}

.msg-user {
  align-self: flex-end;
  flex-direction: row-reverse;
}

.msg-assistant {
  align-self: flex-start;
}

.msg-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  flex-shrink: 0;
  background: var(--el-fill-color-lighter);
}

.msg-bubble {
  padding: 10px 14px;
  border-radius: 12px;
  line-height: 1.6;
  word-break: break-word;
}

.msg-user .msg-bubble {
  background: var(--el-color-primary);
  color: #fff;
  border-bottom-right-radius: 4px;
}

.msg-assistant .msg-bubble {
  background: var(--el-fill-color-lighter);
  color: var(--el-text-color-primary);
  border-bottom-left-radius: 4px;
}

.msg-content {
  white-space: pre-wrap;
  font-size: 14px;
}

/* Markdown 内容样式 */
.md-body {
  white-space: normal;
}
.md-body :deep(h1),
.md-body :deep(h2),
.md-body :deep(h3),
.md-body :deep(h4) {
  margin: 12px 0 6px;
  line-height: 1.4;
}
.md-body :deep(h1) { font-size: 1.3em; }
.md-body :deep(h2) { font-size: 1.15em; }
.md-body :deep(h3) { font-size: 1.05em; }
.md-body :deep(p) {
  margin: 6px 0;
}
.md-body :deep(ul),
.md-body :deep(ol) {
  padding-left: 1.5em;
  margin: 6px 0;
}
.md-body :deep(li) {
  margin: 2px 0;
}
.md-body :deep(code) {
  background: var(--el-fill-color);
  padding: 2px 5px;
  border-radius: 3px;
  font-size: 0.9em;
  font-family: 'Courier New', Courier, monospace;
}
.md-body :deep(pre) {
  background: var(--el-fill-color-dark);
  padding: 12px;
  border-radius: 6px;
  overflow-x: auto;
  margin: 8px 0;
}
.md-body :deep(pre code) {
  background: none;
  padding: 0;
  font-size: 0.85em;
  color: var(--el-text-color-primary);
}
.md-body :deep(blockquote) {
  border-left: 3px solid var(--el-color-primary);
  padding-left: 12px;
  margin: 8px 0;
  color: var(--el-text-color-secondary);
}
.md-body :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin: 8px 0;
}
.md-body :deep(th),
.md-body :deep(td) {
  border: 1px solid var(--el-border-color);
  padding: 6px 10px;
  text-align: left;
}
.md-body :deep(th) {
  background: var(--el-fill-color-lighter);
}
.md-body :deep(strong) {
  font-weight: 600;
}
.md-body :deep(a) {
  color: var(--el-color-primary);
}
.md-body :deep(hr) {
  border: none;
  border-top: 1px solid var(--el-border-color-lighter);
  margin: 10px 0;
}

/* 打字动画 */
.typing-indicator {
  display: flex;
  gap: 4px;
  padding: 4px 0;
}
.typing-indicator span {
  width: 8px;
  height: 8px;
  background: var(--el-text-color-secondary);
  border-radius: 50%;
  animation: typing 1.2s infinite;
}
.typing-indicator span:nth-child(2) { animation-delay: 0.2s; }
.typing-indicator span:nth-child(3) { animation-delay: 0.4s; }

@keyframes typing {
  0%, 60%, 100% { opacity: 0.3; transform: scale(0.8); }
  30% { opacity: 1; transform: scale(1); }
}

.chat-input-area {
  display: flex;
  gap: 10px;
  padding: 16px 20px;
  border-top: 1px solid var(--el-border-color-lighter);
  background: var(--el-bg-color-overlay);
  align-items: flex-end;
}

.chat-input-area :deep(.el-textarea__inner) {
  box-shadow: none;
  border: 1px solid var(--el-border-color);
  border-radius: 8px;
  padding: 8px 12px;
}

.send-btn {
  height: 38px;
  border-radius: 8px;
}
</style>
