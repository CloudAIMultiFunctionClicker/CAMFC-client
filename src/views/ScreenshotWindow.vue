

<template>
  <div class="screenshot-container" :class="{ 'light-mode': isLightMode }">

    <div class="toolbar" v-show="!isCropMode" @mousedown="startToolbarDrag">

      <template v-if="!isAnnotateMode">
        <div class="toolbar-left">
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
          <button class="zoom-btn" @click="handleAnnotate" title="标注">
            <i class="ri-edit-line"></i>
          </button>
          <button class="zoom-btn" @click="handleCrop" title="裁切">
            <i class="ri-crop-line"></i>
          </button>
        </div>
        <div class="toolbar-right">
          <button class="zoom-btn" @click="saveToNotes" title="保存到笔记">
            <i class="ri-sticky-note-line"></i>
          </button>
          <button class="zoom-btn" @click="saveScreenshot" title="保存">
            <i class="ri-save-line"></i>
          </button>
          <div class="control-divider"></div>
          <button class="zoom-btn window-control-btn" @click="minimizeWindow" title="最小化">
            <i class="ri-subtract-line"></i>
          </button>
          <button class="zoom-btn window-control-btn" @click="maximizeWindow" title="最大化">
            <i class="ri-rectangle-line"></i>
          </button>
          <button class="zoom-btn close-btn" @click="closeWindow" title="关闭">
            <i class="ri-close-line"></i>
          </button>
        </div>
      </template>

      <template v-else>
        <div class="toolbar-left">

          <button
            :class="['zoom-btn', { active: annotateCurrentTool === 'select' }]"
            @click="selectAnnotateTool('select')"
            title="选择工具"
          >
            <i class="ri-cursor-line"></i>
          </button>
          <button
            :class="['zoom-btn', { active: annotateCurrentTool === 'free' }]"
            @click="selectAnnotateTool('free')"
            title="自由绘制"
          >
            <i class="ri-edit-line"></i>
          </button>
          <div class="control-divider"></div>

          <div class="color-picker-group">
            <button
              v-for="color in annotateColors"
              :key="color.value"
              :class="['annotate-color-btn', { active: annotateCurrentColor === color.value }]"
              :style="{ backgroundColor: color.value }"
              @click.stop="selectAnnotateColor(color.value)"
              :title="color.name"
            >
              <i v-if="annotateCurrentColor === color.value" class="ri-check-line"></i>
            </button>
          </div>
          <div class="control-divider"></div>

          <div class="stroke-width-group">
            <button
              v-for="width in annotateStrokeWidths"
              :key="width"
              :class="['annotate-stroke-btn', { active: annotateCurrentStrokeWidth === width }]"
              @click.stop="selectAnnotateStrokeWidth(width)"
              :title="`粗细：${width}px`"
            >
              <div
                class="annotate-stroke-preview"
                :style="{
                  width: width + 'px',
                  height: width + 'px',
                  backgroundColor: annotateCurrentColor
                }"
              ></div>
            </button>
          </div>
          <div class="control-divider"></div>

          <button class="zoom-btn" @click="undoAnnotate" :disabled="!annotateCanUndo" title="撤销">
            <i class="ri-arrow-go-back-line"></i>
          </button>
          <button class="zoom-btn" @click="redoAnnotate" :disabled="!annotateCanRedo" title="重做">
            <i class="ri-arrow-go-forward-line"></i>
          </button>
          <button class="zoom-btn" @click="clearAnnotate" title="清除所有">
            <i class="ri-delete-bin-line"></i>
          </button>
        </div>
        <div class="toolbar-right">
          <button class="zoom-btn cancel-btn" @click="handleAnnotateCancel" title="取消标注">
            <i class="ri-close-line"></i>
          </button>
          <button class="zoom-btn primary-btn" @click="() => annotatePanelRef?.completeAnnotate()" title="完成标注">
            <i class="ri-check-line"></i>
          </button>
          <div class="control-divider"></div>
          <button class="zoom-btn window-control-btn" @click="minimizeWindow" title="最小化">
            <i class="ri-subtract-line"></i>
          </button>
          <button class="zoom-btn window-control-btn" @click="maximizeWindow" title="最大化">
            <i class="ri-rectangle-line"></i>
          </button>
          <button class="zoom-btn close-btn" @click="closeWindow" title="关闭">
            <i class="ri-close-line"></i>
          </button>
        </div>
      </template>
    </div>

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

      <div v-if="isAnnotateMode" class="annotate-overlay">
        <AnnotatePanel
          ref="annotatePanelRef"
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
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { showToast } from '../components/layout/showToast.js'
import AnnotatePanel from '../components/annotate/AnnotatePanel.vue'
import { saveAnnotations, loadAnnotations, generateImageId } from '../utils/annotationStorage.js'

import { Window } from '@tauri-apps/api/window'

const isLightMode = ref(false)

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

const annotatePanelRef = ref(null)

const annotateColors = [
  { value: '#ef4444', name: '红色' },
  { value: '#3178c6', name: '蓝色' },
  { value: '#10b981', name: '绿色' },
  { value: '#f59e0b', name: '黄色' },
  { value: '#8b5cf6', name: '紫色' }
]

const annotateStrokeWidths = [2, 4, 6, 8, 10]

const annotateCurrentTool = computed(() => annotatePanelRef.value?.currentTool || 'free')
const annotateCurrentColor = computed(() => annotatePanelRef.value?.currentColor || '#ef4444')
const annotateCurrentStrokeWidth = computed(() => annotatePanelRef.value?.currentStrokeWidth || 4)
const annotateCanUndo = computed(() => annotatePanelRef.value?.canUndo || false)
const annotateCanRedo = computed(() => annotatePanelRef.value?.canRedo || false)

const selectAnnotateTool = (tool) => {
  annotatePanelRef.value?.selectTool(tool)
}

const selectAnnotateColor = (color) => {
  annotatePanelRef.value?.selectColor(color)
}

const selectAnnotateStrokeWidth = (width) => {
  annotatePanelRef.value?.selectStrokeWidth(width)
}

const undoAnnotate = () => {
  annotatePanelRef.value?.undo()
}

const redoAnnotate = () => {
  annotatePanelRef.value?.redo()
}

const clearAnnotate = () => {
  annotatePanelRef.value?.clearAll()
}

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
  console.log('[ScreenshotView] 收到标注完成数据', {
    hasData: !!data.imageData,
    dataLength: data.imageData?.length,
    imageDataPreview: data.imageData?.substring(0, 50)
  })

  if (!data.imageData || data.imageData.length === 0) {
    console.error('[ScreenshotView] 标注数据为空')
    showToast('标注数据无效', '#ef4444')
    isAnnotateMode.value = false
    return
  }

  const img = new Image()
  img.onload = () => {
    console.log('[ScreenshotView] 标注图片加载成功', img.width, 'x', img.height)

    annotatedImageData.value = data.imageData
    screenshotData.value = data.imageData
    width.value = img.width
    height.value = img.height

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

    resetZoom()
  }

  img.onerror = () => {
    console.error('[ScreenshotView] 标注图片加载失败', {
      dataLength: data.imageData?.length,
      hasData: !!data.imageData
    })
    showToast('图片保存失败', '#ef4444')
    isAnnotateMode.value = false
  }

  img.src = data.imageData
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

    setTimeout(() => {
      openNoteEditorWindow({ uuid, title: noteTitle, content: markdownContent })
    }, 200)

  } catch (error) {
    console.error('保存到笔记失败:', error)
    showToast('保存失败：' + (error.message || '网络错误'), '#ef4444')
  }
}

const openNoteEditorWindow = (note) => {
  const windowLabel = `note-editor-${note.uuid}`
  const url = `/note-editor?uuid=${note.uuid}&title=${encodeURIComponent(note.title)}`

  console.log('准备打开笔记编辑窗口:', {
    windowLabel,
    url,
    noteTitle: note.title
  })

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
        console.log('已发送笔记内容到窗口')
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

const minimizeWindow = async () => {
  try {
    const currentWindow = await getCurrentWindow()
    await currentWindow.minimize()
  } catch (e) {
    console.error('最小化窗口失败:', e)
  }
}

const maximizeWindow = async () => {
  try {
    const currentWindow = await getCurrentWindow()
    await currentWindow.maximize()
  } catch (e) {
    console.error('最大化窗口失败:', e)
  }
}

const closeWindow = async () => {
  try {
    const currentWindow = await getCurrentWindow()
    await currentWindow.close()
  } catch (e) {
    console.error('关闭窗口失败:', e)
  }
}

const isToolbarDragging = ref(false)
const toolbarStartX = ref(0)
const toolbarStartY = ref(0)
const toolbarWindowStartX = ref(0)
const toolbarWindowStartY = ref(0)

const startToolbarDrag = async (e) => {

  if (e.target.closest('.zoom-btn')) return

  isToolbarDragging.value = true
  toolbarStartX.value = e.clientX
  toolbarStartY.value = e.clientY

  try {
    const currentWindow = await getCurrentWindow()
    const position = await currentWindow.outerPosition()
    toolbarWindowStartX.value = position.x
    toolbarWindowStartY.value = position.y
  } catch (err) {
    console.log('获取窗口位置失败:', err)
  }

  document.addEventListener('mousemove', onToolbarDrag)
  document.addEventListener('mouseup', endToolbarDrag)
}

const onToolbarDrag = async (e) => {
  if (!isToolbarDragging.value) return

  const dx = e.clientX - toolbarStartX.value
  const dy = e.clientY - toolbarStartY.value

  try {
    const currentWindow = await getCurrentWindow()
    await currentWindow.setPosition({
      x: toolbarWindowStartX.value + dx,
      y: toolbarWindowStartY.value + dy
    })
  } catch (err) {
    console.log('移动窗口失败:', err)
  }
}

const endToolbarDrag = () => {
  isToolbarDragging.value = false
  document.removeEventListener('mousemove', onToolbarDrag)
  document.removeEventListener('mouseup', endToolbarDrag)
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

function initTheme() {
  try {
    const savedTheme = localStorage.getItem('theme-preference')
    if (savedTheme === 'light' || savedTheme === 'dark') {
      isLightMode.value = savedTheme === 'light'
    } else {

      isLightMode.value = window.matchMedia('(prefers-color-scheme: light)').matches
    }
  } catch (e) {
    console.error('初始化主题失败:', e)
    isLightMode.value = false
  }
}

function setupThemeListener() {
  try {

    listen('theme-changed', (event) => {
      const theme = event.payload
      isLightMode.value = theme === 'light'
      console.log('收到主题变化事件:', theme)
    })
  } catch (e) {
    console.error('设置主题监听失败:', e)
  }
}

onMounted(() => {
  setupScreenshotListener().then(() => {
    console.log('截图监听器设置完成')
  }).catch((e) => {
    console.error('设置截图监听器失败:', e)
  })
  document.addEventListener('keydown', handleKeyDown)

  initTheme()

  setupThemeListener()
})

onUnmounted(() => {
  if (unlistenScreenshotData) {
    unlistenScreenshotData()
  }
  document.removeEventListener('keydown', handleKeyDown)
})
</script>

<style scoped>

.screenshot-container {

  --bg-primary: #0d1117;
  --bg-secondary: #161b22;
  --bg-tertiary: #21262d;
  --bg-header: #0d0d0d;
  --text-primary: #c9d1d9;
  --text-secondary: #8b949e;
  --text-muted: #6e7681;
  --border-color: #30363d;
  --accent-blue: #58a6ff;
  --accent-blue-rgb: 88, 166, 255;
  --accent-blue-bright: #1f6feb;
  --accent-green: #3fb950;
  --accent-green-rgb: 63, 185, 80;
  --accent-red: #f85149;
  --accent-red-rgb: 248, 81, 73;
  --hover-bg: rgba(255, 255, 255, 0.08);
  --danger-btn-bg: rgba(248, 81, 73, 0.1);
  --danger-btn-text: #f85149;
  --danger-btn-border: rgba(248, 81, 73, 0.4);
  --danger-btn-hover-bg: #f85149;
  --danger-btn-hover-text: #ffffff;
  --danger-btn-hover-border: #f85149;
}

.screenshot-container.light-mode {

  --bg-primary: #ffffff;
  --bg-secondary: #f6f8fa;
  --bg-tertiary: #eaeef2;
  --bg-header: #f6f8fa;
  --text-primary: #24292f;
  --text-secondary: #57606a;
  --text-muted: #8c959f;
  --border-color: rgba(0, 0, 0, 0.1);
  --accent-blue: #0969da;
  --accent-blue-rgb: 9, 105, 218;
  --accent-blue-bright: #0550ae;
  --accent-green: #2da44e;
  --accent-green-rgb: 45, 164, 78;
  --accent-red: #cf222e;
  --accent-red-rgb: 207, 34, 46;
  --hover-bg: rgba(0, 0, 0, 0.05);
  --danger-btn-bg: #ffebe9;
  --danger-btn-text: #cf222e;
  --danger-btn-border: rgba(207, 34, 46, 0.4);
  --danger-btn-hover-bg: #cf222e;
  --danger-btn-hover-text: #ffffff;
  --danger-btn-hover-border: #cf222e;
}

.screenshot-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--bg-primary);
}

.toolbar {
  width: 100%;
  height: 48px;
  padding: 0 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: var(--bg-header);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
  box-sizing: border-box;
  cursor: move;
  -webkit-app-region: drag;
  position: relative;
  z-index: 200;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 0;
  margin-left: auto;
  margin-right: -16px;
}

.toolbar .zoom-btn {
  -webkit-app-region: no-drag;
  cursor: pointer;
}

.toolbar .zoom-btn.window-control-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-primary, #f8fafc);
}

.toolbar .zoom-btn.close-btn:hover {
  background-color: var(--accent-red);
  color: white;
}

.toolbar .zoom-btn.active {
  background-color: #3b82f6;
  color: #fff;
}

.color-picker-group {
  display: flex;
  gap: 6px;
  align-items: center;
}

.stroke-width-group {
  display: flex;
  gap: 6px;
  align-items: center;
}

.annotate-color-btn {
  width: 26px;
  height: 26px;
  border-radius: 2px;
  border: 2px solid transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  position: relative;
  padding: 0;
}

.annotate-color-btn:hover {
  transform: scale(1.1);
}

.annotate-color-btn.active {
  border-color: #fff;
  box-shadow: 0 0 0 2px var(--bg-secondary, #1e293b);
}

.annotate-color-btn i {
  color: #fff;
  font-size: 14px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

.annotate-stroke-btn {
  width: 30px;
  height: 30px;
  border-radius: 2px;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  background-color: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  padding: 0;
}

.annotate-stroke-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.05));
}

.annotate-stroke-btn.active {
  border-color: var(--accent-blue, #3178c6);
  background-color: rgba(var(--accent-blue-rgb, 49, 120, 198), 0.1);
}

.annotate-stroke-preview {
  border-radius: 2px;
}

.toolbar .primary-btn {
  background-color: #10b981;
  color: #fff;
}

.toolbar .primary-btn:hover {
  background-color: #059669;
  color: white;
}

.toolbar .cancel-btn:hover {
  background-color: #ef4444;
  color: white;
}

.screenshot-main {
  flex: 1;
  position: relative;
  overflow: hidden;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: var(--bg-primary);
}

.image-wrapper {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
  cursor: grab;
}

.image-wrapper:active {
  cursor: grabbing;
}

.image-drag-container {
  display: flex;
  justify-content: center;
  align-items: center;
  transition: transform 0.1s ease-out;
}

.screenshot-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  user-select: none;
  pointer-events: none;
}

.crop-image-wrapper {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
  position: relative;
}

.crop-base-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  user-select: none;
  pointer-events: none;
}

.crop-selection {
  position: absolute;
  border: 2px solid #3b82f6;
  background-color: rgba(59, 130, 246, 0.2);
  cursor: move;
}

.crop-handle {
  position: absolute;
  width: 12px;
  height: 12px;
  background-color: #3b82f6;
  border: 2px solid white;
  border-radius: 2px;
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
  background-color: #3b82f6;
  color: white;
  padding: 2px 8px;
  border-radius: 2px;
  font-size: 12px;
  pointer-events: none;
}

.crop-selection-actions {
  position: absolute;
  top: -40px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 4px;
}

.crop-action-btn {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 2px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.crop-action-btn.cancel {
  background-color: #ef4444;
  color: white;
}

.crop-action-btn.cancel:hover {
  background-color: #dc2626;
}

.crop-action-btn.apply {
  background-color: #10b981;
  color: white;
}

.crop-action-btn.apply:hover {
  background-color: #059669;
}

.zoom-controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px;
  background-color: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.zoom-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 2px;
  background: transparent;
  color: var(--text-secondary, #cbd5e1);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  font-size: 16px;
}

.zoom-btn:hover:not(:disabled) {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-primary, #f8fafc);
}

.zoom-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.zoom-btn.close-btn:hover {
  background-color: var(--accent-red);
  color: white;
}

.zoom-scale-btn {
  min-width: 60px;
  height: 36px;
  border: none;
  border-radius: 2px;
  background: transparent;
  color: var(--text-secondary, #cbd5e1);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.zoom-scale-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-primary, #f8fafc);
}

.control-divider {
  width: 1px;
  height: 24px;
  background-color: var(--border-color);
}

.annotate-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.8);
  z-index: 50;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
