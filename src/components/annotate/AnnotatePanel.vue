<template>
  <div class="annotate-panel">
    <!-- 标注画布 -->
    <div 
      class="annotate-canvas-wrapper"
      ref="canvasWrapper"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
      @mouseleave="handleMouseUp"
    >
      <canvas 
        ref="annotateCanvas"
        class="annotate-canvas"
      ></canvas>
    </div>

    <!-- 底部控制栏 -->
    <div class="annotate-bottom-bar">
      <!-- 左侧：标注工具栏 -->
      <div class="annotate-toolbar">
        <!-- 工具选择 -->
        <div class="tool-group">
          <button 
            v-for="tool in tools" 
            :key="tool.id"
            :class="['tool-btn', { active: currentTool === tool.id }]"
            @click="selectTool(tool.id)"
            :title="tool.name"
          >
            <i :class="tool.icon"></i>
          </button>
        </div>

        <div class="divider"></div>

        <!-- 颜色选择 -->
        <div class="tool-group">
          <div class="color-picker">
            <button 
              v-for="color in colors" 
              :key="color.value"
              :class="['color-btn', { active: currentColor === color.value }]"
              :style="{ backgroundColor: color.value }"
              @click="selectColor(color.value)"
              :title="color.name"
            >
              <i v-if="currentColor === color.value" class="ri-check-line"></i>
            </button>
          </div>
        </div>

        <div class="divider"></div>

        <!-- 线条粗细 -->
        <div class="tool-group">
          <div class="stroke-width-picker">
            <button 
              v-for="width in strokeWidths" 
              :key="width"
              :class="['stroke-btn', { active: currentStrokeWidth === width }]"
              @click="selectStrokeWidth(width)"
              :title="`粗细：${width}px`"
            >
              <div 
                class="stroke-preview" 
                :style="{ 
                  width: width + 'px', 
                  height: width + 'px',
                  backgroundColor: currentColor 
                }"
              ></div>
            </button>
          </div>
        </div>

        <div class="divider"></div>

        <!-- 操作按钮 -->
        <div class="tool-group">
          <button 
            class="action-btn"
            @click="undo"
            :disabled="!canUndo"
            title="撤销 (Ctrl+Z)"
          >
            <i class="ri-arrow-go-back-line"></i>
          </button>
          <button 
            class="action-btn"
            @click="redo"
            :disabled="!canRedo"
            title="重做 (Ctrl+Y)"
          >
            <i class="ri-arrow-go-forward-line"></i>
          </button>
          <button 
            class="action-btn"
            @click="clearAll"
            title="清除所有标注"
          >
            <i class="ri-delete-bin-line"></i>
          </button>
        </div>
      </div>

      <!-- 右侧：完成/取消按钮 -->
      <div class="annotate-actions">
        <button 
          class="action-btn cancel-btn"
          @click="cancelAnnotate"
          title="取消标注"
        >
          <i class="ri-close-line"></i>
        </button>
        <button 
          class="action-btn primary-btn"
          @click="completeAnnotate"
          title="完成标注"
        >
          <i class="ri-check-line"></i>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import { showToast } from '../layout/showToast.js'

const props = defineProps({
  imageData: {
    type: String,
    required: true
  },
  imageWidth: {
    type: Number,
    default: 0
  },
  imageHeight: {
    type: Number,
    default: 0
  }
})

const emit = defineEmits(['complete', 'cancel'])

// 工具定义 - 只保留选择和自由绘制
const tools = [
  { id: 'select', name: '选择', icon: 'ri-cursor-line' },
  { id: 'free', name: '自由绘制', icon: 'ri-edit-line' }
]

// 颜色定义 - 5 种常见颜色
const colors = [
  { value: '#ef4444', name: '红色' },
  { value: '#3b82f6', name: '蓝色' },
  { value: '#10b981', name: '绿色' },
  { value: '#f59e0b', name: '黄色' },
  { value: '#8b5cf6', name: '紫色' }
]

// 线条粗细选项
const strokeWidths = [2, 4, 6, 8, 10]

// 状态
const currentTool = ref('free')
const currentColor = ref('#ef4444')
const currentStrokeWidth = ref(4)
const isDrawing = ref(false)
const startPoint = ref({ x: 0, y: 0 })
const currentPoint = ref({ x: 0, y: 0 })

// 标注数据
const annotations = ref([])
const selectedAnnotationId = ref(null)
const history = ref([])
const historyIndex = ref(-1)

// 画布相关
const canvasWrapper = ref(null)
const annotateCanvas = ref(null)
let ctx = null

// 文字输入
const textInput = ref(null)
const isTextInputMode = ref(false)
const textPosition = ref({ x: 0, y: 0 })

// 计算属性
const canUndo = computed(() => historyIndex.value >= 0)
const canRedo = computed(() => historyIndex.value < history.value.length - 1)

// 初始化
onMounted(() => {
  initCanvas()
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})

// 初始化画布
const initCanvas = () => {
  if (!canvasWrapper.value || !annotateCanvas.value) return
  
  const wrapper = canvasWrapper.value
  
  // 使用图片原始分辨率作为 canvas 尺寸，避免模糊
  if (props.imageWidth && props.imageHeight) {
    annotateCanvas.value.width = props.imageWidth
    annotateCanvas.value.height = props.imageHeight
  } else {
    annotateCanvas.value.width = wrapper.clientWidth
    annotateCanvas.value.height = wrapper.clientHeight
  }
  
  ctx = annotateCanvas.value.getContext('2d')
  
  // 加载背景图片
  const img = new Image()
  img.crossOrigin = 'anonymous'
  img.src = props.imageData
  
  img.onload = () => {
    drawBackground(img)
    redrawAnnotations()
  }
}

// 绘制背景图片
const drawBackground = (img) => {
  if (!ctx) return
  
  const canvas = annotateCanvas.value
  
  // 如果 canvas 尺寸等于图片原始尺寸，直接绘制
  if (canvas.width === props.imageWidth && canvas.height === props.imageHeight) {
    ctx.drawImage(img, 0, 0)
  } else {
    // 否则按比例缩放绘制
    const scaleX = canvas.width / props.imageWidth
    const scaleY = canvas.height / props.imageHeight
    const scale = Math.min(scaleX, scaleY)
    
    const width = props.imageWidth * scale
    const height = props.imageHeight * scale
    const x = (canvas.width - width) / 2
    const y = (canvas.height - height) / 2
    
    ctx.drawImage(img, x, y, width, height)
  }
}

// 重新绘制所有标注
const redrawAnnotations = () => {
  if (!ctx || !props.imageData) return
  
  const img = new Image()
  img.crossOrigin = 'anonymous'
  img.src = props.imageData
  
  img.onload = () => {
    drawBackground(img)
    annotations.value.forEach(annotation => {
      drawAnnotation(annotation)
    })
  }
}

// 绘制单个标注
const drawAnnotation = (annotation) => {
  if (!ctx) return
  
  ctx.strokeStyle = annotation.color
  ctx.fillStyle = annotation.color
  ctx.lineWidth = annotation.strokeWidth
  ctx.lineCap = 'round'
  ctx.lineJoin = 'round'
  
  switch (annotation.type) {
    case 'rect':
      drawRect(annotation)
      break
    case 'circle':
      drawCircle(annotation)
      break
    case 'arrow':
      drawArrow(annotation)
      break
    case 'free':
      drawFree(annotation)
      break
    case 'text':
      drawText(annotation)
      break
  }
  
  // 如果选中，绘制选中框
  if (annotation.id === selectedAnnotationId.value) {
    drawSelectionBox(annotation)
  }
}

// 绘制矩形
const drawRect = (annotation) => {
  ctx.strokeRect(
    annotation.x,
    annotation.y,
    annotation.width,
    annotation.height
  )
}

// 绘制圆形
const drawCircle = (annotation) => {
  ctx.beginPath()
  ctx.ellipse(
    annotation.x + annotation.width / 2,
    annotation.y + annotation.height / 2,
    Math.abs(annotation.width / 2),
    Math.abs(annotation.height / 2),
    0,
    0,
    2 * Math.PI
  )
  ctx.stroke()
}

// 绘制箭头
const drawArrow = (annotation) => {
  const headLength = 15
  const angle = Math.atan2(annotation.endY - annotation.startY, annotation.endX - annotation.startX)
  
  ctx.beginPath()
  ctx.moveTo(annotation.startX, annotation.startY)
  ctx.lineTo(annotation.endX, annotation.endY)
  ctx.stroke()
  
  // 箭头头部
  ctx.beginPath()
  ctx.moveTo(annotation.endX, annotation.endY)
  ctx.lineTo(
    annotation.endX - headLength * Math.cos(angle - Math.PI / 6),
    annotation.endY - headLength * Math.sin(angle - Math.PI / 6)
  )
  ctx.lineTo(
    annotation.endX - headLength * Math.cos(angle + Math.PI / 6),
    annotation.endY - headLength * Math.sin(angle + Math.PI / 6)
  )
  ctx.closePath()
  ctx.fill()
}

// 绘制自由线条
const drawFree = (annotation) => {
  if (!annotation.points || annotation.points.length < 2) return
  
  ctx.beginPath()
  ctx.moveTo(annotation.points[0].x, annotation.points[0].y)
  
  for (let i = 1; i < annotation.points.length; i++) {
    ctx.lineTo(annotation.points[i].x, annotation.points[i].y)
  }
  
  ctx.stroke()
}

// 绘制文字
const drawText = (annotation) => {
  ctx.font = `${16 + annotation.strokeWidth}px Arial`
  ctx.fillText(annotation.text, annotation.x, annotation.y)
}

// 绘制选中框
const drawSelectionBox = (annotation) => {
  ctx.strokeStyle = '#3b82f6'
  ctx.lineWidth = 1
  ctx.setLineDash([5, 5])
  
  let bounds = getAnnotationBounds(annotation)
  ctx.strokeRect(bounds.x - 5, bounds.y - 5, bounds.width + 10, bounds.height + 10)
  
  ctx.setLineDash([])
}

// 获取标注边界
const getAnnotationBounds = (annotation) => {
  switch (annotation.type) {
    case 'rect':
    case 'circle':
      return {
        x: Math.min(annotation.x, annotation.x + annotation.width),
        y: Math.min(annotation.y, annotation.y + annotation.height),
        width: Math.abs(annotation.width),
        height: Math.abs(annotation.height)
      }
    case 'arrow':
      return {
        x: Math.min(annotation.startX, annotation.endX),
        y: Math.min(annotation.startY, annotation.endY),
        width: Math.abs(annotation.endX - annotation.startX),
        height: Math.abs(annotation.endY - annotation.startY)
      }
    case 'free':
      const xs = annotation.points.map(p => p.x)
      const ys = annotation.points.map(p => p.y)
      return {
        x: Math.min(...xs),
        y: Math.min(...ys),
        width: Math.max(...xs) - Math.min(...xs),
        height: Math.max(...ys) - Math.min(...ys)
      }
    case 'text':
      return {
        x: annotation.x,
        y: annotation.y - 20,
        width: ctx.measureText(annotation.text).width,
        height: 20
      }
    default:
      return { x: 0, y: 0, width: 0, height: 0 }
  }
}

// 鼠标事件处理
const handleMouseDown = (e) => {
  if (!ctx) return
  
  const rect = annotateCanvas.value.getBoundingClientRect()
  const scaleX = annotateCanvas.value.width / rect.width
  const scaleY = annotateCanvas.value.height / rect.height
  const x = (e.clientX - rect.left) * scaleX
  const y = (e.clientY - rect.top) * scaleY
  
  if (currentTool.value === 'select') {
    // 选择模式
    const clickedAnnotation = findAnnotationAtPoint(x, y)
    if (clickedAnnotation) {
      selectedAnnotationId.value = clickedAnnotation.id
      isDrawing.value = true
      startPoint.value = { x, y }
    } else {
      selectedAnnotationId.value = null
    }
    redrawAnnotations()
  } else if (currentTool.value === 'text') {
    // 文字输入模式
    isTextInputMode.value = true
    textPosition.value = { x, y }
    showTextInput(x, y)
  } else {
    // 绘制模式
    isDrawing.value = true
    startPoint.value = { x, y }
    currentPoint.value = { x, y }
    
    if (currentTool.value === 'free') {
      // 自由绘制
      const newAnnotation = {
        id: Date.now(),
        type: 'free',
        color: currentColor.value,
        strokeWidth: currentStrokeWidth.value,
        points: [{ x, y }]
      }
      annotations.value.push(newAnnotation)
    }
  }
}

const handleMouseMove = (e) => {
  if (!isDrawing.value || !ctx) return
  
  const rect = annotateCanvas.value.getBoundingClientRect()
  const scaleX = annotateCanvas.value.width / rect.width
  const scaleY = annotateCanvas.value.height / rect.height
  const x = (e.clientX - rect.left) * scaleX
  const y = (e.clientY - rect.top) * scaleY
  currentPoint.value = { x, y }
  
  if (currentTool.value === 'select' && selectedAnnotationId.value) {
    // 移动选中的标注
    const annotation = annotations.value.find(a => a.id === selectedAnnotationId.value)
    if (annotation) {
      const dx = x - startPoint.value.x
      const dy = y - startPoint.value.y
      moveAnnotation(annotation, dx, dy)
      startPoint.value = { x, y }
      redrawAnnotations()
    }
  } else if (currentTool.value === 'free') {
    // 自由绘制
    const annotation = annotations.value[annotations.value.length - 1]
    if (annotation) {
      annotation.points.push({ x, y })
      redrawAnnotations()
    }
  } else {
    // 其他工具
    redrawAnnotations()
    drawPreview()
  }
}

const handleMouseUp = () => {
  if (!isDrawing.value) return
  
  if (currentTool.value !== 'select' && currentTool.value !== 'free' && currentTool.value !== 'text') {
    // 创建新标注
    const annotation = createAnnotation()
    if (annotation) {
      annotations.value.push(annotation)
      saveState()
    }
  } else if (currentTool.value === 'free') {
    saveState()
  }
  
  isDrawing.value = false
}

// 创建标注
const createAnnotation = () => {
  const base = {
    id: Date.now(),
    color: currentColor.value,
    strokeWidth: currentStrokeWidth.value
  }
  
  switch (currentTool.value) {
    case 'rect':
      return {
        ...base,
        type: 'rect',
        x: startPoint.value.x,
        y: startPoint.value.y,
        width: currentPoint.value.x - startPoint.value.x,
        height: currentPoint.value.y - startPoint.value.y
      }
    case 'circle':
      return {
        ...base,
        type: 'circle',
        x: startPoint.value.x,
        y: startPoint.value.y,
        width: currentPoint.value.x - startPoint.value.x,
        height: currentPoint.value.y - startPoint.value.y
      }
    case 'arrow':
      return {
        ...base,
        type: 'arrow',
        startX: startPoint.value.x,
        startY: startPoint.value.y,
        endX: currentPoint.value.x,
        endY: currentPoint.value.y
      }
    default:
      return null
  }
}

// 移动标注
const moveAnnotation = (annotation, dx, dy) => {
  switch (annotation.type) {
    case 'rect':
    case 'circle':
      annotation.x += dx
      annotation.y += dy
      break
    case 'arrow':
      annotation.startX += dx
      annotation.startY += dy
      annotation.endX += dx
      annotation.endY += dy
      break
    case 'free':
      annotation.points.forEach(point => {
        point.x += dx
        point.y += dy
      })
      break
    case 'text':
      annotation.x += dx
      annotation.y += dy
      break
  }
}

// 绘制预览
const drawPreview = () => {
  redrawAnnotations()
  
  const annotation = createAnnotation()
  if (annotation) {
    drawAnnotation(annotation)
  }
}

// 查找点击位置的标注
const findAnnotationAtPoint = (x, y) => {
  for (let i = annotations.value.length - 1; i >= 0; i--) {
    const annotation = annotations.value[i]
    const bounds = getAnnotationBounds(annotation)
    
    if (x >= bounds.x && x <= bounds.x + bounds.width &&
        y >= bounds.y && y <= bounds.y + bounds.height) {
      return annotation
    }
  }
  return null
}

// 显示文字输入框
const showTextInput = (x, y) => {
  const input = document.createElement('input')
  input.type = 'text'
  input.className = 'annotate-text-input'
  input.style.position = 'absolute'
  input.style.left = x + 'px'
  input.style.top = y + 'px'
  input.style.padding = '4px 8px'
  input.style.border = '2px solid #3b82f6'
  input.style.borderRadius = '4px'
  input.style.fontSize = '14px'
  input.style.outline = 'none'
  
  input.onkeydown = (e) => {
    if (e.key === 'Enter') {
      e.preventDefault()
      if (input.value.trim()) {
        annotations.value.push({
          id: Date.now(),
          type: 'text',
          color: currentColor.value,
          strokeWidth: currentStrokeWidth.value,
          text: input.value.trim(),
          x,
          y
        })
        saveState()
        showToast('文字标注已添加', '#10b981')
      }
      input.remove()
      isTextInputMode.value = false
      redrawAnnotations()
    } else if (e.key === 'Escape') {
      input.remove()
      isTextInputMode.value = false
    }
  }
  
  input.onblur = () => {
    setTimeout(() => {
      if (input.parentNode) {
        input.remove()
        isTextInputMode.value = false
      }
    }, 100)
  }
  
  canvasWrapper.value.appendChild(input)
  input.focus()
}

// 工具选择
const selectTool = (toolId) => {
  currentTool.value = toolId
  selectedAnnotationId.value = null
  redrawAnnotations()
}

// 颜色选择
const selectColor = (color) => {
  currentColor.value = color
}

// 线条粗细选择
const selectStrokeWidth = (width) => {
  currentStrokeWidth.value = width
}

// 删除标注
const deleteAnnotation = (id) => {
  const index = annotations.value.findIndex(a => a.id === id)
  if (index !== -1) {
    annotations.value.splice(index, 1)
    if (selectedAnnotationId.value === id) {
      selectedAnnotationId.value = null
    }
    saveState()
    redrawAnnotations()
    showToast('标注已删除', '#10b981')
  }
}

// 撤销
const undo = () => {
  if (historyIndex.value >= 0) {
    historyIndex.value--
    if (historyIndex.value >= 0) {
      annotations.value = JSON.parse(JSON.stringify(history.value[historyIndex.value]))
    } else {
      annotations.value = []
    }
    redrawAnnotations()
  }
}

// 重做
const redo = () => {
  if (historyIndex.value < history.value.length - 1) {
    historyIndex.value++
    annotations.value = JSON.parse(JSON.stringify(history.value[historyIndex.value]))
    redrawAnnotations()
  }
}

// 保存状态
const saveState = () => {
  history.value = history.value.slice(0, historyIndex.value + 1)
  history.value.push(JSON.parse(JSON.stringify(annotations.value)))
  historyIndex.value = history.value.length - 1
}

// 清除所有标注
const clearAll = () => {
  if (annotations.value.length === 0) return
  
  annotations.value = []
  selectedAnnotationId.value = null
  saveState()
  redrawAnnotations()
  showToast('已清除所有标注', '#10b981')
}

// 键盘快捷键
const handleKeyDown = (e) => {
  if (isTextInputMode.value) return
  
  if (e.ctrlKey || e.metaKey) {
    if (e.key === 'z') {
      e.preventDefault()
      undo()
    } else if (e.key === 'y') {
      e.preventDefault()
      redo()
    }
  }
  
  if (e.key === 'Delete' || e.key === 'Backspace') {
    if (selectedAnnotationId.value) {
      deleteAnnotation(selectedAnnotationId.value)
    }
  }
  
  if (e.key === 'Escape') {
    selectedAnnotationId.value = null
    currentTool.value = 'select'
    redrawAnnotations()
  }
}

// 完成标注
const completeAnnotate = () => {
  if (annotations.value.length === 0) {
    showToast('没有标注内容', '#f59e0b')
    return
  }
  
  // 将标注绘制到 canvas 上并导出
  const canvas = document.createElement('canvas')
  canvas.width = annotateCanvas.value.width
  canvas.height = annotateCanvas.value.height
  const newCtx = canvas.getContext('2d')
  
  const img = new Image()
  img.crossOrigin = 'anonymous'
  img.src = props.imageData
  
  img.onload = () => {
    const scaleX = canvas.width / props.imageWidth
    const scaleY = canvas.height / props.imageHeight
    const scale = Math.min(scaleX, scaleY)
    
    const width = props.imageWidth * scale
    const height = props.imageHeight * scale
    const x = (canvas.width - width) / 2
    const y = (canvas.height - height) / 2
    
    newCtx.drawImage(img, x, y, width, height)
    
    // 重绘所有标注
    annotations.value.forEach(annotation => {
      const tempCtx = ctx
      ctx = newCtx
      drawAnnotation(annotation)
      ctx = tempCtx
    })
    
    const annotatedData = canvas.toDataURL('image/png')
    emit('complete', {
      imageData: annotatedData,
      annotations: annotations.value
    })
    showToast('标注已保存', '#10b981')
  }
}

// 取消标注
const cancelAnnotate = () => {
  emit('cancel')
}

// 监听窗口大小变化
watch(() => props.imageData, () => {
  setTimeout(() => {
    initCanvas()
  }, 100)
})
</script>

<style scoped>
.annotate-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary, #0f172a);
}

.annotate-bottom-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: var(--bg-secondary, #1e293b);
  border-top: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  gap: 16px;
}

.annotate-toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  flex-wrap: wrap;
}

.annotate-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.tool-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.divider {
  width: 1px;
  height: 32px;
  background-color: var(--border-color, rgba(255, 255, 255, 0.1));
}

.tool-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background-color: transparent;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 8px;
  color: var(--text-primary, #f1f5f9);
  font-size: 20px;
  cursor: pointer;
  transition: all 0.2s;
}

.tool-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.05));
  border-color: var(--accent-blue, #3b82f6);
}

.tool-btn.active {
  background-color: var(--accent-blue, #3b82f6);
  border-color: var(--accent-blue, #3b82f6);
  color: #fff;
}

.color-picker {
  display: flex;
  gap: 8px;
}

.color-btn {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 2px solid transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  position: relative;
}

.color-btn:hover {
  transform: scale(1.1);
}

.color-btn.active {
  border-color: #fff;
  box-shadow: 0 0 0 2px var(--bg-secondary, #1e293b);
}

.color-btn i {
  color: #fff;
  font-size: 16px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

.stroke-width-picker {
  display: flex;
  gap: 8px;
  align-items: center;
}

.stroke-btn {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  background-color: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.stroke-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.05));
}

.stroke-btn.active {
  border-color: var(--accent-blue, #3b82f6);
  background-color: rgba(59, 130, 246, 0.1);
}

.stroke-preview {
  border-radius: 50%;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background-color: var(--bg-primary, #0f172a);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 8px;
  color: var(--text-primary, #f1f5f9);
  font-size: 18px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover:not(:disabled) {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 工具栏中的操作按钮 */
.annotate-toolbar .action-btn {
  width: 36px;
  height: 36px;
  padding: 0;
}

/* 底部操作栏的按钮 */
.annotate-actions .action-btn {
  width: 40px;
  height: 40px;
  font-size: 20px;
  padding: 0;
}

.action-btn.primary-btn {
  background-color: var(--accent-blue, #3b82f6);
  border-color: var(--accent-blue, #3b82f6);
  color: #fff;
}

.action-btn.primary-btn:hover {
  background-color: #2563eb;
}

.action-btn.cancel-btn {
  background-color: var(--bg-primary, #0f172a);
  color: var(--text-secondary, #94a3b8);
}

.action-btn.cancel-btn:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  border-color: #ef4444;
}

.annotate-canvas-wrapper {
  flex: 1;
  position: relative;
  overflow: auto;
  background-color: var(--bg-primary, #0f172a);
  cursor: crosshair;
  display: flex;
  align-items: center;
  justify-content: center;
}

.annotate-canvas {
  display: block;
  image-rendering: -webkit-optimize-contrast;
  image-rendering: crisp-edges;
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}
</style>
