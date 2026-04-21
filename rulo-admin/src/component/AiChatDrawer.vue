<script setup lang="ts" name="AiChatDrawer">
import { ref, nextTick, computed, watch } from 'vue'
import { Promotion, Delete, Close } from '@element-plus/icons-vue'
import { chatComplete, chatStream } from '@/api/admin/ai'
import type { ChatMessage } from '@/api/admin/ai'
import { ElMessage } from 'element-plus'
import { marked } from 'marked'
import DOMPurify from 'dompurify'

marked.setOptions({ gfm: true, breaks: true })

function renderMarkdown(content: string, role: string): string {
  if (role === 'user') return DOMPurify.sanitize(content)
  return DOMPurify.sanitize(marked.parse(content) as string)
}

const visible = defineModel<boolean>({ default: false })

const messages = ref<ChatMessage[]>([])
const userInput = ref('')
const loading = ref(false)
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

watch(visible, (val) => {
  if (val) scrollToBottom()
})

async function sendMessage() {
  const content = userInput.value.trim()
  if (!content || loading.value) return

  messages.value.push({ role: 'user', content })
  userInput.value = ''
  scrollToBottom()
  loading.value = true

  if (outputMode.value === 'full') {
    await sendFull()
  } else {
    await sendStream()
  }

  loading.value = false
  scrollToBottom()
}

async function sendFull() {
  const apiMessages = buildApiMessages()
  try {
    const resp = await chatComplete(apiMessages)
    messages.value.push({ role: 'assistant', content: resp.content })
  } catch (e: any) {
    messages.value.push({ role: 'assistant', content: `❌ 请求失败: ${e.message || e}` })
  }
}

async function sendStream() {
  const apiMessages = buildApiMessages()
  messages.value.push({ role: 'assistant', content: '' })
  const idx = messages.value.length - 1
  try {
    await chatStream(
      apiMessages,
      (chunk) => {
        messages.value[idx].content += chunk
        scrollToBottom()
      },
      () => {},
      (err) => {
        messages.value[idx].content += `\n❌ ${err}`
      },
    )
  } catch (e: any) {
    messages.value[idx].content += `\n❌ 请求异常: ${e.message || e}`
  }
}

function buildApiMessages(): ChatMessage[] {
  return messages.value.slice(-20).map((m) => ({ role: m.role, content: m.content }))
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
  <Teleport to="body">
    <Transition name="ai-drawer">
      <div v-show="visible" class="ai-drawer-mask" @click.self="visible = false">
        <div class="ai-drawer-panel" @click.stop>
          <!-- 顶部 -->
          <div class="drawer-header">
            <div class="drawer-header-left">
              <span class="drawer-title">🤖 AI 助手</span>
              <el-tag type="info" size="small">GLM-4-Flash</el-tag>
            </div>
            <div class="drawer-header-right">
              <el-tooltip :content="outputMode === 'stream' ? '流式输出' : '全量输出'" placement="top">
                <el-switch
                  v-model="outputMode"
                  active-value="stream"
                  inactive-value="full"
                  active-text="流式"
                  inactive-text="全量"
                  :inline-prompt="true"
                  style="margin-right: 8px;"
                />
              </el-tooltip>
              <el-button :icon="Delete" text size="small" @click="clearMessages" :disabled="loading" />
              <el-button :icon="Close" text size="small" @click="visible = false" />
            </div>
          </div>

          <!-- 消息区 -->
          <div class="drawer-messages" ref="chatBoxRef">
            <div v-if="messages.length === 0" class="chat-empty">
              <div class="empty-icon">🤖</div>
              <p>你好！有什么可以帮助你的吗？</p>
              <p style="font-size: 12px; color: var(--el-text-color-secondary);">Enter 发送，Shift+Enter 换行</p>
            </div>
            <div
              v-for="(msg, index) in messages"
              :key="index"
              :class="['chat-message', msg.role === 'user' ? 'msg-user' : 'msg-assistant']"
            >
              <div class="msg-avatar">{{ msg.role === 'user' ? '🧑' : '🤖' }}</div>
              <div class="msg-bubble">
                <div
                  class="msg-content"
                  :class="{ 'md-body': msg.role === 'assistant' }"
                  v-html="renderMarkdown(msg.content, msg.role)"
                />
              </div>
            </div>
            <div v-if="loading && outputMode === 'full'" class="chat-message msg-assistant">
              <div class="msg-avatar">🤖</div>
              <div class="msg-bubble">
                <div class="msg-content typing-indicator"><span /><span /><span /></div>
              </div>
            </div>
          </div>

          <!-- 输入区 -->
          <div class="drawer-input">
            <el-input
              v-model="userInput"
              type="textarea"
              :autosize="{ minRows: 1, maxRows: 4 }"
              placeholder="输入消息..."
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
            />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
/* 遮罩 */
.ai-drawer-mask {
  position: fixed;
  inset: 0;
  z-index: 2000;
  display: flex;
  justify-content: flex-end;
}

/* 面板 */
.ai-drawer-panel {
  width: 420px;
  max-width: 100vw;
  height: 100vh;
  background: var(--el-bg-color);
  border-left: 1px solid var(--el-border-color-lighter);
  box-shadow: -4px 0 24px rgba(0, 0, 0, 0.12);
  display: flex;
  flex-direction: column;
}

/* 过渡动画 */
.ai-drawer-enter-active,
.ai-drawer-leave-active {
  transition: opacity 0.25s ease;
}
.ai-drawer-enter-active .ai-drawer-panel,
.ai-drawer-leave-active .ai-drawer-panel {
  transition: transform 0.28s cubic-bezier(0.4, 0, 0.2, 1);
}
.ai-drawer-enter-from,
.ai-drawer-leave-to {
  opacity: 0;
}
.ai-drawer-enter-from .ai-drawer-panel,
.ai-drawer-leave-to .ai-drawer-panel {
  transform: translateX(100%);
}

/* 头部 */
.drawer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--el-border-color-lighter);
  flex-shrink: 0;
}
.drawer-header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}
.drawer-title {
  font-size: 15px;
  font-weight: 600;
}
.drawer-header-right {
  display: flex;
  align-items: center;
}

/* 消息区 */
.drawer-messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 14px;
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
  font-size: 40px;
  margin-bottom: 8px;
}

.chat-message {
  display: flex;
  gap: 8px;
  max-width: 90%;
}
.msg-user {
  align-self: flex-end;
  flex-direction: row-reverse;
}
.msg-assistant {
  align-self: flex-start;
}
.msg-avatar {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  flex-shrink: 0;
  background: var(--el-fill-color-lighter);
}
.msg-bubble {
  padding: 8px 12px;
  border-radius: 10px;
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
  font-size: 13px;
}

/* Markdown */
.md-body { white-space: normal; }
.md-body :deep(h1), .md-body :deep(h2), .md-body :deep(h3), .md-body :deep(h4) { margin: 10px 0 4px; line-height: 1.4; }
.md-body :deep(h1) { font-size: 1.2em; }
.md-body :deep(h2) { font-size: 1.1em; }
.md-body :deep(p) { margin: 4px 0; }
.md-body :deep(ul), .md-body :deep(ol) { padding-left: 1.4em; margin: 4px 0; }
.md-body :deep(li) { margin: 2px 0; }
.md-body :deep(code) {
  background: var(--el-fill-color);
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 0.88em;
  font-family: 'Courier New', Courier, monospace;
}
.md-body :deep(pre) {
  background: var(--el-fill-color-dark);
  padding: 10px;
  border-radius: 6px;
  overflow-x: auto;
  margin: 6px 0;
}
.md-body :deep(pre code) { background: none; padding: 0; font-size: 0.85em; color: var(--el-text-color-primary); }
.md-body :deep(blockquote) { border-left: 3px solid var(--el-color-primary); padding-left: 10px; margin: 6px 0; color: var(--el-text-color-secondary); }
.md-body :deep(table) { border-collapse: collapse; width: 100%; margin: 6px 0; }
.md-body :deep(th), .md-body :deep(td) { border: 1px solid var(--el-border-color); padding: 4px 8px; text-align: left; }
.md-body :deep(th) { background: var(--el-fill-color-lighter); }
.md-body :deep(strong) { font-weight: 600; }
.md-body :deep(a) { color: var(--el-color-primary); }
.md-body :deep(hr) { border: none; border-top: 1px solid var(--el-border-color-lighter); margin: 8px 0; }

/* 打字动画 */
.typing-indicator { display: flex; gap: 4px; padding: 4px 0; }
.typing-indicator span { width: 7px; height: 7px; background: var(--el-text-color-secondary); border-radius: 50%; animation: typing 1.2s infinite; }
.typing-indicator span:nth-child(2) { animation-delay: 0.2s; }
.typing-indicator span:nth-child(3) { animation-delay: 0.4s; }
@keyframes typing {
  0%, 60%, 100% { opacity: 0.3; transform: scale(0.8); }
  30% { opacity: 1; transform: scale(1); }
}

/* 输入区 */
.drawer-input {
  display: flex;
  gap: 8px;
  padding: 12px 16px;
  border-top: 1px solid var(--el-border-color-lighter);
  align-items: flex-end;
}
.drawer-input :deep(.el-textarea__inner) {
  box-shadow: none;
  border: 1px solid var(--el-border-color);
  border-radius: 8px;
  padding: 6px 10px;
}
.send-btn {
  height: 34px;
  width: 34px;
  border-radius: 8px;
  flex-shrink: 0;
}

@media (max-width: 480px) {
  .ai-drawer-panel {
    width: 100vw;
  }
}
</style>
