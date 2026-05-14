

<template>
  <div class="annotate-panel">

    <div
      class="annotate-canvas-wrapper"
      :class="{ panning: isPanning }"
      ref="canvasWrapper"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
      @mouseleave="handleMouseUp"
    >
      <canvas
        ref="annotateCanvas"
        class="annotate-canvas"
        :style="{
          transform: `translate(${panOffset.x}px, ${panOffset.y}px) scale(${scale})`
        }"
      ></canvas>
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

const tools = [
  { id: 'select', name: '选择', icon: 'ri-cursor-line' },
  { id: 'free', name: '自由绘制', icon: 'ri-edit-line' }
]

const colors = [
  { value: '#ef4444', name: '红色' },
  { value: '#3178c6', name: '蓝色' },
  { value: '#10b981', name: '绿色' },
  { value: '#f59e0b', name: '黄色' },
  { value: '#8b5cf6', name: '紫色' }
]

const strokeWidths = [2, 4, 6, 8, 10]

const currentTool = ref('free')
const currentColor = ref('#ef4444')
const currentStrokeWidth = ref(4)
const isDrawing = ref(false)
const startPoint = ref({ x: 0, y: 0 })
const currentPoint = ref({ x: 0, y: 0 })

const scale = ref(1)
const panOffset = ref({ x: 0, y: 0 })
const isPanning = ref(false)
const panStart = ref({ x: 0, y: 0 })
const panStartOffset = ref({ x: 0, y: 0 })

const annotations = ref([])
const selectedAnnotationId = ref(null)
const history = ref([])
const historyIndex = ref(-1)

const canvasWrapper = ref(null)
const annotateCanvas = ref(null)
let ctx = null

const textInput = ref(null)
const isTextInputMode = ref(false)
const textPosition = ref({ x: 0, y: 0 })

const canUndo = computed(() => historyIndex.value >= 0)
const canRedo = computed(() => historyIndex.value < history.value.length - 1)

onMounted(() => {
  initCanvas()
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('wheel', handleWheel, { passive: false })
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
  window.removeEventListener('wheel', handleWheel)
})

const initCanvas = () => {
  if (!canvasWrapper.value || !annotateCanvas.value) return

  console.log('[AnnotatePanel] initCanvas 调用', {
    hasImageData: !!props.imageData,
    imageDataLength: props.imageData?.length,
    imageWidth: props.imageWidth,
    imageHeight: props.imageHeight
  })

  if (!props.imageData) {
    console.error('[AnnotatePanel] imageData 为空，无法初始化')
    showToast('图片数据加载失败', '#ef4444')
    return
  }

  const wrapper = canvasWrapper.value

  if (props.imageWidth && props.imageHeight) {
    annotateCanvas.value.width = props.imageWidth
    annotateCanvas.value.height = props.imageHeight
  } else {
    annotateCanvas.value.width = wrapper.clientWidth
    annotateCanvas.value.height = wrapper.clientHeight
  }

  ctx = annotateCanvas.value.getContext('2d')

  const img = new Image()
  img.crossOrigin = 'anonymous'
  img.src = props.imageData

  img.onload = () => {
    console.log('[AnnotatePanel] 图片加载成功', img.width, img.height)
    drawBackground(img)
    redrawAnnotations()
  }

  img.onerror = (err) => {
    console.error('[AnnotatePanel] 图片加载失败', err)
    showToast('图片加载失败', '#ef4444')
  }
}

const drawBackground = (img) => {
  if (!ctx) return

  const canvas = annotateCanvas.value

  if (canvas.width === props.imageWidth && canvas.height === props.imageHeight) {
    ctx.drawImage(img, 0, 0)
  } else {

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

  if (annotation.id === selectedAnnotationId.value) {
    drawSelectionBox(annotation)
  }
}

const drawRect = (annotation) => {
  ctx.strokeRect(
    annotation.x,
    annotation.y,
    annotation.width,
    annotation.height
  )
}

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

const drawArrow = (annotation) => {
  const headLength = 15
  const angle = Math.atan2(annotation.endY - annotation.startY, annotation.endX - annotation.startX)

  ctx.beginPath()
  ctx.moveTo(annotation.startX, annotation.startY)
  ctx.lineTo(annotation.endX, annotation.endY)
  ctx.stroke()

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

const drawFree = (annotation) => {
  if (!annotation.points || annotation.points.length < 2) return

  ctx.beginPath()
  ctx.moveTo(annotation.points[0].x, annotation.points[0].y)

  for (let i = 1; i < annotation.points.length; i++) {
    ctx.lineTo(annotation.points[i].x, annotation.points[i].y)
  }

  ctx.stroke()
}

const drawText = (annotation) => {
  ctx.font = `${16 + annotation.strokeWidth}px Arial`
  ctx.fillText(annotation.text, annotation.x, annotation.y)
}

const drawSelectionBox = (annotation) => {
  ctx.strokeStyle = '#3178c6'
  ctx.lineWidth = 1
  ctx.setLineDash([5, 5])

  let bounds = getAnnotationBounds(annotation)
  ctx.strokeRect(bounds.x - 5, bounds.y - 5, bounds.width + 10, bounds.height + 10)

  ctx.setLineDash([])
}

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

const handleMouseDown = (e) => {
  if (!ctx) return

  const rect = annotateCanvas.value.getBoundingClientRect()
  const scaleX = annotateCanvas.value.width / rect.width
  const scaleY = annotateCanvas.value.height / rect.height
  const x = ((e.clientX - rect.left - panOffset.value.x) / scale.value) * scaleX
  const y = ((e.clientY - rect.top - panOffset.value.y) / scale.value) * scaleY

  if (currentTool.value === 'select' && e.button === 0) {

    const clickedAnnotation = findAnnotationAtPoint(x, y)
    if (clickedAnnotation) {
      selectedAnnotationId.value = clickedAnnotation.id
      isDrawing.value = true
      startPoint.value = { x, y }
      redrawAnnotations()
      return
    } else {

      selectedAnnotationId.value = null
      isPanning.value = true
      panStart.value = { x: e.clientX, y: e.clientY }
      panStartOffset.value = { ...panOffset.value }
      redrawAnnotations()
      return
    }
  }

  if (currentTool.value === 'text') {

    isTextInputMode.value = true
    textPosition.value = { x, y }
    showTextInput(x, y)
  } else if (currentTool.value === 'free') {

    isDrawing.value = true
    startPoint.value = { x, y }
    currentPoint.value = { x, y }

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

const handleMouseMove = (e) => {

  if (isPanning.value) {
    const dx = e.clientX - panStart.value.x
    const dy = e.clientY - panStart.value.y
    panOffset.value = {
      x: panStartOffset.value.x + dx,
      y: panStartOffset.value.y + dy
    }
    return
  }

  if (!isDrawing.value || !ctx) return

  const rect = annotateCanvas.value.getBoundingClientRect()
  const scaleX = annotateCanvas.value.width / rect.width
  const scaleY = annotateCanvas.value.height / rect.height
  const x = ((e.clientX - rect.left - panOffset.value.x) / scale.value) * scaleX
  const y = ((e.clientY - rect.top - panOffset.value.y) / scale.value) * scaleY
  currentPoint.value = { x, y }

  if (currentTool.value === 'select' && selectedAnnotationId.value) {

    const annotation = annotations.value.find(a => a.id === selectedAnnotationId.value)
    if (annotation) {
      const dx = x - startPoint.value.x
      const dy = y - startPoint.value.y
      moveAnnotation(annotation, dx, dy)
      startPoint.value = { x, y }
      redrawAnnotations()
    }
  } else if (currentTool.value === 'free') {

    const annotation = annotations.value[annotations.value.length - 1]
    if (annotation) {
      annotation.points.push({ x, y })
      redrawAnnotations()
    }
  } else {

    redrawAnnotations()
    drawPreview()
  }
}

const handleMouseUp = () => {
  if (isPanning.value) {
    isPanning.value = false
    return
  }

  if (!isDrawing.value) return

  if (currentTool.value !== 'select' && currentTool.value !== 'free' && currentTool.value !== 'text') {

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

const drawPreview = () => {
  redrawAnnotations()

  const annotation = createAnnotation()
  if (annotation) {
    drawAnnotation(annotation)
  }
}

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

const showTextInput = (x, y) => {
  const input = document.createElement('input')
  input.type = 'text'
  input.className = 'annotate-text-input'
  input.style.position = 'absolute'
  input.style.left = x + 'px'
  input.style.top = y + 'px'
  input.style.padding = '4px 8px'
  input.style.border = '2px solid #3178c6'
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

const selectTool = (toolId) => {
  currentTool.value = toolId
  selectedAnnotationId.value = null
  redrawAnnotations()
}

const selectColor = (color) => {
  currentColor.value = color
}

const selectStrokeWidth = (width) => {
  currentStrokeWidth.value = width
}

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

const redo = () => {
  if (historyIndex.value < history.value.length - 1) {
    historyIndex.value++
    annotations.value = JSON.parse(JSON.stringify(history.value[historyIndex.value]))
    redrawAnnotations()
  }
}

const saveState = () => {
  history.value = history.value.slice(0, historyIndex.value + 1)
  history.value.push(JSON.parse(JSON.stringify(annotations.value)))
  historyIndex.value = history.value.length - 1
}

const clearAll = () => {
  if (annotations.value.length === 0) return

  annotations.value = []
  selectedAnnotationId.value = null
  saveState()
  redrawAnnotations()
  showToast('已清除所有标注', '#10b981')
}

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

const completeAnnotate = () => {
  console.log('[AnnotatePanel] completeAnnotate 调用', {
    hasCanvas: !!annotateCanvas.value,
    canvasWidth: annotateCanvas.value?.width,
    canvasHeight: annotateCanvas.value?.height,
    annotationsCount: annotations.value.length,
    hasCtx: !!ctx
  })

  if (!annotateCanvas.value) {
    console.error('[AnnotatePanel] canvas 不存在')
    showToast('画布未初始化', '#ef4444')
    return
  }

  if (!ctx) {
    console.error('[AnnotatePanel] canvas 上下文不存在')
    showToast('画布未准备好', '#ef4444')
    return
  }

  const canvas = document.createElement('canvas')
  canvas.width = annotateCanvas.value.width
  canvas.height = annotateCanvas.value.height
  const newCtx = canvas.getContext('2d')

  if (!newCtx) {
    console.error('[AnnotatePanel] 无法获取 canvas 上下文')
    showToast('无法导出图片', '#ef4444')
    return
  }

  const img = new Image()
  img.crossOrigin = 'anonymous'
  img.src = props.imageData

  img.onload = () => {
    console.log('[AnnotatePanel] 导出图片 - 背景图片加载成功')

    drawBackgroundOnContext(newCtx, img, canvas.width, canvas.height)

    annotations.value.forEach(annotation => {
      drawAnnotationOnContext(newCtx, annotation)
    })

    const annotatedData = canvas.toDataURL('image/png')

    console.log('[AnnotatePanel] 导出图片成功', {
      dataLength: annotatedData?.length,
      hasData: !!annotatedData
    })

    emit('complete', {
      imageData: annotatedData,
      annotations: annotations.value
    })
    if (annotations.value.length > 0) {
      showToast('标注已保存', '#10b981')
    }
  }

  img.onerror = (err) => {
    console.error('[AnnotatePanel] 导出图片 - 背景图片加载失败', err)
    showToast('图片导出失败', '#ef4444')
  }
}

const drawBackgroundOnContext = (context, img, width, height) => {
  if (props.imageWidth && props.imageHeight) {
    const scaleX = width / props.imageWidth
    const scaleY = height / props.imageHeight
    const scale = Math.min(scaleX, scaleY)

    const drawWidth = props.imageWidth * scale
    const drawHeight = props.imageHeight * scale
    const x = (width - drawWidth) / 2
    const y = (height - drawHeight) / 2

    context.drawImage(img, x, y, drawWidth, drawHeight)
  } else {
    context.drawImage(img, 0, 0, width, height)
  }
}

const drawAnnotationOnContext = (context, annotation) => {
  context.strokeStyle = annotation.color
  context.fillStyle = annotation.color
  context.lineWidth = annotation.strokeWidth
  context.lineCap = 'round'
  context.lineJoin = 'round'

  switch (annotation.type) {
    case 'rect':
      context.strokeRect(
        annotation.x,
        annotation.y,
        annotation.width,
        annotation.height
      )
      break
    case 'circle':
      context.beginPath()
      context.ellipse(
        annotation.x + annotation.width / 2,
        annotation.y + annotation.height / 2,
        Math.abs(annotation.width / 2),
        Math.abs(annotation.height / 2),
        0,
        0,
        2 * Math.PI
      )
      context.stroke()
      break
    case 'arrow':
      const headLength = 15
      const angle = Math.atan2(annotation.endY - annotation.startY, annotation.endX - annotation.startX)

      context.beginPath()
      context.moveTo(annotation.startX, annotation.startY)
      context.lineTo(annotation.endX, annotation.endY)
      context.stroke()

      context.beginPath()
      context.moveTo(annotation.endX, annotation.endY)
      context.lineTo(
        annotation.endX - headLength * Math.cos(angle - Math.PI / 6),
        annotation.endY - headLength * Math.sin(angle - Math.PI / 6)
      )
      context.lineTo(
        annotation.endX - headLength * Math.cos(angle + Math.PI / 6),
        annotation.endY - headLength * Math.sin(angle + Math.PI / 6)
      )
      context.closePath()
      context.fill()
      break
    case 'free':
      if (!annotation.points || annotation.points.length < 2) return

      context.beginPath()
      context.moveTo(annotation.points[0].x, annotation.points[0].y)

      for (let i = 1; i < annotation.points.length; i++) {
        context.lineTo(annotation.points[i].x, annotation.points[i].y)
      }

      context.stroke()
      break
    case 'text':
      context.font = `${16 + annotation.strokeWidth}px Arial`
      context.fillText(annotation.text, annotation.x, annotation.y)
      break
  }
}

const cancelAnnotate = () => {
  emit('cancel')
}

const handleWheel = (e) => {
  if (!canvasWrapper.value) return

  e.preventDefault()

  const rect = canvasWrapper.value.getBoundingClientRect()
  const mouseX = e.clientX - rect.left
  const mouseY = e.clientY - rect.top

  const delta = -e.deltaY
  const zoomSpeed = 0.001
  const zoomFactor = Math.pow(1 + zoomSpeed, delta)

  const newScale = Math.min(Math.max(scale.value * zoomFactor, 0.5), 5)

  const scaleRatio = newScale / scale.value
  panOffset.value = {
    x: mouseX - (mouseX - panOffset.value.x) * scaleRatio,
    y: mouseY - (mouseY - panOffset.value.y) * scaleRatio
  }

  scale.value = newScale
}

watch(() => props.imageData, () => {
  setTimeout(() => {
    initCanvas()
  }, 100)
})

defineExpose({
  currentTool,
  currentColor,
  currentStrokeWidth,
  canUndo,
  canRedo,
  selectTool,
  selectColor,
  selectStrokeWidth,
  undo,
  redo,
  clearAll,
  completeAnnotate
})
</script>

<style scoped>
.annotate-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary, #0f172a);
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
  transform-origin: center center;
  transition: transform 0.1s ease-out;
}

.annotate-canvas-wrapper.panning {
  cursor: grabbing !important;
}

.annotate-canvas-wrapper.panning .annotate-canvas {
  cursor: grabbing !important;
}

.annotate-tool-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background-color: transparent;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 2px;
  color: var(--text-primary, #f1f5f9);
  font-size: 18px;
  cursor: pointer;
  transition: all 0.2s;
}

.annotate-tool-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.05));
  border-color: var(--accent-blue, #3178c6);
}

.annotate-tool-btn.active {
  background-color: var(--accent-blue, #3178c6);
  border-color: var(--accent-blue, #3178c6);
  color: #fff;
}

.annotate-color-btn {
  width: 28px;
  height: 28px;
  border-radius: 2px;
  border: 2px solid transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  position: relative;
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
  width: 32px;
  height: 32px;
  border-radius: 2px;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  background-color: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
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
</style>
