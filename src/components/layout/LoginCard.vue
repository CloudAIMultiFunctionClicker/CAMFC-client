<!--
登录卡片组件 - 简化版登录界面
功能：用户名密码输入，记住密码选项
注意：目前只是个占位符，实际登录逻辑还没接
-->

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { showToast } from './showToast.js'

const router = useRouter()

// 表单数据
const username = ref('')
const password = ref('')
const rememberMe = ref(false)
const isLoading = ref(false)

// 登录处理 - 统一错误处理
const handleLogin = async () => {
  if (!username.value || !password.value) {
    showToast('请输入用户名和密码', 'var(--accent-red)')
    return
  }

  isLoading.value = true
  
  try {
    // TODO: 这里接入实际的登录 API
    // 现在只是模拟延迟
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    showToast('登录成功', 'var(--accent-green)')
    router.push('/main')
  } catch (error) {
    console.error('登录失败:', error)
    showToast('登录失败：' + error.message, 'var(--accent-red)')
  } finally {
    isLoading.value = false
  }
}

// 快速填充测试账号
const fillTestAccount = () => {
  username.value = 'test'
  password.value = '123456'
}
</script>

<template>
  <div class="login-container">
    <div class="login-card">
      <div class="login-header">
        <h2 class="login-title">
          <i class="ri-user-line"></i>
          用户登录
        </h2>
        <p class="login-subtitle">CAMFC Cloud</p>
      </div>

      <form class="login-form" @submit.prevent="handleLogin">
        <div class="form-group">
          <label class="form-label">用户名</label>
          <input
            v-model="username"
            type="text"
            class="form-input"
            placeholder="请输入用户名"
            :disabled="isLoading"
          />
        </div>

        <div class="form-group">
          <label class="form-label">密码</label>
          <input
            v-model="password"
            type="password"
            class="form-input"
            placeholder="请输入密码"
            :disabled="isLoading"
          />
        </div>

        <div class="form-options">
          <label class="checkbox-label">
            <input type="checkbox" v-model="rememberMe" :disabled="isLoading" />
            <span>记住密码</span>
          </label>
          
          <button type="button" class="test-account-btn" @click="fillTestAccount">
            测试账号
          </button>
        </div>

        <button type="submit" class="login-btn" :disabled="isLoading">
          <i v-if="isLoading" class="ri-loader-4-line spinning"></i>
          <span v-else>登录</span>
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: calc(100vh - 65px);
  padding: 20px;
}

.login-card {
  width: 100%;
  max-width: 400px;
  background: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  padding: 32px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.login-title {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 8px 0;
}

.login-title i {
  font-size: 28px;
  color: var(--accent-blue, #0969da);
}

.login-subtitle {
  font-size: 14px;
  color: var(--text-secondary, #57606a);
  margin: 0;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary, #24292f);
}

.form-input {
  padding: 10px 14px;
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  font-size: 14px;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #24292f);
  transition: all 0.2s ease;
}

.form-input:focus {
  outline: none;
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.form-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary, #57606a);
  cursor: pointer;
  user-select: none;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.test-account-btn {
  padding: 6px 12px;
  background: transparent;
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  font-size: 13px;
  color: var(--text-secondary, #57606a);
  cursor: pointer;
  transition: all 0.2s ease;
}

.test-account-btn:hover {
  background: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
}

.login-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px;
  background: var(--accent-blue, #0969da);
  color: white;
  border: none;
  border-radius: .375rem;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.login-btn:hover:not(:disabled) {
  filter: brightness(1.1);
}

.login-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>