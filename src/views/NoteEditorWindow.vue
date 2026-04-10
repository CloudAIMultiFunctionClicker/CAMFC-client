<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<template>
  <div class="editor-window" :class="{ 'light-mode': isLightMode }">
    <div class="editor-header" data-tauri-drag-region>
      <input 
        v-model="noteTitle" 
        class="editor-title-input" 
        placeholder="未命名笔记"
        type="text"
      />
      <div class="editor-actions">
        <button class="action-btn window-btn" @click="minimizeWindow" title="最小化">
          <i class="ri-subtract-line"></i>
        </button>
        <button class="action-btn window-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
          <i :class="isMaximized ? 'ri-fullscreen-exit-line' : 'ri-fullscreen-line'"></i>
        </button>
        <button class="action-btn save-btn" @click="saveAndClose" title="保存">
          <i class="ri-check-line"></i>
        </button>
        <button class="action-btn close-btn" @click="handleClose" title="关闭">
          <i class="ri-close-line"></i>
        </button>
      </div>
    </div>
    
    <div class="editor-body">
      <div 
        class="editor-container" 
        :class="{ 'dragging-over': isDraggingOver }" 
        data-tauri-drag-region
        @paste="handlePaste" 
        @dragenter="handleDragEnter" 
        @dragleave="handleDragLeave" 
        @drop="handleDrop"
      >
        <div
          ref="editorTextarea"
          class="note-editor-content"
          contenteditable="true"
          placeholder="使用 Markdown 格式书写... 支持 Ctrl+V 粘贴图片、拖拽图片到此处"
          @input="handleEditorInput"
          @keydown="handleEditorKeydown"
        ></div>
      </div>
    </div>
    
    <div class="editor-toolbar">
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('h1')">
          <i class="ri-h-1"></i>
        </button>
        <span class="tooltip">一级标题<span class="tooltip-syntax">语法: # 标题</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('h2')">
          <i class="ri-h-2"></i>
        </button>
        <span class="tooltip">二级标题<span class="tooltip-syntax">语法: ## 标题</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('h3')">
          <i class="ri-h-3"></i>
        </button>
        <span class="tooltip">三级标题<span class="tooltip-syntax">语法: ### 标题</span></span>
      </div>
      <div class="toolbar-divider"></div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('bold')">
          <i class="ri-bold"></i>
        </button>
        <span class="tooltip">加粗<span class="tooltip-syntax">语法: **文本**</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('italic')">
          <i class="ri-italic"></i>
        </button>
        <span class="tooltip">斜体<span class="tooltip-syntax">语法: *文本*</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('strike')">
          <i class="ri-strikethrough"></i>
        </button>
        <span class="tooltip">删除线<span class="tooltip-syntax">语法: ~~文本~~</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('code')">
          <i class="ri-code-line"></i>
        </button>
        <span class="tooltip">行内代码<span class="tooltip-syntax">语法: `代码`</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="insertMarkdown('list')">
          <i class="ri-list-unordered"></i>
        </button>
        <span class="tooltip">列表<span class="tooltip-syntax">语法: - 项目</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="handleImageClick">
          <i class="ri-image-line"></i>
        </button>
        <span class="tooltip">图片<span class="tooltip-syntax">Ctrl+V 粘贴或拖拽图片</span></span>
      </div>
      <div class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="aiAnalyzeImages" :disabled="isAiAnalyzing">
          <i class="ri-robot-line"></i>
        </button>
        <span class="tooltip">AI 解释图片<span class="tooltip-syntax">分析笔记中的图片</span></span>
      </div>
    </div>

    <!-- AI 解释结果显示区域 -->
    <div v-if="aiAnalysisResult" class="ai-analysis-result">
      <div class="ai-analysis-header">
        <i class="ri-robot-line"></i>
        <span>AI 解释结果</span>
        <button class="ai-close-btn" @click="closeAiResult">
          <i class="ri-close-line"></i>
        </button>
      </div>
      <div class="ai-analysis-content" v-html="renderAiResult(aiAnalysisResult)"></div>
    </div>

    <!-- 保存确认弹窗 -->
    <Transition name="modal">
      <div v-if="showSaveConfirmModal" class="modal-overlay" @click="cancelClose">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-save-line"></i> 保存更改</h3>
            <button class="close-btn" @click="cancelClose">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body save-modal-body">
            <p>您对笔记做了更改，是否保存？</p>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="discardChanges">不保存</button>
            <button class="confirm-btn" @click="confirmSave">保存</button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { emit, listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../components/layout/showToast.js'
import { getBackendUrl } from '../config/backend.js'

const timeOut = 3000
const route = useRoute()

// 笔记数据
const noteUuid = ref('')
const noteTitle = ref('')
const noteContent = ref('')
const originalContent = ref('')

// 编辑器引用
const editorTextarea = ref(null)
const isDraggingOver = ref(false)
const showSaveConfirmModal = ref(false)

// AI 分析状态
const isAiAnalyzing = ref(false)
const aiAnalysisResult = ref(null)

// 窗口状态
const isMaximized = ref(false)
const currentWindow = getCurrentWindow()

// 主题状态
const isLightMode = ref(false)

// 获取认证头
async function getAuthHeader() {
  try {
    const { getDeviceId, getTotp } = await import('../components/data/bluetooth.js')
    const deviceId = await getDeviceId()
    const currentTotp = await getTotp()
    return { "Id": deviceId, "Totp": currentTotp }
  } catch {
    return {}
  }
}

// API 请求
async function apiRequest(url, data = {}) {
  const authHeader = await getAuthHeader()
  const response = await axios.post(getBackendUrl() + url, data, {
    headers: { ...authHeader, 'Content-Type': 'application/json' },
    timeout: timeOut
  })
  return response.data
}

// 上传 base64 图片到服务器
async function uploadBase64Image(base64Data) {
  try {
    const authHeader = await getAuthHeader()
    
    // 验证认证信息是否存在
    if (!authHeader.Id || !authHeader.Totp) {
      throw new Error('认证信息缺失，请检查设备连接状态')
    }
    
    // 生成文件名
    const filename = `image_${Date.now()}.png`
    
    console.log('上传图片:', {
      url: getBackendUrl() + '/upload/base64?target_path=.note/.images',
      filename: filename,
      hasAuth: !!authHeader.Id
    })
    
    // 使用新的 base64 上传 API
    const response = await axios.post(
      getBackendUrl() + '/upload/base64?target_path=.note/.images',
      {
        filename: filename,
        base64_data: base64Data
      },
      {
        headers: {
          'Id': authHeader.Id,
          'Totp': authHeader.Totp,
          'Content-Type': 'application/json'
        },
        timeout: timeOut
      }
    )
    
    console.log('上传成功:', response.data)
    
    return {
      success: true,
      filename: response.data.filename,
      file_id: response.data.file_id,
      path: `.note/.images/${response.data.filename}`
    }
  } catch (error) {
    console.error('上传图片失败:', error)
    if (error.response) {
      console.error('错误响应:', error.response.data)
    }
    throw error
  }
}

// AI 分析图片
async function aiAnalyzeImages() {
  isAiAnalyzing.value = true
  aiAnalysisResult.value = null
  
  try {
    // 获取笔记中的所有图片
    const images = extractImagesFromContent(noteContent.value)
    
    if (images.length === 0) {
      showToast('笔记中没有找到图片', '#f59e0b')
      isAiAnalyzing.value = false
      return
    }
    
    showToast(`正在分析 ${images.length} 张图片...`, '#3b82f6')
    
    // 调用后端 Tauri 命令进行 AI 分析
    const authHeader = await getAuthHeader()
    const deviceId = authHeader.Id
    const totp = authHeader.Totp
    
    // 处理 base64 图片：上传并替换路径
    const processedImages = []
    const imageMappings = []
    for (const imageInfo of images) {
      if (imageInfo.isBase64) {
        try {
          showToast(`上传图片 ${imageInfo.filename}...`, '#3b82f6')
          const uploadResult = await uploadBase64Image(imageInfo.url)
          
          if (uploadResult.success && uploadResult.filename) {
            const newPath = `/note/image/${deviceId}/${uploadResult.filename}`
            processedImages.push({
              filename: uploadResult.filename,
              url: newPath,
              isBase64: false
            })
            imageMappings.push({
              oldPath: imageInfo.url,
              newPath: newPath
            })
          } else {
            processedImages.push(imageInfo)
          }
        } catch (error) {
          console.error(`上传图片失败: ${imageInfo.filename}`, error)
          processedImages.push(imageInfo)
        }
      } else {
        processedImages.push(imageInfo)
      }
    }
    
    // 更新笔记内容中的图片路径
    if (imageMappings.length > 0) {
      noteContent.value = updateImagePathsInContent(noteContent.value, imageMappings)
    }
    
    // 依次分析每张图片
    const results = []
    for (const imageInfo of processedImages) {
      try {
        // 构建文件路径（笔记图片在 .note/.images 目录下）
        const filePath = `.note/.images/${imageInfo.filename}`
        
        console.log(`查询分析状态：${filePath}`)
        
        // 使用 HTTP API 查询分析状态
        const statusResponse = await axios.get(
          getBackendUrl() + `/ai/analysis/hash-status/${encodeURIComponent(filePath)}`,
          {
            headers: {
              'Id': authHeader.Id,
              'Totp': authHeader.Totp
            },
            timeout: timeOut
          }
        )
        
        const status = statusResponse.data
        console.log(`分析状态：`, status)
        
        // 如果未分析，触发重新分析
        if (!status.analyzed) {
          console.log(`图片未分析，触发 AI 分析：${filePath}`)
          showToast(`正在分析图片：${imageInfo.filename}...`, '#3b82f6')
          
          // 调用重新分析 API
          const reanalyzeResponse = await axios.post(
            getBackendUrl() + `/ai/analysis/reanalyze/${encodeURIComponent(filePath)}`,
            null,
            {
              headers: {
                'Id': authHeader.Id,
                'Totp': authHeader.Totp
              },
              timeout: timeOut
            }
          )
          
          console.log(`触发 AI 分析成功：`, reanalyzeResponse.data)
          
          // 等待分析完成（轮询）
          let analysisComplete = false
          let maxAttempts = 30 // 最多等待 30 秒
          let attempts = 0
          
          while (!analysisComplete && attempts < maxAttempts) {
            await new Promise(resolve => setTimeout(resolve, 1000)) // 等待 1 秒
            attempts++
            
            try {
              // 检查认证头
              const authHeader = await getAuthHeader()
              console.log(`轮询请求 - 认证头:`, authHeader)
              console.log(`轮询请求 - URL:`, getBackendUrl() + `/ai/analysis/hash-status/${encodeURIComponent(filePath)}`)
              
              const checkStatusResponse = await axios.get(
                getBackendUrl() + `/ai/analysis/hash-status/${encodeURIComponent(filePath)}`,
                {
                  headers: {
                    'Id': authHeader.Id,
                    'Totp': authHeader.Totp
                  },
                  timeout: timeOut
                }
              )
              
              const checkStatus = checkStatusResponse.data
              console.log(`轮询分析状态 (${attempts}/${maxAttempts}):`, checkStatus)
              
              if (checkStatus.analyzed) {
                analysisComplete = true
                status.analyzed = true
                // 从 analysis_info 获取分析结果，并保存到 status.analysis_info
                status.analysis_info = checkStatus.analysis_info || null
                console.log(`分析完成：`, status.analysis_info)
              }
            } catch (e) {
              console.error(`轮询分析状态失败 (${attempts}):`, e)
              console.error('错误详情:', {
                message: e.message,
                code: e.code,
                response: e.response?.data,
                status: e.response?.status
              })
            }
          }
          
          if (!analysisComplete) {
            console.warn(`分析超时：${filePath}`)
            showToast(`分析超时，请稍后重试`, '#f59e0b')
          }
        }
        
        // 如果已分析但没有 analysis_info，获取完整的分析结果
        if (status.analyzed && !status.analysis_info) {
          try {
            const authHeader = await getAuthHeader()
            console.log(`获取完整分析结果 - 认证头:`, authHeader)
            console.log(`获取完整分析结果 - URL:`, getBackendUrl() + `/ai/analysis/${encodeURIComponent(filePath)}`)
            
            const analysisResponse = await axios.get(
              getBackendUrl() + `/ai/analysis/${encodeURIComponent(filePath)}`,
              {
                headers: {
                  'Id': authHeader.Id,
                  'Totp': authHeader.Totp
                },
                timeout: timeOut
              }
            )
            // 从返回的数据中提取 analysis_info
            status.analysis_info = analysisResponse.data.data || analysisResponse.data.analysis || null
            console.log(`获取分析结果成功：`, status.analysis_info)
          } catch (e) {
            console.error(`获取分析结果失败：${imageInfo.filename}`, e)
            console.error('错误详情:', {
              message: e.message,
              code: e.code,
              response: e.response?.data,
              status: e.response?.status
            })
          }
        }
        
        results.push({
          filename: imageInfo.filename,
          filePath: filePath,
          status: status,
          analyzed: status.analyzed || false
        })
      } catch (error) {
        console.error(`查询分析状态失败：${imageInfo.filename}`, error)
        if (error.response) {
          console.error('错误响应:', error.response.data)
        }
        results.push({
          filename: imageInfo.filename,
          error: error.response?.data?.detail || error.message || '分析失败'
        })
      }
    }
    
    // 显示分析结果
    aiAnalysisResult.value = {
      images: results,
      total: images.length,
      analyzedCount: results.filter(r => r.analyzed).length
    }
    
    showToast(`分析完成: ${aiAnalysisResult.value.analyzedCount}/${aiAnalysisResult.value.total} 张图片`, '#10b981')
    
  } catch (error) {
    console.error('AI 分析失败:', error)
    showToast('AI 分析失败: ' + error.message, '#ef4444')
  } finally {
    isAiAnalyzing.value = false
  }
}

// 从笔记内容中提取图片
function extractImagesFromContent(content) {
  const images = []
  // 匹配 Markdown 图片语法 ![alt](url)
  const markdownImageRegex = /!\[([^\]]*)\]\(([^)]+)\)/g
  let match
  while ((match = markdownImageRegex.exec(content)) !== null) {
    const url = match[2]
    let filename
    if (url.startsWith('data:')) {
      filename = `image_base64_${Date.now()}_${Math.random().toString(36).substring(2, 10)}.png`
    } else if (url.startsWith('/note/image/')) {
      filename = url.split('/').pop() || 'image.png'
    } else {
      filename = url.split('/').pop() || 'image.png'
    }
    images.push({
      alt: match[1],
      url: url,
      filename: filename,
      isBase64: url.startsWith('data:')
    })
  }
  // 匹配 HTML 图片标签
  const htmlImageRegex = /<img[^>]+src="([^"]+)"[^>]*>/g
  while ((match = htmlImageRegex.exec(content)) !== null) {
    const url = match[1]
    let filename
    if (url.startsWith('data:')) {
      filename = `image_base64_${Date.now()}_${Math.random().toString(36).substring(2, 10)}.png`
    } else if (url.startsWith('/note/image/')) {
      filename = url.split('/').pop() || 'image.png'
    } else {
      filename = url.split('/').pop() || 'image.png'
    }
    images.push({
      alt: '',
      url: url,
      filename: filename,
      isBase64: url.startsWith('data:')
    })
  }
  // 去重
  const uniqueImages = []
  const seenUrls = new Set()
  for (const img of images) {
    if (!seenUrls.has(img.url)) {
      seenUrls.add(img.url)
      uniqueImages.push(img)
    }
  }
  return uniqueImages
}

// 渲染 AI 结果
function renderAiResult(result) {
  let html = '<div class="ai-results-list">'
  
  for (const imageResult of result.images) {
    html += `<div class="ai-image-result">
      <div class="ai-image-info">
        <span class="ai-image-name">${imageResult.filename}</span>
        <span class="ai-image-status ${imageResult.analyzed ? 'analyzed' : 'not-analyzed'}">
          ${imageResult.analyzed ? '已分析' : '未分析'}
        </span>
      </div>`
    
    if (imageResult.error) {
      html += `<div class="ai-image-error">错误: ${imageResult.error}</div>`
    } else if (imageResult.status && imageResult.status.analysis_info) {
      const analysis = imageResult.status.analysis_info
      const aiData = analysis.result || analysis // 兼容两种格式
      
      // 优先显示完整的 AI 分析内容
      if (aiData && aiData.summary) {
        // 显示完整的 AI 分析内容
        html += `<div class="ai-analysis-content">
          <div class="ai-summary">
            <strong>📝 摘要：</strong>
            <p>${aiData.summary}</p>
          </div>`
        
        if (aiData.content_type) {
          html += `<div class="ai-detail-item">
            <span class="ai-detail-label">类型:</span>
            <span class="ai-detail-value">${aiData.content_type}</span>
          </div>`
        }
        
        if (aiData.subject) {
          html += `<div class="ai-detail-item">
            <span class="ai-detail-label">主题:</span>
            <span class="ai-detail-value">${aiData.subject}</span>
          </div>`
        }
        
        if (aiData.key_points && aiData.key_points.length > 0) {
          html += `<div class="ai-key-points">
            <strong>🔑 关键点：</strong>
            <ul>`
          for (const point of aiData.key_points) {
            html += `<li>${point}</li>`
          }
          html += `</ul></div>`
        }
        
        if (aiData.difficulty) {
          html += `<div class="ai-detail-item">
            <span class="ai-detail-label">难度:</span>
            <span class="ai-detail-value">${aiData.difficulty}</span>
          </div>`
        }
        
        if (aiData.confidence) {
          html += `<div class="ai-detail-item">
            <span class="ai-detail-label">置信度:</span>
            <span class="ai-detail-value">${(aiData.confidence * 100).toFixed(1)}%</span>
          </div>`
        }
        
        // 分析时间可能在 result.analyzed_at 或 metadata.analyzed_at
        const analyzeTime = aiData.analyzed_at || analysis.last_analyzed_at || (analysis.metadata && analysis.metadata.analyzed_at)
        if (analyzeTime) {
          html += `<div class="ai-detail-item">
            <span class="ai-detail-label">分析时间:</span>
            <span class="ai-detail-value">${new Date(analyzeTime).toLocaleString('zh-CN')}</span>
          </div>`
        }
        
        html += `</div>`
      } else {
        // 备用：显示基本状态
        html += `<div class="ai-image-details">
          <div class="ai-detail-item">
            <span class="ai-detail-label">状态:</span>
            <span class="ai-detail-value">${imageResult.analyzed ? '已分析' : '未分析'}</span>
          </div>
        </div>`
      }
    }
    
    html += '</div>'
  }
  
  html += '</div>'
  
  if (result.analyzedCount < result.total) {
    html += `<div class="ai-action-area">
      <button class="ai-reanalyze-btn" onclick="window.__reanalyzeAllImages()">
        <i class="ri-refresh-line"></i>
        重新分析所有图片
      </button>
    </div>`
  }
  
  return html
}

// 重新分析所有图片
async function reanalyzeAllImages() {
  if (!aiAnalysisResult.value) return
  
  const images = aiAnalysisResult.value.images
  const total = images.length
  let completed = 0
  let failed = 0
  
  showToast(`开始重新分析 ${total} 张图片...`, '#3b82f6')
  
  // 处理 base64 图片：上传并替换路径
  const processedImages = []
  const imageMappings = []
  for (const imageResult of images) {
    // 检查是否需要重新上传 base64 图片
    if (imageResult.filename.startsWith('image_base64_')) {
      try {
        const authHeader = await getAuthHeader()
        const deviceId = authHeader.Id
        
        // 从笔记内容中找到原始 base64 数据
        const content = noteContent.value
        const base64Pattern = /data:image\/[^;]+;base64,([^"]+)/g
        let match
        while ((match = base64Pattern.exec(content)) !== null) {
          const base64Data = match[1]
          if (base64Data.substring(0, 20) === imageResult.filename.substring(16, 36)) {
            const uploadResult = await uploadBase64Image(match[0])
            
            if (uploadResult.success && uploadResult.filename) {
              const newPath = `/note/image/${deviceId}/${uploadResult.filename}`
              processedImages.push({
                filename: uploadResult.filename,
                url: newPath,
                isBase64: false
              })
              imageMappings.push({
                oldPath: match[0],
                newPath: newPath
              })
              break
            }
          }
        }
      } catch (error) {
        console.error(`上传图片失败: ${imageResult.filename}`, error)
        processedImages.push(imageResult)
      }
    } else {
      processedImages.push(imageResult)
    }
  }
  
  // 更新笔记内容中的图片路径
  if (imageMappings.length > 0) {
    noteContent.value = updateImagePathsInContent(noteContent.value, imageMappings)
  }
  
  // 依次重新分析每张图片
  for (const imageResult of processedImages) {
    try {
      // 构建文件路径（笔记图片在 .note/.images 目录下）
      const filePath = imageResult.filePath || `.note/.images/${imageResult.filename}`
      
      // 使用 HTTP API 重新解析
      const authHeader = await getAuthHeader()
      const reanalyzeResponse = await axios.post(
        getBackendUrl() + `/ai/analysis/reanalyze/${encodeURIComponent(filePath)}`,
        {},
        {
          headers: authHeader,
          timeout: timeOut
        }
      )
      
      console.log(`重新分析 ${imageResult.filename}:`, reanalyzeResponse.data)
      completed++
    } catch (error) {
      console.error(`重新分析 ${imageResult.filename} 失败:`, error)
      failed++
    }
  }
  
  if (failed === 0) {
    showToast(`重新分析完成 (${completed}/${total})`, '#10b981')
  } else {
    showToast(`重新分析完成: 成功 ${completed}, 失败 ${failed}`, '#f59e0b')
  }
  
  // 重新获取分析结果
  await aiAnalyzeImages()
}

// 关闭 AI 结果
function closeAiResult() {
  aiAnalysisResult.value = null
}

// 初始化
onMounted(async () => {
  // 从 URL 参数获取笔记信息（content 不再从 URL 获取）
  const uuid = route.query.uuid
  const title = route.query.title
  
  if (!uuid) {
    showToast('笔记信息不完整', '#ef4444')
    setTimeout(() => closeWindow(), 1500)
    return
  }
  
  noteUuid.value = uuid
  noteTitle.value = title || '未命名笔记'
  // content 初始为空，等待主窗口发送
  noteContent.value = ''
  originalContent.value = ''
  
  // 监听主窗口发送的内容
  const unlistenContent = await listen('load-note-content', (event) => {
    const content = event.payload?.content || ''
    noteContent.value = content
    originalContent.value = content
    // 初始化编辑器内容
    if (editorTextarea.value) {
      editorTextarea.value.innerHTML = renderMarkdown(content)
    }
  })
  
  // 监听自动 AI 分析指令
  const unlistenAutoAnalyze = await listen('auto-ai-analyze', async () => {
    console.log('收到自动 AI 分析指令')
    if (!isAiAnalyzing.value) {
      await aiAnalyzeImages()
    }
  })
  
  // 保存 unlisten 函数用于卸载
  window._unlistenContent = unlistenContent
  window._unlistenAutoAnalyze = unlistenAutoAnalyze
  
  // 暴露重新分析函数到全局窗口对象
  window.__reanalyzeAllImages = reanalyzeAllImages
  
  // 监听保存快捷键
  document.addEventListener('keydown', handleGlobalKeydown)
  
  // 检查窗口状态
  checkWindowState()
  
  // 初始化主题
  initTheme()
  
  // 监听主题变化
  setupThemeListener()
  
  // 通知主窗口刷新笔记列表（打开时）
  try {
    const { emit } = await import('@tauri-apps/api/event')
    await emit('note-editor-opened', { uuid })
  } catch (e) {
    console.error('发送打开事件失败:', e)
  }
})

function cleanup() {
  document.removeEventListener('keydown', handleGlobalKeydown)
  if (window._unlistenContent) {
    window._unlistenContent()
  }
  if (window._unlistenAutoAnalyze) {
    window._unlistenAutoAnalyze()
  }
  if (window.__reanalyzeAllImages) {
    delete window.__reanalyzeAllImages
  }
}

onUnmounted(() => {
  cleanup()
})

// 检查窗口状态
async function checkWindowState() {
  try {
    isMaximized.value = await currentWindow.isMaximized()
  } catch (error) {
    console.error('检查窗口状态失败:', error)
  }
}

// 最小化窗口
async function minimizeWindow() {
  try {
    await currentWindow.minimize()
  } catch (error) {
    console.error('最小化窗口失败:', error)
  }
}

// 切换最大化
async function toggleMaximize() {
  try {
    if (isMaximized.value) {
      await currentWindow.unmaximize()
    } else {
      await currentWindow.maximize()
    }
    // 等待一小段时间让窗口状态更新
    setTimeout(() => {
      checkWindowState()
    }, 50)
  } catch (error) {
    console.error('切换最大化失败:', error)
  }
}

// 初始化主题
function initTheme() {
  try {
    const savedTheme = localStorage.getItem('theme-preference')
    if (savedTheme === 'light' || savedTheme === 'dark') {
      isLightMode.value = savedTheme === 'light'
    } else {
      // 检测系统偏好
      isLightMode.value = window.matchMedia('(prefers-color-scheme: light)').matches
    }
  } catch (e) {
    console.error('初始化主题失败:', e)
    isLightMode.value = false
  }
}

// 监听主题变化
function setupThemeListener() {
  try {
    // 监听来自主窗口的主题变化事件
    listen('theme-changed', (event) => {
      const theme = event.payload
      isLightMode.value = theme === 'light'
      console.log('收到主题变化事件:', theme)
    })
  } catch (e) {
    console.error('设置主题监听失败:', e)
  }
}

// 全局键盘事件（Ctrl+S 保存）
function handleGlobalKeydown(e) {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    saveNote()
  }
}

// 关闭窗口
async function closeWindow() {
  try {
    // 先通知主窗口刷新笔记列表
    try {
      const { emit } = await import('@tauri-apps/api/event')
      await emit('note-editor-closed', { uuid: noteUuid.value })
    } catch (e) {
      console.error('发送关闭事件失败:', e)
    }
    // 等待 0.1s 让主窗口刷新
    await new Promise(resolve => setTimeout(resolve, 100))
    // 再关闭子窗口
    const appWindow = getCurrentWindow()
    await appWindow.close()
  } catch (e) {
    console.error('关闭窗口失败:', e)
    showToast('关闭失败', '#ef4444')
  }
}

// 保存并关闭
async function saveAndClose() {
  await saveNote()
  await closeWindow()
}

// 处理关闭按钮
function handleClose() {
  if (noteContent.value !== originalContent.value) {
    showSaveConfirmModal.value = true
  } else {
    closeWindow()
  }
}

// 确认保存
async function confirmSave() {
  await saveNote()
  showSaveConfirmModal.value = false
  await closeWindow()
}

// 放弃更改
async function discardChanges() {
  showSaveConfirmModal.value = false
  await closeWindow()
}

// 取消关闭
function cancelClose() {
  showSaveConfirmModal.value = false
}

// 更新笔记内容中的图片路径
function updateImagePathsInContent(content, imageMappings) {
  let updatedContent = content
  
  for (const mapping of imageMappings) {
    if (mapping.oldPath && mapping.newPath) {
      updatedContent = updatedContent.replace(mapping.oldPath, mapping.newPath)
    }
  }
  
  return updatedContent
}

// 保存笔记
async function saveNote() {
  try {
    // 处理 base64 图片：上传并替换路径
    await processNoteContent()
    
    // 如果是新建笔记（没有 UUID），先创建笔记
    if (!noteUuid.value) {
      showToast('创建新笔记...', '#3b82f6')
      const createResponse = await apiRequest('/note/add', { 
        title: noteTitle.value || '无标题笔记'
      })
      
      // 获取新创建的笔记 UUID
      noteUuid.value = createResponse.uuid
      showToast('笔记已创建，继续保存...', '#10b981')
    }
    
    // 更新笔记内容
    await apiRequest('/note/update', { 
      uuid: noteUuid.value, 
      content: noteContent.value || '',
      title: noteTitle.value 
    })
    originalContent.value = noteContent.value
    showToast('保存成功', '#10b981')
    
    // 通知主窗口刷新笔记列表
    await emit('note-saved', { uuid: noteUuid.value })
  } catch (e) {
    console.error('保存笔记失败:', e)
    showToast('保存失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

// 编辑器输入处理
function handleEditorInput() {
  if (editorTextarea.value) {
    const html = editorTextarea.value.innerHTML
    noteContent.value = convertHtmlToMarkdown(html)
  }
}

// 编辑器键盘事件
function handleEditorKeydown(e) {
  if (e.key === 'Enter' && !e.shiftKey) {
    const selection = window.getSelection()
    if (selection.rangeCount > 0) {
      const range = selection.getRangeAt(0)
      const container = range.startContainer
      if (container.nodeType === Node.TEXT_NODE && container.textContent.startsWith('#')) {
        e.preventDefault()
        document.execCommand('insertHTML', false, '<div><br></div>')
      }
    }
  }
}

// Markdown 渲染（用于初始化显示）
function renderMarkdown(text) {
  if (!text) return ''
  
  let html = text
    .replace(/^### (.*$)/gim, '<h3>$1</h3>')
    .replace(/^## (.*$)/gim, '<h2>$1</h2>')
    .replace(/^# (.*$)/gim, '<h1>$1</h1>')
    .replace(/\*\*(.*)\*\*/gim, '<strong>$1</strong>')
    .replace(/\*(.*)\*/gim, '<em>$1</em>')
    .replace(/~~(.*)~~/gim, '<del>$1</del>')
    .replace(/`([^`]+)`/gim, '<code>$1</code>')
    .replace(/^- (.*$)/gim, '<li>$1</li>')
    .replace(/!\[([^\]]*)\]\(([^)]+)\)/gim, '<div class="markdown-image-wrapper"><img src="$2" alt="$1" class="markdown-image" onerror="this.style.display=\'none\'; this.nextSibling && (this.nextSibling.style.display=\'flex\')"></div><div class="markdown-image-error" style="display:none"><i class="ri-image-line"></i><span>图片加载失败</span></div>')
    .replace(/\n/gim, '<br>')
  
  return html
}

// HTML 转 Markdown
function convertHtmlToMarkdown(html) {
  let text = html
    .replace(/<div><br><\/div>/g, '\n')
    .replace(/<div>(.*?)<\/div>/g, '$1\n')
    .replace(/<h1>(.*?)<\/h1>/g, '# $1\n')
    .replace(/<h2>(.*?)<\/h2>/g, '## $1\n')
    .replace(/<h3>(.*?)<\/h3>/g, '### $1\n')
    .replace(/<strong>(.*?)<\/strong>/g, '**$1**')
    .replace(/<b>(.*?)<\/b>/g, '**$1**')
    .replace(/<em>(.*?)<\/em>/g, '*$1*')
    .replace(/<i>(.*?)<\/i>/g, '*$1*')
    .replace(/<del>(.*?)<\/del>/g, '~~$1~~')
    .replace(/<code>(.*?)<\/code>/g, '`$1`')
    .replace(/<li>(.*?)<\/li>/g, '- $1\n')
    .replace(/<ul>|<\/ul>|<ol>|<\/ol>/g, '')
    .replace(/<br\s*\/?>/g, '\n')
    .replace(/<img src="([^"]*)" alt="([^"]*)"[^>]*>/g, '![$2]($1)')
    .replace(/<span class="markdown-image-wrapper">/g, '')
    .replace(/<div class="markdown-image-error"[^>]*>.*?<\/div>/g, '')
    .replace(/<div class="img-error">.*?<\/div>/g, '')
    .replace(/<[^>]+>/g, '')
    .replace(/&nbsp;/g, ' ')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&amp;/g, '&')
  return text.trim()
}

// 粘贴处理
async function handlePaste(event) {
  const clipboardData = event.clipboardData
  if (!clipboardData) return

  const items = clipboardData.items
  for (let i = 0; i < items.length; i++) {
    if (items[i].type.startsWith('image/')) {
      event.preventDefault()
      const blob = items[i].getAsFile()
      if (blob) {
        await insertImageFromBlob(blob)
      }
      return
    }
  }
}

// 从 Blob 插入图片
function insertImageFromBlob(blob) {
  return new Promise((resolve) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      const base64 = e.target.result
      insertMarkdownImage(base64)
      resolve()
    }
    reader.readAsDataURL(blob)
  })
}

// 插入 Markdown 图片
function insertMarkdownImage(imageData) {
  const editor = editorTextarea.value
  if (!editor) return

  const imgHtml = `<div class="markdown-image-wrapper"><img src="${imageData}" alt="截图_${Date.now()}" class="markdown-image" onerror="this.style.display='none'; this.nextSibling && (this.nextSibling.style.display='flex')"></div><div class="markdown-image-error" style="display:none"><i class="ri-image-line"></i><span>图片加载失败</span></div><div><br></div>`

  const selection = window.getSelection()
  if (selection.rangeCount > 0) {
    const range = selection.getRangeAt(0)
    range.deleteContents()
    const tempDiv = document.createElement('div')
    tempDiv.innerHTML = imgHtml
    range.insertNode(tempDiv)
    range.collapse(false)
  } else {
    editor.innerHTML += imgHtml
  }

  noteContent.value = convertHtmlToMarkdown(editor.innerHTML)
  showToast('图片已插入', '#10b981')
}

// 拖拽处理
function handleDragEnter(e) {
  e.preventDefault()
  isDraggingOver.value = true
}

function handleDragLeave(e) {
  e.preventDefault()
  isDraggingOver.value = false
}

async function handleDrop(e) {
  e.preventDefault()
  isDraggingOver.value = false

  const files = e.dataTransfer.files
  if (files.length === 0) return

  const file = files[0]
  if (!file.type.startsWith('image/')) {
    showToast('仅支持图片文件', '#f59e0b')
    return
  }

  await insertImageFromBlob(file)
}

// 图片按钮点击
function handleImageClick() {
  navigator.clipboard.read().then(items => {
    for (const item of items) {
      if (item.types.some(type => type.startsWith('image/'))) {
        item.getType('image/').then(blob => {
          insertImageFromBlob(blob)
        })
        return
      }
    }
    showToast('剪贴板无图片，可直接 Ctrl+V 粘贴或拖拽图片', '#f59e0b')
  }).catch(() => {
    showToast('可直接 Ctrl+V 粘贴或拖拽图片到编辑器', '#f59e0b')
  })
}

// 插入 Markdown
function insertMarkdown(type) {
  const editor = editorTextarea.value
  if (!editor) return

  const selection = window.getSelection()
  let selectedText = ''
  if (selection.rangeCount > 0) {
    selectedText = selection.toString()
  }

  let insert = ''
  switch (type) {
    case 'h1':
      insert = selectedText ? `<h1>${selectedText}</h1>` : '<h1>标题</h1>'
      break
    case 'h2':
      insert = selectedText ? `<h2>${selectedText}</h2>` : '<h2>标题</h2>'
      break
    case 'h3':
      insert = selectedText ? `<h3>${selectedText}</h3>` : '<h3>标题</h3>'
      break
    case 'bold':
      insert = selectedText ? `<strong>${selectedText}</strong>` : '<strong>加粗文本</strong>'
      break
    case 'italic':
      insert = selectedText ? `<em>${selectedText}</em>` : '<em>斜体文本</em>'
      break
    case 'strike':
      insert = selectedText ? `<del>${selectedText}</del>` : '<del>删除线</del>'
      break
    case 'code':
      insert = selectedText ? `<code>${selectedText}</code>` : '<code>代码</code>'
      break
    case 'list':
      insert = selectedText ? `<li>${selectedText}</li>` : '<li>列表项</li>'
      break
    case 'image':
      showToast('请使用 Ctrl+V 粘贴或拖拽图片', '#f59e0b')
      return
  }

  if (selection.rangeCount > 0) {
    const range = selection.getRangeAt(0)
    range.deleteContents()
    const tempDiv = document.createElement('div')
    tempDiv.innerHTML = insert
    range.insertNode(tempDiv)
    range.collapse(false)
  } else {
    editor.innerHTML += insert
  }

  noteContent.value = convertHtmlToMarkdown(editor.innerHTML)
}
</script>

<style scoped>
/* 主题变量定义 */
.editor-window {
  /* 暗色主题（默认） */
  --bg-primary: #0d1117;
  --bg-secondary: #161b22;
  --bg-tertiary: #21262d;
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
}

.editor-window.light-mode {
  /* 亮色主题 */
  --bg-primary: #ffffff;
  --bg-secondary: #f6f8fa;
  --bg-tertiary: #eaeef2;
  --text-primary: #24292f;
  --text-secondary: #57606a;
  --text-muted: #8c959f;
  --border-color: #d0d7de;
  --accent-blue: #0969da;
  --accent-blue-rgb: 9, 105, 218;
  --accent-blue-bright: #0550ae;
  --accent-green: #2da44e;
  --accent-green-rgb: 45, 164, 78;
  --accent-red: #cf222e;
  --accent-red-rgb: 207, 34, 46;
  --hover-bg: #f3f4f6;
}

.editor-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  -webkit-app-region: drag;
}

.editor-title-input {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  background-color: transparent;
  border: none;
  outline: none;
  padding: 4px 8px;
  border-radius: .375rem;
  flex: 1;
  min-width: 0;
  transition: all 0.2s;
}

.editor-title-input:focus {
  background-color: var(--bg-primary);
}

.editor-title-input::placeholder {
  color: var(--text-muted);
}

.editor-actions {
  display: flex;
  gap: 6px;
  -webkit-app-region: no-drag;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  background: none;
  border: none;
  border-radius: .375rem;
  color: var(--text-secondary, #8b949e);
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.save-btn {
  color: #238636;
}

.save-btn:hover {
  background-color: rgba(35, 134, 54, 0.15);
}

.close-btn:hover {
  color: #f85149;
  background-color: rgba(248, 81, 73, 0.15);
}

.window-btn {
  color: var(--text-secondary);
}

.window-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}

.editor-body {
  flex: 1;
  overflow: hidden;
  padding: 16px;
}

.editor-container {
  width: 100%;
  height: 100%;
  overflow-y: auto;
}

.editor-container.dragging-over {
  border: 2px dashed var(--accent-blue);
  background-color: rgba(88, 166, 255, 0.1);
  border-radius: .375rem;
}

.note-editor-content {
  width: 100%;
  min-height: 100%;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 15px;
  line-height: 1.7;
  outline: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.note-editor-content:empty::before {
  content: attr(placeholder);
  color: var(--text-muted, #6e7681);
  pointer-events: none;
  display: block;
}

.note-editor-content :deep(h1),
.note-editor-content :deep(h2),
.note-editor-content :deep(h3) {
  margin: 16px 0 10px;
  color: var(--text-primary, #c9d1d9);
  font-weight: 600;
}

.note-editor-content :deep(h1) {
  font-size: 24px;
  border-bottom: 1px solid var(--border-color, #30363d);
  padding-bottom: 8px;
}

.note-editor-content :deep(h2) {
  font-size: 20px;
}

.note-editor-content :deep(h3) {
  font-size: 16px;
}

.note-editor-content :deep(code) {
  background-color: rgba(110, 118, 129, 0.4);
  padding: 2px 6px;
  border-radius: .375rem;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 14px;
  color: #ff7b72;
}

.note-editor-content :deep(strong) {
  font-weight: 600;
}

.note-editor-content :deep(del) {
  color: var(--text-muted, #6e7681);
}

.note-editor-content :deep(li) {
  margin-left: 20px;
  margin-bottom: 4px;
}

.note-editor-content :deep(.markdown-image) {
  max-width: 100%;
  border-radius: .375rem;
  margin: 12px 0;
  border: 1px solid var(--border-color, #30363d);
}

.note-editor-content :deep(.markdown-image-wrapper) {
  display: inline-block;
  width: 100%;
}

.note-editor-content :deep(.markdown-image-error) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 20px;
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  color: var(--text-muted, #6e7681);
  margin: 12px 0;
}

.editor-toolbar {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
  z-index: 10;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: none;
  border: none;
  border-radius: .375rem;
  color: var(--text-secondary, #8b949e);
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
  color: var(--text-primary, #c9d1d9);
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background-color: var(--border-color, #30363d);
  margin: 0 4px;
}

.toolbar-btn-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toolbar-btn-wrapper:hover .tooltip {
  opacity: 1;
  visibility: visible;
  transform: translateX(-50%) translateY(0);
}

.tooltip {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%) translateY(5px);
  padding: 8px 12px;
  background-color: var(--bg-primary, #0d1117);
  color: var(--text-primary, #c9d1d9);
  font-size: 12px;
  white-space: nowrap;
  border-radius: .375rem;
  border: 1px solid var(--border-color, #30363d);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  opacity: 0;
  visibility: hidden;
  transition: all 0.2s ease;
  pointer-events: none;
  margin-bottom: 8px;
  z-index: 100;
}

.tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 6px solid transparent;
  border-top-color: var(--bg-primary, #0d1117);
}

.tooltip-syntax {
  display: block;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px dashed var(--border-color, #30363d);
  font-family: 'Monaco', 'Menlo', monospace;
  color: var(--accent-blue, #58a6ff);
  font-size: 11px;
}

/* 弹窗样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-secondary, #161b22);
  border-radius: .375rem;
  width: 90%;
  max-width: 400px;
  border: 1px solid var(--border-color, #30363d);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, #30363d);
}

.modal-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--text-primary, #c9d1d9);
  display: flex;
  align-items: center;
  gap: 8px;
}

.modal-body {
  padding: 20px;
}

.save-modal-body p {
  margin: 0;
  color: var(--text-secondary, #8b949e);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color, #30363d);
}

.cancel-btn {
  padding: 8px 16px;
  background-color: transparent;
  color: var(--text-primary, #c9d1d9);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.cancel-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.confirm-btn {
  padding: 8px 16px;
  background-color: #238636;
  color: white;
  border: none;
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.confirm-btn:hover {
  background-color: #2ea043;
}

/* 动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* 竖屏适配 */
@media (max-width: 600px) {
  .editor-header {
    padding: 10px 12px;
  }
  
  .editor-title {
    font-size: 13px;
    max-width: 50%;
  }
  
  .editor-body {
    padding: 12px;
  }
  
  .editor-toolbar {
    bottom: 12px;
    right: 12px;
    padding: 6px 8px;
    gap: 4px;
  }
  
  .toolbar-btn {
    width: 28px;
    height: 28px;
    font-size: 14px;
  }
  
  .toolbar-divider {
    height: 16px;
    margin: 0 2px;
  }
}

.ai-analysis-result {
  position: fixed;
  bottom: 80px;
  right: 20px;
  width: 350px;
  max-height: 400px;
  background-color: var(--card-bg, #ffffff);
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  overflow: hidden;
  animation: slideInRight 0.3s ease;
}

.ai-analysis-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: var(--ai-header-bg, #f0f9ff);
  border-bottom: 1px solid var(--ai-header-border, #e0e7ff);
}

.ai-analysis-header i {
  color: var(--ai-header-icon, #3b82f6);
  font-size: 18px;
  margin-right: 8px;
}

.ai-analysis-header span {
  font-weight: 600;
  color: var(--ai-header-text, #1e40af);
  font-size: 14px;
}

.ai-close-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--ai-close-btn, #64748b);
  font-size: 16px;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.ai-close-btn:hover {
  background-color: var(--ai-close-btn-hover, #e2e8f0);
  color: var(--ai-close-btn-hover-text, #0f172a);
}

.ai-analysis-content {
  padding: 16px;
  overflow-y: auto;
  max-height: 320px;
}

.ai-results-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.ai-image-result {
  padding: 12px;
  background-color: var(--ai-result-bg, #f8fafc);
  border-radius: 8px;
  border: 1px solid var(--ai-result-border, #e2e8f0);
}

.ai-image-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.ai-image-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--ai-image-name, #1e293b);
  word-break: break-all;
}

.ai-image-status {
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
}

.ai-image-status.analyzed {
  background-color: var(--ai-status-analyzed-bg, #dcfce7);
  color: var(--ai-status-analyzed-text, #166534);
}

.ai-image-status.not-analyzed {
  background-color: var(--ai-status-not-analyzed-bg, #f1f5f9);
  color: var(--ai-status-not-analyzed-text, #64748b);
}

.ai-image-error {
  font-size: 12px;
  color: var(--ai-error-text, #ef4444);
  margin-top: 4px;
}

.ai-image-details {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--ai-details-border, #e2e8f0);
}

.ai-detail-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  margin-bottom: 4px;
}

.ai-detail-label {
  color: var(--ai-detail-label, #64748b);
  font-weight: 500;
}

.ai-detail-value {
  color: var(--ai-detail-value, #475569);
  word-break: break-all;
}

.ai-detail-row {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  font-size: 12px;
  margin-bottom: 6px;
  line-height: 1.5;
}

.ai-detail-row .ai-detail-label {
  flex-shrink: 0;
  min-width: 70px;
  font-weight: 600;
}

.ai-detail-row .ai-detail-value {
  flex: 1;
  word-break: break-word;
}

.ai-analysis-details {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--ai-details-border, #e2e8f0);
}

.ai-hash-info {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--ai-details-border, #e2e8f0);
  font-family: 'Monaco', 'Menlo', monospace;
}

.hash-value {
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 10px;
}

.ai-action-area {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--ai-action-border, #e2e8f0);
  text-align: center;
}

.ai-reanalyze-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  width: 100%;
  padding: 10px 16px;
  background-color: var(--ai-reanalyze-btn, #3b82f6);
  color: var(--ai-reanalyze-btn-text, #ffffff);
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.ai-reanalyze-btn:hover {
  background-color: var(--ai-reanalyze-btn-hover, #2563eb);
}

.ai-reanalyze-btn:active {
  transform: scale(0.98);
}

.ai-reanalyze-btn i {
  font-size: 14px;
}

@keyframes slideInRight {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@media (max-width: 768px) {
  .ai-analysis-result {
    width: calc(100% - 40px);
    bottom: 120px;
  }
}
</style>
