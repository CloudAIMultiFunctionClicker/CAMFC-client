<template>
  <div class="settings-page">
    <main class="settings-content">
      <div class="settings-panel">
        <h3>学生认证设置</h3>
        <div class="setting-card">
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">学生用户名</span>
              <span class="label-desc">用于访问群组共享文件的学生账号</span>
            </div>
            <div class="setting-control">
              <input 
                type="text" 
                v-model="studentUsername"
                class="text-input"
                placeholder="请输入学生用户名"
              />
            </div>
          </div>
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">学生密码</span>
              <span class="label-desc">用于访问群组共享文件的学生密码</span>
            </div>
            <div class="setting-control">
              <input 
                type="password" 
                v-model="studentPassword"
                class="text-input"
                placeholder="请输入学生密码"
              />
            </div>
          </div>
          <div class="setting-actions">
            <button class="action-btn" @click="saveStudentCredentials">
              <i class="ri-save-line"></i>
              <span>保存认证信息</span>
            </button>
            <button class="action-btn danger" @click="clearStudentCredentials">
              <i class="ri-delete-bin-line"></i>
              <span>清除认证信息</span>
            </button>
          </div>
          <div class="setting-tip">
            <i class="ri-information-line"></i>
            <span>设置后，学生可以使用用户名密码访问群组共享文件。如果没有蓝牙设备，将自动使用学生认证。</span>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData, saveAppData } from '../components/data/storage.js'

const studentUsername = ref('')
const studentPassword = ref('')

// 保存学生认证信息
const saveStudentCredentials = async () => {
  if (!studentUsername.value || !studentPassword.value) {
    showToast('请输入用户名和密码', '#f59e0b')
    return
  }
  
  try {
    await saveAppData('student_username', studentUsername.value)
    await saveAppData('student_password', studentPassword.value)
    showToast('学生认证信息已保存', '#10b981')
    console.log('[学生认证设置] 学生认证信息已保存')
  } catch (error) {
    console.error('保存学生认证信息失败:', error)
    showToast('保存失败', '#ef4444')
  }
}

// 清除学生认证信息
const clearStudentCredentials = async () => {
  try {
    await saveAppData('student_username', '')
    await saveAppData('student_password', '')
    studentUsername.value = ''
    studentPassword.value = ''
    showToast('学生认证信息已清除', '#10b981')
    console.log('[学生认证设置] 学生认证信息已清除')
  } catch (error) {
    console.error('清除学生认证信息失败:', error)
    showToast('清除失败', '#ef4444')
  }
}

// 加载学生认证信息
const loadStudentCredentials = async () => {
  try {
    studentUsername.value = await loadAppData('student_username') || ''
    studentPassword.value = await loadAppData('student_password') || ''
    console.log('[学生认证设置] 学生认证信息已加载')
  } catch (error) {
    console.error('加载学生认证信息失败:', error)
  }
}

onMounted(() => {
  loadStudentCredentials()
})
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
  border-radius: .375rem;
  padding: 20px;
  margin-bottom: 16px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  margin-bottom: 12px;
  color: var(--text-primary, #24292f);
  font-size: 15px;
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.label-text {
  font-size: 15px;
  font-weight: 500;
}

.label-desc {
  font-size: 13px;
  color: var(--text-muted, #8c959f);
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.text-input {
  width: 100%;
  padding: 10px 14px;
  background-color: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  color: var(--text-primary, #24292f);
  font-size: 14px;
  transition: all 0.2s ease;
}

.text-input:focus {
  outline: none;
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.text-input::placeholder {
  color: var(--text-muted, #8c959f);
}

.setting-actions {
  display: flex;
  gap: 12px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color, #d0d7de);
}

.action-btn {
  margin-top: 16px;
  padding: 10px 20px;
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
}

.action-btn.danger {
  background-color: var(--danger-btn-bg, #212830);
  color: var(--danger-btn-text, #f85149);
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
}

.action-btn.danger:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, white);
  border-color: var(--danger-btn-hover-border, #f85149);
}

.action-btn.danger i,
.action-btn.danger svg {
  color: inherit;
}

.setting-tip {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin-top: 16px;
  padding: 12px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border-radius: .375rem;
  border: 1px solid var(--border-color, #d0d7de);
  font-size: 13px;
  color: var(--text-secondary, #57606a);
}

.setting-tip i {
  font-size: 16px;
  color: var(--accent-blue, #0969da);
  flex-shrink: 0;
  margin-top: 1px;
}

@media (max-width: 768px) {
  .settings-page {
    flex-direction: column;
  }

  .settings-content {
    padding: 20px;
  }
}
</style>
