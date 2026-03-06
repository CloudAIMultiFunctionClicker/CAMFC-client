<template>
  <div class="screenshot-page">
    <header class="screenshot-header">
      <button class="back-btn" @click="goBack">
        <i class="ri-close-line"></i>
        取消
      </button>
      <h1>屏幕截图</h1>
      <div class="header-actions">
        <button class="action-btn primary" @click="saveScreenshot" :disabled="!screenshotData">
          <i class="ri-download-line"></i>
          保存
        </button>
      </div>
    </header>

    <main class="screenshot-content">
      <div v-if="loading" class="loading-state">
        <div class="spinner"></div>
        <p>正在截取屏幕...</p>
      </div>

      <div v-else-if="error" class="error-state">
        <i class="ri-error-warning-line"></i>
        <p>{{ error }}</p>
        <button class="action-btn" @click="retakeScreenshot">重试</button>
      </div>

      <div v-else-if="screenshotData" class="preview-container">
        <div class="screenshot-info">
          <span>截图时间：{{ screenshotTime }}</span>
          <span>分辨率：{{ width }} x {{ height }}</span>
          <span>缩放：{{ Math.round(scale * 100) }}%</span>
        </div>
        <div class="zoom-controls">
          <button class="zoom-btn" @click="zoomOut" :disabled="scale <= 0.1">
            <i class="ri-zoom-out-line"></i>
          </button>
          <button class="zoom-btn" @click="resetZoom">
            <i class="ri-fullscreen-line"></i>
          </button>
          <button class="zoom-btn" @click="zoomIn" :disabled="scale >= 3">
            <i class="ri-zoom-in-line"></i>
          </button>
        </div>
        <div class="image-wrapper" ref="imageWrapper">
          <div 
            class="image-drag-container" 
            :style="{ transform: `translate(${translateX}px, ${translateY}px) scale(${scale})` }"
            @mousedown="startDrag"
            @mousemove="drag"
            @mouseup="endDrag"
            @mouseleave="endDrag"
          >
            <img :src="screenshotData" alt="屏幕截图" class="screenshot-image" draggable="false" />
          </div>
        </div>
      </div>

      <div v-else class="empty-state">
        <i class="ri-screenshot-line"></i>
        <p>暂无截图</p>
        <button class="action-btn primary" @click="captureScreenshot">
          <i class="ri-camera-line"></i>
          立即截图
        </button>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../components/layout/showToast.js'

const screenshotData = ref(null)
const width = ref(0)
const height = ref(0)
const screenshotTime = ref('')
const loading = ref(false)
const error = ref(null)

// 缩放和拖动相关
const scale = ref(1)
const translateX = ref(0)
const translateY = ref(0)
const isDragging = ref(false)
const startX = ref(0)
const startY = ref(0)
const imageWrapper = ref(null)

const captureScreenshot = async () => {
  loading.value = true
  error.value = null

  try {
    const result = await invoke('capture_screen')
    
    if (result.success) {
      screenshotData.value = result.image_data
      width.value = result.width
      height.value = result.height
      screenshotTime.value = new Date().toLocaleString('zh-CN')
      showToast('截图成功', '#10b981')
      resetZoom()
    } else {
      error.value = result.error || '截图失败'
      showToast(error.value, '#ef4444')
    }
  } catch (e) {
    error.value = e.toString()
    showToast('截图失败：' + e, '#ef4444')
  } finally {
    loading.value = false
  }
}

const zoomIn = () => {
  if (scale.value < 3) {
    scale.value = Math.min(scale.value + 0.25, 3)
  }
}

const zoomOut = () => {
  if (scale.value > 0.1) {
    scale.value = Math.max(scale.value - 0.25, 0.1)
  }
}

const resetZoom = () => {
  scale.value = 1
  translateX.value = 0
  translateY.value = 0
}

const startDrag = (e) => {
  if (scale.value <= 1) return
  isDragging.value = true
  startX.value = e.clientX - translateX.value
  startY.value = e.clientY - translateY.value
  e.preventDefault()
}

const drag = (e) => {
  if (!isDragging.value || scale.value <= 1) return
  translateX.value = e.clientX - startX.value
  translateY.value = e.clientY - startY.value
}

const endDrag = () => {
  isDragging.value = false
}

const saveScreenshot = async () => {
  showToast('保存功能开发中...', '#f59e0b')
}

const goBack = () => {
  window.history.back()
}

onMounted(() => {
  captureScreenshot()
})
</script>

<style scoped>
.screenshot-page {
  min-height: 100vh;
  background-color: var(--bg-primary, #0f172a);
  display: flex;
  flex-direction: column;
}

.screenshot-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 24px;
  background-color: var(--bg-secondary, #1e293b);
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.screenshot-header h1 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #f1f5f9);
  margin: 0;
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: none;
  border: none;
  color: var(--text-secondary, #94a3b8);
  font-size: 14px;
  cursor: pointer;
  border-radius: 8px;
  transition: all 0.2s;
}

.back-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.05));
  color: var(--text-primary, #f1f5f9);
}

.header-actions {
  display: flex;
  gap: 12px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background-color: var(--bg-secondary, #1e293b);
  color: var(--text-primary, #f1f5f9);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.action-btn.primary {
  background-color: var(--accent-blue, #3b82f6);
  border-color: var(--accent-blue, #3b82f6);
}

.action-btn.primary:hover {
  background-color: #2563eb;
}

.screenshot-content {
  flex: 1;
  padding: 24px;
  display: flex;
  flex-direction: column;
}

.loading-state,
.error-state,
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-muted, #64748b);
}

.loading-state .spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-top-color: var(--accent-blue, #3b82f6);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-state i {
  font-size: 48px;
  color: #ef4444;
  margin-bottom: 16px;
}

.empty-state i {
  font-size: 64px;
  color: var(--text-muted, #64748b);
  margin-bottom: 16px;
}

.preview-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.screenshot-info {
  display: flex;
  gap: 24px;
  color: var(--text-muted, #64748b);
  font-size: 14px;
}

.image-wrapper {
  flex: 1;
  background-color: var(--bg-secondary, #1e293b);
  border-radius: 12px;
  padding: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.screenshot-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  cursor: grab;
  user-select: none;
  -webkit-user-drag: none;
}

.screenshot-image:active {
  cursor: grabbing;
}

.zoom-controls {
  display: flex;
  gap: 8px;
  justify-content: center;
  margin-bottom: 12px;
}

.zoom-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background-color: var(--bg-secondary, #1e293b);
  color: var(--text-primary, #f1f5f9);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 8px;
  font-size: 18px;
  cursor: pointer;
  transition: all 0.2s;
}

.zoom-btn:hover:not(:disabled) {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.zoom-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.image-drag-container {
  transform-origin: center center;
  transition: transform 0.1s ease-out;
}

.image-wrapper:has(.image-drag-container:active) .screenshot-image {
  cursor: grabbing;
}
</style>
