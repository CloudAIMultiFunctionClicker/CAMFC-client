<template>
  <div class="settings-page">
    <main class="settings-content">
      <div class="settings-panel help-panel">
        <h3>帮助与反馈</h3>
        <div class="setting-card">
          <div class="feedback-container">
            <div class="feedback-options" v-if="showFeedbackOptions">
              <button class="action-btn" @click="openGitHubIssue">
                <i class="ri-github-line"></i>
                <span>GitHub Issue</span>
              </button>
              <button class="action-btn" @click="openEmail">
                <i class="ri-mail-line"></i>
                <span>发送邮件</span>
              </button>
              <div class="email-container">
                <span class="email-address" @click="switchEmail" title="点击切换邮箱">
                  {{ developerEmails[currentEmailIndex] }}
                </span>
                <button class="copy-btn" @click="copyEmail" title="复制邮箱">
                  <i class="ri-file-copy-line"></i>
                </button>
              </div>
            </div>
            <div class="feedback-actions">
              <button class="action-btn" @click="showFeedbackOptions = true" v-if="!showFeedbackOptions">提交问题或反馈</button>
              <button class="action-btn cancel-btn" @click="showFeedbackOptions = false" v-else>返回</button>
            </div>
          </div>
          <button class="action-btn" @click="showFaq">常见问题 (FAQ)</button>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { openUrl } from '@tauri-apps/plugin-opener'

const showFeedbackOptions = ref(false)

const developerEmails = [
  'admin@mc666.top',
  'ANTmmmmm@outlook.com',
  'abc.cxh2009@foxmail.com',
  '1220594170@qq.com'
]
const currentEmailIndex = ref(0)

const showFaq = () => {

}

const openGitHubIssue = () => {
  openUrl('https://github.com/CloudAIMultiFunctionClicker/CAMFC-client/issues/')
}

const openEmail = () => {
  const subject = encodeURIComponent('CAMFC Cloud 客户端反馈')
  const body = encodeURIComponent('您好，我有一些反馈想与您分享：\n\n')
  window.location.href = `mailto:abc.cxh2009@foxmail.com?subject=${subject}&body=${body}`
}

const copyEmail = async () => {
  try {
    const email = developerEmails[currentEmailIndex.value]
    await navigator.clipboard.writeText(email)
    showToast('邮箱复制成功', '#10b981')
  } catch (e) {
    console.error('复制邮箱失败:', e)
    showToast('复制失败', '#ef4444')
  }
}

const switchEmail = () => {
  currentEmailIndex.value = (currentEmailIndex.value + 1) % developerEmails.length
}
</script>

<style scoped>
.settings-page {
  display: flex;
  height: 100%;
  background-color: var(--bg-primary, #ffffff);
  overflow: hidden;
}

.settings-content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
  background-color: var(--bg-primary, #ffffff);
  height: 100%;
}

.settings-panel {
  width: 100%;
  max-width: 800px;
}

.settings-panel h3 {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 24px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
}

.setting-card {
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  padding: 20px;
  margin-bottom: 16px;
}

.help-panel {
  max-width: 100%;
  height: calc(100vh - 150px);
}

.help-panel h3 {
  margin-bottom: 16px;
}

.feedback-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.feedback-options {
  display: flex;
  flex-direction: row;
  gap: 12px;
  padding: 12px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  animation: fadeIn 0.3s ease-out;
  align-items: center;
}

.feedback-options .action-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  background-color: var(--bg-secondary, #ffffff);
  color: var(--text-primary, #333);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.feedback-options .action-btn:hover {
  background-color: var(--hover-bg, #f5f5f5);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.feedback-options .action-btn i {
  font-size: 14px;
}

.email-container {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 3px;
}

.email-address {
  font-size: 13px;
  color: var(--text-secondary, #57606a);
  font-family: inherit;
  white-space: nowrap;
  padding: 8px 0;
  margin-top: 13px;
  cursor: pointer;
  transition: color 0.2s ease;
}

.email-address:hover {
  color: var(--accent-blue, #0969da);
}

.copy-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  margin-top: 11px;
  background-color: transparent;
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  color: var(--text-secondary, #57606a);
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.copy-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
  border-color: var(--accent-blue, #0969da);
}

.copy-btn i {
  font-size: 14px;
}

.cancel-btn {
  width: 100%;
  background-color: transparent;
  border: 1px solid var(--border-color, #ddd);
  color: var(--text-secondary, #666);
  margin-top: 8px;
}

.cancel-btn:hover {
  background-color: var(--hover-bg, #f5f5f5);
  color: var(--text-primary, #333);
}

.feedback-container .action-btn {
  width: auto;
  max-width: 200px;
}

.feedback-actions {
  display: flex;
  justify-content: flex-start;
  gap: 12px;
  margin-top: 0;
}

.feedback-actions .action-btn {
  padding: 10px 20px;
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  width: auto;
  max-width: none;
  flex: none;
}

.feedback-actions .action-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
}

.feedback-actions .cancel-btn {
  background-color: transparent;
  color: var(--text-secondary, #666);
}

.feedback-actions .cancel-btn:hover {
  background-color: var(--hover-bg, #f5f5f5);
  color: var(--text-primary, #333);
}

.action-btn {
  margin-top: 16px;
  padding: 10px 20px;
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
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

@media (max-width: 768px) {
  .settings-page {
    flex-direction: column;
  }

  .settings-content {
    padding: 20px;
  }

  .help-panel {
    height: calc(100vh - 250px);
  }
}
</style>
