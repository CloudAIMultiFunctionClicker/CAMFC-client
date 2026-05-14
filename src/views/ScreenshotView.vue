

<template>
  <div class="screenshot-container">
    <main class="screenshot-main">

      <div v-if="isCropMode" class="crop-image-wrapper" ref="cropImageWrapper" @mousedown="startDrawCrop" @mousemove="onDrawing" @mouseup="endDrawCrop" @mouseleave="endDrawCrop">
        <img :src="screenshotData" alt="裁切预览" class="crop-base-image" />

        <div
          v-if="cropBox.width > 0 && cropBox.height > 0"
          class="crop-selection"
          :style="{
            left: cropBox.x + 'px',
            top: cropBox.y + 'px',
            width: cropBox.width + 'px',
            height: cropBox.height + 'px'
          }"
        >

          <div class="crop-handle crop-handle-nw" @mousedown.stop="startResize('nw')"></div>
          <div class="crop-handle crop-handle-n" @mousedown.stop="startResize('n')"></div>
          <div class="crop-handle crop-handle-ne" @mousedown.stop="startResize('ne')"></div>
          <div class="crop-handle crop-handle-e" @mousedown.stop="startResize('e')"></div>
          <div class="crop-handle crop-handle-se" @mousedown.stop="startResize('se')"></div>
          <div class="crop-handle crop-handle-s" @mousedown.stop="startResize('s')"></div>
          <div class="crop-handle crop-handle-sw" @mousedown.stop="startResize('sw')"></div>
          <div class="crop-handle crop-handle-w" @mousedown.stop="startResize('w')"></div>

          <div class="crop-size-label">
            {{ Math.round(cropBox.width) }} x {{ Math.round(cropBox.height) }}
          </div>

          <div class="crop-selection-actions" @mousedown.stop @mousemove.stop @mouseup.stop>
            <button class="crop-action-btn cancel" @mousedown.stop @click="cancelCrop" title="取消">
              <i class="ri-close-line"></i>
            </button>
            <button class="crop-action-btn apply" @mousedown.stop @click="applyCrop" title="应用">
              <i class="ri-check-line"></i>
            </button>
          </div>
        </div>
      </div>

      <div v-else class="image-wrapper" ref="imageWrapper" @wheel="handleWheel">
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
      <div class="zoom-controls" v-show="!isCropMode && !isAnnotateMode">
        <button class="zoom-btn" @click="zoomOut" :disabled="scale <= 0.1">
          <i class="ri-zoom-out-line"></i>
        </button>
        <button class="zoom-scale-btn" @click="resetZoom">
          {{ Math.round(scale * 100) }}%
        </button>
        <button class="zoom-btn" @click="zoomIn" :disabled="scale >= 3">
          <i class="ri-zoom-in-line"></i>
        </button>
        <div class="control-divider"></div>
        <button class="zoom-btn" @click="saveToNotes" title="保存到笔记">
          <i class="ri-sticky-note-line"></i>
        </button>
        <button class="zoom-btn" @click="saveScreenshot" title="保存">
          <i class="ri-save-line"></i>
        </button>
        <button class="zoom-btn" @click="handleAnnotate" title="标注">
          <i class="ri-edit-line"></i>
        </button>
        <button class="zoom-btn" @click="handleCrop" title="裁切">
          <i class="ri-crop-line"></i>
        </button>
      </div>

      <div v-if="isAnnotateMode" class="annotate-overlay">
        <AnnotatePanel
          :image-data="screenshotData"
          :image-width="width"
          :image-height="height"
          :existing-annotations="hasExistingAnnotations ? loadAnnotations(currentImageId)?.annotations : null"
          @complete="handleAnnotateComplete"
          @cancel="handleAnnotateCancel"
        />
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onBeforeMount, onMounted, onUnmounted, watch } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { showToast } from '../components/layout/showToast.js'
import AnnotatePanel from '../components/annotate/AnnotatePanel.vue'
import { saveAnnotations, loadAnnotations, generateImageId } from '../utils/annotationStorage.js'

const screenshotData = ref(null)
const width = ref(0)
const height = ref(0)
const screenshotTime = ref('')
const loading = ref(false)
const error = ref(null)

const isCropMode = ref(false)
const cropBox = ref({
  x: 0,
  y: 0,
  width: 0,
  height: 0
})
const cropImageWrapper = ref(null)
const isCropping = ref(false)
const cropStart = ref({ x: 0, y: 0 })
const cropOriginal = ref({ x: 0, y: 0, width: 0, height: 0 })
const resizeHandle = ref('')
const isDrawing = ref(false)
const drawStart = ref({ x: 0, y: 0 })

const scale = ref(1)
const translateX = ref(0)
const translateY = ref(0)
const isDragging = ref(false)
const startX = ref(0)
const startY = ref(0)
const imageWrapper = ref(null)

const isAnnotateMode = ref(false)
const annotatedImageData = ref(null)
const currentImageId = ref(null)
const hasExistingAnnotations = ref(false)

let unlistenScreenshotData = null

const processScreenshotData = (result) => {
  console.log('处理截图数据:', {
    hasImageData: !!result.image_data,
    width: result.width,
    height: result.height,
    imageDataLength: result.image_data?.length
  })

  isCropMode.value = false
  isAnnotateMode.value = false
  cropBox.value = { x: 0, y: 0, width: 0, height: 0 }

  screenshotData.value = result.image_data
  width.value = result.width
  height.value = result.height
  screenshotTime.value = new Date().toLocaleString('zh-CN')

  const img = new Image()
  img.onload = () => {
    console.log('图片加载成功:', img.width, img.height)
    showToast('截图成功', '#10b981')
    resetZoom()
  }
  img.onerror = () => {
    console.error('图片加载失败')
    showToast('图片加载失败', '#ef4444')
  }
  img.src = result.image_data
}

const setupScreenshotListener = async () => {

  unlistenScreenshotData = await listen('screenshot-data', (event) => {
    console.log('收到截图数据事件')
    const result = event.payload
    if (result.success) {
      processScreenshotData(result)
    } else {
      error.value = result.error || '截图失败'
      showToast(error.value, '#ef4444')
    }
  })

  console.log('截图监听器已设置完成')
}

const captureScreenshot = async () => {

  showToast('请从悬浮窗重新截图', '#f59e0b')
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
  isDragging.value = true
  startX.value = e.clientX - translateX.value
  startY.value = e.clientY - translateY.value
  e.preventDefault()
}

const drag = (e) => {
  if (!isDragging.value) return
  translateX.value = e.clientX - startX.value
  translateY.value = e.clientY - startY.value
}

const endDrag = () => {
  isDragging.value = false
}

const handleWheel = (e) => {
  e.preventDefault()
  const delta = e.deltaY > 0 ? -0.1 : 0.1
  const newScale = scale.value + delta
  scale.value = Math.max(0.1, Math.min(3, newScale))
}

const handleCrop = () => {
  if (!screenshotData.value) return

  isCropMode.value = true

  cropBox.value = { x: 0, y: 0, width: 0, height: 0 }
}

const handleAnnotate = () => {
  if (!screenshotData.value) {
    console.error('[ScreenshotView] screenshotData 为空，无法进入标注模式')
    return
  }

  console.log('[ScreenshotView] 进入标注模式', {
    hasData: !!screenshotData.value,
    dataLength: screenshotData.value?.length,
    width: width.value,
    height: height.value
  })

  if (!currentImageId.value) {
    currentImageId.value = generateImageId(screenshotData.value)
  }

  const savedData = loadAnnotations(currentImageId.value)
  if (savedData) {
    hasExistingAnnotations.value = true
    showToast('已加载之前保存的标注', '#10b981')
  }

  isAnnotateMode.value = true
}

const handleAnnotateComplete = (data) => {
  annotatedImageData.value = data.imageData
  screenshotData.value = data.imageData

  const img = new Image()
  img.onload = () => {
    width.value = img.width
    height.value = img.height
    console.log('[ScreenshotView] 标注完成，图片尺寸更新为:', img.width, 'x', img.height)
  }
  img.src = data.imageData

  if (currentImageId.value) {
    const success = saveAnnotations(
      currentImageId.value,
      data.annotations,
      data.imageData
    )
    if (success) {
      showToast('标注已保存', '#10b981')
    } else {
      showToast('保存失败', '#ef4444')
    }
  }

  isAnnotateMode.value = false
}

const handleAnnotateCancel = () => {
  isAnnotateMode.value = false
  if (annotatedImageData.value) {
    screenshotData.value = annotatedImageData.value
  }
  showToast('已取消标注', '#94a3b8')
}

const cancelCrop = () => {
  isCropMode.value = false
  cropBox.value = { x: 0, y: 0, width: 0, height: 0 }
}

const startDrawCrop = (e) => {
  if (!cropImageWrapper.value) return

  const wrapper = cropImageWrapper.value
  const rect = wrapper.getBoundingClientRect()
  const img = wrapper.querySelector('.crop-base-image')

  if (!img) return

  const imgDisplayWidth = img.width
  const imgDisplayHeight = img.height
  const imgOffsetX = (rect.width - imgDisplayWidth) / 2
  const imgOffsetY = (rect.height - imgDisplayHeight) / 2

  const mouseX = e.clientX - rect.left
  const mouseY = e.clientY - rect.top

  if (mouseX < imgOffsetX || mouseX > imgOffsetX + imgDisplayWidth ||
      mouseY < imgOffsetY || mouseY > imgOffsetY + imgDisplayHeight) {
    return
  }

  isDrawing.value = true

  drawStart.value = {
    x: mouseX,
    y: mouseY
  }

  cropBox.value = {
    x: mouseX,
    y: mouseY,
    width: 0,
    height: 0
  }

  e.preventDefault()
  e.stopPropagation()
}

const onDrawing = (e) => {
  if (!isDrawing.value || !cropImageWrapper.value) return

  const wrapper = cropImageWrapper.value
  const rect = wrapper.getBoundingClientRect()
  const img = wrapper.querySelector('.crop-base-image')

  if (!img) return

  const imgDisplayWidth = img.width
  const imgDisplayHeight = img.height
  const imgOffsetX = (rect.width - imgDisplayWidth) / 2
  const imgOffsetY = (rect.height - imgDisplayHeight) / 2

  const mouseX = e.clientX - rect.left
  const mouseY = e.clientY - rect.top

  const x = Math.min(drawStart.value.x, mouseX)
  const y = Math.min(drawStart.value.y, mouseY)
  const w = Math.abs(mouseX - drawStart.value.x)
  const h = Math.abs(mouseY - drawStart.value.y)

  const minX = imgOffsetX
  const minY = imgOffsetY
  const maxX = imgOffsetX + imgDisplayWidth
  const maxY = imgOffsetY + imgDisplayHeight

  cropBox.value = {
    x: Math.max(minX, Math.min(x, maxX - 1)),
    y: Math.max(minY, Math.min(y, maxY - 1)),
    width: Math.min(w, maxX - Math.max(minX, x)),
    height: Math.min(h, maxY - Math.max(minY, y))
  }

  e.preventDefault()
  e.stopPropagation()
}

const endDrawCrop = (e) => {
  if (!isDrawing.value) return
  isDrawing.value = false
  e?.preventDefault()
  e?.stopPropagation()
}

const applyCrop = () => {
  if (!cropImageWrapper.value || !cropBox.value.width || !cropBox.value.height) return

  const wrapper = cropImageWrapper.value
  const img = wrapper.querySelector('.crop-base-image')
  if (!img) return

  const imgDisplayWidth = img.width
  const imgDisplayHeight = img.height

  const scaleX = width.value / imgDisplayWidth
  const scaleY = height.value / imgDisplayHeight

  const cropX = cropBox.value.x * scaleX
  const cropY = cropBox.value.y * scaleY
  const cropW = cropBox.value.width * scaleX
  const cropH = cropBox.value.height * scaleY

  const canvas = document.createElement('canvas')
  canvas.width = Math.round(cropW)
  canvas.height = Math.round(cropH)

  const ctx = canvas.getContext('2d')
  const imgElement = new Image()
  imgElement.crossOrigin = 'anonymous'
  imgElement.src = screenshotData.value

  imgElement.onload = () => {
    ctx.drawImage(
      imgElement,
      cropX, cropY, cropW, cropH,
      0, 0, canvas.width, canvas.height
    )

    screenshotData.value = canvas.toDataURL('image/png')
    width.value = canvas.width
    height.value = canvas.height

    isCropMode.value = false
    cropBox.value = { x: 0, y: 0, width: 0, height: 0 }

    resetZoom()

    showToast('裁切成功', '#10b981')
  }

  imgElement.onerror = () => {
    showToast('裁切失败', '#ef4444')
  }
}

watch(() => [cropBox.value.x, cropBox.value.y, cropBox.value.width, cropBox.value.height], () => {
  if (isCropMode.value) {
    updateCropSizeLabel()
  }
}, { deep: true })

const updateCropSizeLabel = () => {

}

const saveScreenshot = async () => {
  if (!screenshotData.value) {
    showToast('没有可保存的图片', '#f59e0b')
    return
  }

  try {

    const { save } = await import('@tauri-apps/plugin-dialog')
    const { writeBinaryFile } = await import('@tauri-apps/plugin-fs')
    const { join } = await import('@tauri-apps/api/path')

    const filePath = await save({
      title: '保存图片',
      defaultPath: 'screenshot.png',
      filters: [{
        name: 'PNG Image',
        extensions: ['png']
      }]
    })

    if (filePath) {

      const base64Data = screenshotData.value.split(',')[1]
      const binaryData = Uint8Array.from(atob(base64Data), c => c.charCodeAt(0))

      await writeBinaryFile(filePath, binaryData)
      showToast('图片已保存', '#10b981')
    }
  } catch (error) {
    console.error('保存图片失败:', error)
    showToast('保存失败', '#ef4444')
  }
}

const saveToNotes = async () => {
  if (!screenshotData.value) {
    showToast('没有可保存的图片', '#f59e0b')
    return
  }

  try {

    const { getBackendUrl } = await import('../config/backend.js')

    const { getDeviceId, getTotp } = await import('../components/data/bluetooth.js')
    const deviceId = await getDeviceId()
    const currentTotp = await getTotp()
    const authHeader = { "Id": deviceId, "Totp": currentTotp }

    const now = new Date()
    const timeStr = `${now.getMonth() + 1}月${now.getDate()}日 ${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}`
    const noteTitle = `截图笔记_${timeStr}`
    const uuid = crypto.randomUUID()

    const createResponse = await fetch(getBackendUrl() + '/note/add', {
      method: 'POST',
      headers: { ...authHeader, 'Content-Type': 'application/json' },
      body: JSON.stringify({ uuid, title: noteTitle })
    })

    if (!createResponse.ok) {
      throw new Error('创建笔记失败')
    }

    const markdownContent = `# ${noteTitle}\n\n![截图](${screenshotData.value})\n`

    const updateResponse = await fetch(getBackendUrl() + '/note/update', {
      method: 'POST',
      headers: { ...authHeader, 'Content-Type': 'application/json' },
      body: JSON.stringify({
        uuid,
        title: noteTitle,
        content: markdownContent
      })
    })

    if (!updateResponse.ok) {
      throw new Error('更新笔记内容失败')
    }

    showToast('截图已保存到笔记', '#10b981')

    window.location.href = '/'

    openNoteEditorWindow({ uuid, title: noteTitle, content: markdownContent })

  } catch (error) {
    console.error('保存到笔记失败:', error)
    showToast('保存失败：' + (error.message || '网络错误'), '#ef4444')
  }
}

const openNoteEditorWindow = (note) => {
  const windowLabel = `note-editor-${note.uuid}`
  const url = `/note-editor?uuid=${note.uuid}&title=${encodeURIComponent(note.title)}`

  const webview = new WebviewWindow(windowLabel, {
    url: url,
    title: note.title || '编辑笔记',
    width: 900,
    height: 600,
    minWidth: 400,
    minHeight: 300,
    center: true,
    decorations: false,
    resizable: true
  })

  webview.once('tauri://created', async () => {
    console.log('笔记编辑窗口创建成功:', windowLabel)

    setTimeout(async () => {
      try {
        await webview.emit('load-note-content', { content: note.content || '' })
      } catch (e) {
        console.error('发送笔记内容失败:', e)
      }
    }, 300)
  })

  webview.once('tauri://error', (e) => {
    console.error('笔记编辑窗口创建失败:', e)
    showToast('打开编辑窗口失败', '#ef4444')
  })
}

const goBack = () => {
  window.history.back()
}

const handleKeyDown = (e) => {
  if (!isCropMode.value) return

  if (e.key === 'Enter') {
    e.preventDefault()
    applyCrop()
  }

  if (e.key === 'Escape') {
    e.preventDefault()
    cancelCrop()
  }
}

const startCropDrag = (e) => {
  isCropping.value = true
  cropStart.value = { x: e.clientX, y: e.clientY }
  cropOriginal.value = { ...cropBox.value }

  document.addEventListener('mousemove', onCropDrag)
  document.addEventListener('mouseup', stopCropDrag)
  e.preventDefault()
}

const onCropDrag = (e) => {
  if (!isCropping.value) return

  const dx = e.clientX - cropStart.value.x
  const dy = e.clientY - cropStart.value.y

  cropBox.value.x = cropOriginal.value.x + dx
  cropBox.value.y = cropOriginal.value.y + dy

  const wrapper = cropImageWrapper.value
  if (wrapper) {
    const img = wrapper.querySelector('.crop-base-image')
    if (img) {
      const imgDisplayWidth = img.width
      const imgDisplayHeight = img.height

      cropBox.value.x = Math.max(0, Math.min(cropBox.value.x, imgDisplayWidth - cropBox.value.width))
      cropBox.value.y = Math.max(0, Math.min(cropBox.value.y, imgDisplayHeight - cropBox.value.height))
    }
  }
}

const stopCropDrag = () => {
  isCropping.value = false
  document.removeEventListener('mousemove', onCropDrag)
  document.removeEventListener('mouseup', stopCropDrag)
}

const startResize = (handle) => {
  resizeHandle.value = handle
  cropStart.value = { x: event.clientX, y: event.clientY }
  cropOriginal.value = { ...cropBox.value }

  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
  event.preventDefault()
}

const onResize = (e) => {
  if (!resizeHandle.value) return

  const dx = e.clientX - cropStart.value.x
  const dy = e.clientY - cropStart.value.y
  const handle = resizeHandle.value

  if (handle.includes('e')) {
    cropBox.value.width = Math.max(50, cropOriginal.value.width + dx)
  }
  if (handle.includes('w')) {
    const newWidth = Math.max(50, cropOriginal.value.width - dx)
    cropBox.value.x = cropOriginal.value.x + (cropOriginal.value.width - newWidth)
    cropBox.value.width = newWidth
  }
  if (handle.includes('s')) {
    cropBox.value.height = Math.max(50, cropOriginal.value.height + dy)
  }
  if (handle.includes('n')) {
    const newHeight = Math.max(50, cropOriginal.value.height - dy)
    cropBox.value.y = cropOriginal.value.y + (cropOriginal.value.height - newHeight)
    cropBox.value.height = newHeight
  }

  const wrapper = cropImageWrapper.value
  if (wrapper) {
    const img = wrapper.querySelector('.crop-base-image')
    if (img) {
      const imgRect = img.getBoundingClientRect()
      const wrapperRect = wrapper.getBoundingClientRect()
      const imgDisplayWidth = img.width
      const imgDisplayHeight = img.height
      const imgOffsetX = (wrapperRect.width - imgDisplayWidth) / 2
      const imgOffsetY = (wrapperRect.height - imgDisplayHeight) / 2

      cropBox.value.x = Math.max(imgOffsetX, Math.min(cropBox.value.x, imgOffsetX + imgDisplayWidth - 50))
      cropBox.value.y = Math.max(imgOffsetY, Math.min(cropBox.value.y, imgOffsetY + imgDisplayHeight - 50))
      cropBox.value.width = Math.min(imgDisplayWidth, cropBox.value.width)
      cropBox.value.height = Math.min(imgDisplayHeight, cropBox.value.height)
    }
  }
}

const stopResize = () => {
  resizeHandle.value = ''
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
}

onBeforeMount(async () => {
  await setupScreenshotListener()
})

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  if (unlistenScreenshotData) {
    unlistenScreenshotData()
  }
  document.removeEventListener('keydown', handleKeyDown)
})
</script>

<style scoped>
.screenshot-page {
  height: 100vh;
  max-height: 100vh;
  background-color: var(--bg-primary, #0f172a);
  display: flex;
  flex-direction: column;
  overflow: hidden;
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
  border-radius: 2px;
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
  border-radius: 2px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.action-btn.primary {
  background-color: var(--accent-blue, #3178c6);
  border-color: var(--accent-blue, #3178c6);
}

.action-btn.primary:hover {
  background-color: var(--accent-blue-bright, #1f6feb);
}

.screenshot-content {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.screenshot-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.screenshot-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.loading-state,
.error-state {
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
  border-top-color: var(--accent-blue, #3178c6);
  border-radius: 2px;
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

.preview-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  position: relative;
  overflow: hidden;
  min-height: 0;
}

.screenshot-info {
  display: flex;
  gap: 24px;
  color: var(--text-muted, #64748b);
  font-size: 14px;
  flex-shrink: 0;
}

.image-wrapper {
  flex: 1;
  background-color: var(--bg-secondary, #1e293b);
  border-radius: 2px;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  position: relative;
  min-height: 0;
}

.image-drag-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  transform-origin: center center;
  transition: transform 0.1s ease-out;
}

.screenshot-image {
  max-width: 100%;
  max-height: 100%;
  width: auto;
  height: auto;
  object-fit: contain;
  border-radius: 2px;
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
  position: absolute;
  bottom: 32px;
  right: 16px;
  z-index: 10;
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
  border-radius: 2px;
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

.control-divider {
  width: 1px;
  height: 24px;
  background-color: var(--border-color, rgba(255, 255, 255, 0.1));
  margin: 0 4px;
  align-self: center;
}

.zoom-scale-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 12px;
  background-color: var(--bg-secondary, #1e293b);
  color: var(--text-primary, #f1f5f9);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 2px;
  font-size: 14px;
  font-weight: 500;
  min-width: 60px;
  cursor: pointer;
  transition: all 0.2s;
}

.zoom-scale-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.annotate-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.95);
  z-index: 99999;
  overflow: hidden;
}

.crop-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: transparent;
  display: flex;
  flex-direction: column;
  z-index: 100;
  overflow: hidden;
}

.crop-mode-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: var(--bg-secondary, #1e293b);
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  z-index: 101;
}

.crop-mode-tip {
  color: var(--text-secondary, #94a3b8);
  font-size: 14px;
}

.crop-cancel-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: transparent;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.2));
  border-radius: 2px;
  color: var(--text-primary, #f1f5f9);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.crop-cancel-btn:hover {
  background-color: rgba(239, 68, 68, 0.1);
  border-color: #ef4444;
  color: #ef4444;
}

.crop-cancel-btn i {
  font-size: 16px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background-color: transparent;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.2));
  border-radius: 2px;
  color: var(--text-primary, #f1f5f9);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.action-btn.primary {
  background-color: var(--accent-blue, #3178c6);
  border-color: var(--accent-blue, #3178c6);
}

.action-btn.primary:hover {
  background-color: var(--accent-blue-bright, #1f6feb);
}

.action-btn i {
  font-size: 18px;
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.crop-image-wrapper {
  flex: 1;
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: transparent;
  max-height: calc(100vh - 100px);
  min-height: 0;
}

.crop-base-image {
  display: block;
  max-width: 100%;
  max-height: 100%;
  width: auto;
  height: auto;
  object-fit: contain;
  user-select: none;
  pointer-events: none;
}

.crop-selection {
  position: absolute;
  border: 2px solid var(--accent-blue, #3178c6);
  background-color: rgba(var(--accent-blue-rgb, 49, 120, 198), 0.1);
  cursor: move;
  z-index: 10;
  pointer-events: none;
}

.crop-selection-actions {
  position: fixed;
  bottom: 32px;
  right: 16px;
  display: flex;
  gap: 8px;
  pointer-events: auto !important;
  z-index: 99999;
}

.crop-selection-actions button {
  pointer-events: auto !important;
}

.crop-action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background-color: var(--bg-secondary, #1e293b);
  color: var(--text-primary, #f1f5f9);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 2px;
  font-size: 18px;
  cursor: pointer;
  transition: all 0.2s;
}

.crop-action-btn:hover:not(:disabled) {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.crop-action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.crop-action-btn.cancel {
  background-color: var(--bg-secondary, #1e293b);
  color: var(--text-primary, #f1f5f9);
}

.crop-action-btn.cancel:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  border-color: #ef4444;
}

.crop-action-btn.apply {
  background-color: var(--bg-secondary, #1e293b);
  color: var(--text-primary, #f1f5f9);
}

.crop-action-btn.apply:hover {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
  border-color: #10b981;
}

.crop-handle {
  position: absolute;
  width: 10px;
  height: 10px;
  background-color: #fff;
  border: 2px solid #3b82f6;
  border-radius: 2px;
  z-index: 10;
}

.crop-handle-nw { top: -6px; left: -6px; cursor: nw-resize; }
.crop-handle-n { top: -6px; left: 50%; transform: translateX(-50%); cursor: n-resize; }
.crop-handle-ne { top: -6px; right: -6px; cursor: ne-resize; }
.crop-handle-e { top: 50%; right: -6px; transform: translateY(-50%); cursor: e-resize; }
.crop-handle-se { bottom: -6px; right: -6px; cursor: se-resize; }
.crop-handle-s { bottom: -6px; left: 50%; transform: translateX(-50%); cursor: s-resize; }
.crop-handle-sw { bottom: -6px; left: -6px; cursor: sw-resize; }
.crop-handle-w { top: 50%; left: -6px; transform: translateY(-50%); cursor: w-resize; }

.crop-size-label {
  position: absolute;
  top: -28px;
  left: 50%;
  transform: translateX(-50%);
  background-color: rgba(0, 0, 0, 0.8);
  color: #fff;
  padding: 4px 8px;
  border-radius: 2px;
  font-size: 12px;
  white-space: nowrap;
  pointer-events: none;
}

.image-wrapper:has(.image-drag-container:active) .screenshot-image {
  cursor: grabbing;
}
</style>
