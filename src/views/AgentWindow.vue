

<template>
  <div class="agent-window">
    <div class="agent-content">

      <div class="input-section">
        <label class="input-label">请输入您想要执行的操作：</label>
        <textarea
          v-model="instruction"
          class="instruction-input"
          placeholder="例如：打开 bilibili 网站"
          rows="4"
        ></textarea>
      </div>

      <div class="settings-section">
        <label class="settings-label">
          <span>最大执行步数：</span>
          <input
            v-model.number="maxSteps"
            type="number"
            min="1"
            max="50"
            class="steps-input"
          />
        </label>
        <span class="settings-hint">建议值：10-20 步</span>
      </div>

      <div class="hotkey-section">
        <label class="hotkey-label">
          <span>停止热键：</span>
          <select v-model="hotkey" class="hotkey-select">
            <option value="Escape">Escape (ESC)</option>
            <option value="F12">F12</option>
            <option value="F11">F11</option>
            <option value="F10">F10</option>
            <option value="F9">F9</option>
            <option value="F8">F8</option>
            <option value="F7">F7</option>
            <option value="F6">F6</option>
            <option value="F5">F5</option>
            <option value="F4">F4</option>
            <option value="F3">F3</option>
            <option value="F2">F2</option>
            <option value="F1">F1</option>
          </select>
        </label>
        <span class="hotkey-hint">执行时按此键停止</span>
      </div>

      <div class="button-group">
        <button
          class="execute-btn"
          :disabled="isRunning || !instruction.trim()"
          @click="executeAutomation"
        >
          {{ isRunning ? '执行中...' : '开始执行' }}
        </button>
        <button
          v-show="isRunning"
          class="stop-btn"
          @click="stopAutomation"
        >
          ⏹ 停止执行
        </button>
      </div>

      <div v-if="executionLog" class="log-section">
        <div class="log-header">
          <h3>执行日志</h3>
          <button class="clear-btn" @click="clearLog">清空</button>
        </div>
        <div class="log-content" ref="logContainer">
          <pre>{{ executionLog }}</pre>
        </div>
      </div>

      <div v-if="statusMessage" :class="['status-message', statusType]">
        {{ statusMessage }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'

const instruction = ref('')
const maxSteps = ref(15)
const isRunning = ref(false)
const executionLog = ref('')
const statusMessage = ref('')
const statusType = ref('info')
const logContainer = ref(null)
const hotkey = ref('Escape')

watch(executionLog, () => {
  nextTick(() => {
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  })
})

onMounted(async () => {
  try {
    const savedHotkey = await invoke('get_agent_hotkey')
    hotkey.value = savedHotkey
  } catch (error) {
    console.error('加载热键配置失败:', error)
  }

  try {
    const config = await invoke('get_backend_config')
    executionLog.value = `[配置信息] 后端地址: ${config.full_url}\n[配置加载完成]\n`
    console.log('后端配置:', config)
  } catch (error) {
    executionLog.value = `[配置信息] 获取配置失败: ${error}\n`
    console.error('获取配置失败:', error)
  }

  window.addEventListener('keydown', handleGlobalKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeydown)
})

async function handleGlobalKeydown(event) {
  if (!isRunning.value) return

  if (event.key === hotkey.value) {
    event.preventDefault()
    await stopAutomation()
  }
}

async function executeAutomation() {
  if (!instruction.value.trim()) {
    showStatus('请输入操作指令', 'error')
    return
  }

  try {
    await invoke('set_agent_hotkey', { hotkey: hotkey.value })
  } catch (error) {
    console.error('保存热键配置失败:', error)
  }

  isRunning.value = true
  executionLog.value = ''
  showStatus('正在启动 agent...', 'info')

  try {
    const log = await invoke('run_agent_automation', {
      instruction: instruction.value.trim(),
      maxStep: maxSteps.value
    })

    executionLog.value = log
    showStatus('自动化执行完成', 'success')
  } catch (error) {
    executionLog.value = `错误：${error}`
    showStatus(`执行失败：${error}`, 'error')
  } finally {
    isRunning.value = false
  }
}

async function stopAutomation() {
  if (!isRunning.value) {
    showStatus('Agent 未在运行', 'info')
    return
  }

  try {
    const result = await invoke('stop_agent_automation')
    showStatus(result, 'info')
    executionLog.value += '\n[系统] 用户请求停止，正在等待当前操作完成...\n'
  } catch (error) {
    showStatus(`停止失败：${error}`, 'error')
  }
}

function showStatus(message, type) {
  statusMessage.value = message
  statusType.value = type
  setTimeout(() => {
    statusMessage.value = ''
  }, 5000)
}

function clearLog() {
  executionLog.value = ''
}

async function closeWindow() {
  try {
    const appWindow = getCurrentWindow()
    await appWindow.close()
  } catch (e) {
    console.error('关闭窗口失败:', e)
  }
}
</script>

<style scoped>
.agent-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-primary, #0d1117);
  color: var(--text-primary, #f0f6fc);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

.agent-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.input-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary, #8b949e);
}

.instruction-input {
  width: 100%;
  padding: 12px;
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 2px;
  color: var(--text-primary, #f0f6fc);
  font-size: 14px;
  resize: vertical;
  font-family: inherit;
  box-sizing: border-box;
}

.instruction-input:focus {
  outline: none;
  border-color: var(--accent-blue, #3178c6);
}

.instruction-input::placeholder {
  color: var(--text-hint, #484f58);
}

.settings-section {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background-color: var(--bg-secondary, #161b22);
  border-radius: 2px;
  border: 1px solid var(--border-color, #30363d);
}

.settings-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary, #8b949e);
}

.steps-input {
  width: 60px;
  padding: 6px 8px;
  background-color: var(--bg-primary, #0d1117);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 2px;
  color: var(--text-primary, #f0f6fc);
  font-size: 14px;
  text-align: center;
}

.steps-input:focus {
  outline: none;
  border-color: var(--accent-blue, #3178c6);
}

.settings-hint {
  font-size: 12px;
  color: var(--text-hint, #484f58);
  margin-left: auto;
}

.hotkey-section {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background-color: var(--bg-secondary, #161b22);
  border-radius: 2px;
  border: 1px solid var(--border-color, #30363d);
}

.hotkey-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary, #8b949e);
}

.hotkey-select {
  padding: 6px 10px;
  background-color: var(--bg-primary, #0d1117);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 2px;
  color: var(--text-primary, #f0f6fc);
  font-size: 14px;
  cursor: pointer;
}

.hotkey-select:focus {
  outline: none;
  border-color: var(--accent-blue, #3178c6);
}

.hotkey-hint {
  font-size: 12px;
  color: var(--text-hint, #484f58);
  margin-left: auto;
}

.button-group {
  display: flex;
  gap: 12px;
}

.execute-btn {
  flex: 1;
  padding: 12px;
  background-color: var(--accent-blue, #3178c6);
  color: white;
  border: none;
  border-radius: 2px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.execute-btn:hover:not(:disabled) {
  background-color: #2868a8;
  box-shadow: 0 4px 12px rgba(49, 120, 198, 0.3);
}

.execute-btn:disabled {
  background-color: var(--bg-secondary, #161b22);
  color: var(--text-hint, #484f58);
  cursor: not-allowed;
  border: 1px solid var(--border-color, #30363d);
}

.stop-btn {
  flex: 1;
  padding: 12px;
  background-color: #da3633;
  color: white;
  border: none;
  border-radius: 2px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  animation: pulse 1.5s infinite;
}

.stop-btn:hover {
  background-color: #c2322f;
  box-shadow: 0 4px 12px rgba(218, 54, 51, 0.3);
}

@keyframes pulse {
  0%, 100% {
    box-shadow: 0 0 0 0 rgba(218, 54, 51, 0.4);
  }
  50% {
    box-shadow: 0 0 0 10px rgba(218, 54, 51, 0);
  }
}

.log-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 200px;
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 2px;
  overflow: hidden;
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background-color: var(--bg-tertiary, #21262d);
  border-bottom: 1px solid var(--border-color, #30363d);
}

.log-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
}

.clear-btn {
  padding: 4px 10px;
  background-color: transparent;
  border: 1px solid var(--border-color, #30363d);
  border-radius: 2px;
  color: var(--text-secondary, #8b949e);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.clear-btn:hover {
  background-color: var(--border-color, #30363d);
  color: var(--text-primary, #f0f6fc);
}

.log-content {
  flex: 1;
  padding: 12px;
  overflow-y: auto;
  background-color: var(--bg-primary, #0d1117);
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.5;
}

.log-content pre {
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
  color: var(--text-primary, #f0f6fc);
}

.status-message {
  padding: 12px;
  border-radius: 2px;
  font-size: 14px;
  text-align: center;
  animation: fadeIn 0.3s ease;
}

.status-message.info {
  background-color: rgba(49, 120, 198, 0.1);
  color: var(--accent-blue, #3178c6);
  border: 1px solid var(--accent-blue, #3178c6);
}

.status-message.success {
  background-color: rgba(63, 185, 80, 0.1);
  color: var(--accent-green, #3fb950);
  border: 1px solid var(--accent-green, #3fb950);
}

.status-message.error {
  background-color: rgba(218, 54, 51, 0.1);
  color: #da3633;
  border: 1px solid #da3633;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
