// CAMFC Client - 文件下载模块
// 通过 Tauri 调用 Rust 端的下载功能，支持分片下载和断点续传

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'
import { getActiveDownloads, setActiveDownloads } from './storage.js'

/**
 * 格式化文件大小
 * 将字节数转换为可读的格式 (B, KB, MB, GB, TB)
 * 
 * @param {number} bytes - 字节数
 * @returns {string} 格式化后的大小，如 "1.50 MB"
 */
export function formatFileSize(bytes) {
  if (bytes === 0) return '0 B'
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  
  const size = (bytes / Math.pow(1024, i)).toFixed(2)
  return `${size} ${units[i]}`
}

/**
 * 下载文件
 * 调用 Rust 端的 download_file 命令，文件会下载到应用内目录
 * 
 * 流程：调用 Rust 接口 → 保存下载记录 → 返回结果
 * 支持分片下载和断点续传，下载过程可能需要较长时间
 * 
 * @param {string} fileId - 文件 ID（通常是 SHA256 哈希）
 * @returns {Promise<string>} 下载结果信息
 */
export async function downloadFile(fileId) {
  try {
    console.info(`开始下载文件，文件 ID: ${fileId}`)
    
    const result = await invoke('download_file', { fileId })
    
    // 保存到活跃下载列表
    const stored = await getActiveDownloads()
    stored.push(fileId)
    await setActiveDownloads(stored)
    
    console.info(`文件下载成功：${result}`)
    return result
  } catch (error) {
    console.error(`文件下载失败：${error}`)
    
    // 根据错误类型给出提示
    let errorMessage = '下载失败'
    if (error.includes('获取设备 ID 失败')) {
      errorMessage = '蓝牙设备连接失败，请检查设备连接'
    } else if (error.includes('获取 TOT P 失败')) {
      errorMessage = 'TOTP 验证失败，请重试'
    } else if (error.includes('网络错误')) {
      errorMessage = '网络连接失败，请检查网络'
    } else if (error.includes('超时')) {
      errorMessage = '下载超时，请重试'
    }
    
    showToast(`${errorMessage}: ${error}`, '#ef4444')
    throw new Error(`下载失败：${error}`)
  }
}

/**
 * 获取下载进度
 * 调用 Rust 端的 get_download_progress 命令获取进度信息
 * 
 * 返回的进度包含格式化后的大小和百分比，支持分片进度追踪
 * 如果获取失败会返回默认的空进度信息
 * 
 * @param {string} fileId - 文件 ID
 * @returns {Promise<object>} 下载进度信息对象
 */
export async function getDownloadProgress(fileId) {
  try {
    const progress = await invoke('get_download_progress', { fileId })
    
    // 计算进度百分比（如果后端没返回就自己算）
    const percentage = progress.progress_percentage || 
      (progress.total_size > 0 ? 
        Math.round((progress.downloaded / progress.total_size) * 100) : 0)
    
    // 组装格式化后的进度数据
    const formattedProgress = {
      ...progress,
      progress_percentage: percentage,
      formatted_total_size: progress.total_size > 0 ? 
        formatFileSize(progress.total_size) : '未知大小',
      formatted_downloaded: progress.downloaded > 0 ? 
        formatFileSize(progress.downloaded) : '0 B',
      chunks_info: progress.chunks_total > 0 ? 
        `分片 ${progress.chunks_completed}/${progress.chunks_total}` : '分片信息未知'
    }
    
    console.debug(`获取到下载进度：${fileId} - ${percentage}%`)
    return formattedProgress
  } catch (error) {
    console.error(`获取下载进度失败：${error}`)
    // 返回默认空进度，避免前端崩溃
    return {
      file_id: fileId,
      file_name: '未知文件',
      total_size: 0,
      downloaded: 0,
      status: 'Error',
      chunks_total: 0,
      chunks_completed: 0,
      speed_kbps: 0,
      progress_percentage: 0,
      formatted_total_size: '未知大小',
      formatted_downloaded: '0 B',
      chunks_info: '分片信息未知'
    }
  }
}

/**
 * 暂停下载
 * 调用 Rust 端的 pause_download 命令
 * 
 * @param {string} fileId - 文件 ID
 * @returns {Promise<void>}
 */
export async function pauseDownload(fileId) {
  try {
    await invoke('pause_download', { fileId })
    console.info(`已暂停下载：${fileId}`)
    showToast(`下载已暂停`, '#f59e0b')
  } catch (error) {
    console.error(`暂停下载失败：${error}`)
    // 暂停失败不抛错，可能已经暂停或完成了
  }
}

/**
 * 恢复下载
 * 调用 Rust 端的 resume_download 命令
 * 
 * @param {string} fileId - 文件 ID
 * @returns {Promise<void>}
 */
export async function resumeDownload(fileId) {
  try {
    await invoke('resume_download', { fileId })
    console.info(`已恢复下载：${fileId}`)
    showToast(`下载已恢复`, '#3b82f6')
  } catch (error) {
    console.error(`恢复下载失败：${error}`)
    throw new Error(`恢复下载失败：${error}`)
  }
}

/**
 * 等待下载完成并校验
 * 轮询检查下载进度，直到完成或超时
 * 
 * @param {string} fileId - 文件 ID
 * @param {number} maxChecks - 最大检查次数，默认 30 次（约 15 秒）
 * @returns {Promise<boolean>} 是否成功完成
 */
async function waitForDownloadComplete(fileId, maxChecks = 30) {
  let checkCount = 0
  
  while (checkCount < maxChecks) {
    await new Promise(resolve => setTimeout(resolve, 500))
    const progress = await getDownloadProgress(fileId)
    
    if (progress.status === 'Completed' && progress.progress_percentage >= 100) {
      console.info(`文件 ${fileId} 下载完成并校验通过`)
      return true
    } else if (progress.status === 'Error') {
      throw new Error(`下载失败：${progress.status}`)
    }
    
    checkCount++
  }
  
  return false
}

/**
 * 批量下载文件
 * 依次下载多个文件，显示总体进度
 * 
 * 流程：遍历文件 → 逐个下载 → 等待完成 → 统计结果
 * 每个文件下载完成后会等待校验通过才继续下一个
 * 
 * @param {Array<string>} fileIds - 文件 ID 数组
 * @returns {Promise<Array>} 每个文件的下载结果数组
 */
export async function batchDownloadFiles(fileIds) {
  if (!fileIds || fileIds.length === 0) {
    showToast('请先选择要下载的文件', '#f59e0b')
    return []
  }
  
  console.info(`批量下载 ${fileIds.length} 个文件`)
  
  const results = []
  let successCount = 0
  let errorCount = 0
  
  for (let i = 0; i < fileIds.length; i++) {
    const fileId = fileIds[i]
    try {
      console.info(`下载第 ${i + 1}/${fileIds.length} 个文件：${fileId}`)
      showToast(`下载中：${fileId}`, '#3b82f6')
      
      const result = await downloadFile(fileId)
      
      // 等待下载完成并校验
      const downloadComplete = await waitForDownloadComplete(fileId)
      
      if (!downloadComplete) {
        throw new Error('下载超时')
      }
      
      results.push({ fileId, success: true, result })
      successCount++
      
    } catch (error) {
      console.error(`文件 ${fileId} 下载失败:`, error)
      results.push({ fileId, success: false, error: error.message })
      errorCount++
    }
  }
  
  // 显示最终统计结果
  const message = successCount > 0 
    ? `下载完成：${successCount} 个成功，${errorCount} 个失败`
    : '所有文件下载失败'
  
  const color = successCount > 0 ? '#10b981' : '#ef4444'
  showToast(message, color)
  
  console.info(`批量下载完成：${successCount} 成功，${errorCount} 失败`)
  return results
}

/**
 * 从文件信息中提取文件 ID
 * 根据后端 API 结构，优先使用 file_id 字段
 * 
 * @param {object} fileInfo - 文件信息对象
 * @returns {string} 文件 ID
 */
export function extractFileId(fileInfo) {
  // TODO: 根据实际后端数据结构调整，现在优先取 file_id
  return fileInfo.file_id || fileInfo.path || fileInfo.name
}

// 导出所有函数
export default {
  downloadFile,
  getDownloadProgress,
  pauseDownload,
  resumeDownload,
  batchDownloadFiles,
  extractFileId,
  formatFileSize
}
