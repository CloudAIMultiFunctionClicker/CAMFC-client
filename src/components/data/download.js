// 文件下载模块
// 通过Tauri调用Rust端的下载功能

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'

/**
 * 下载文件
 * 
 * 调用Rust端的download_file命令
 * 文件会下载到应用内目录，支持分片下载和断点续传
 * 
 * 注意：下载过程可能需要较长时间，特别是大文件
 * 
 * @param {string} fileId - 文件ID（通常是SHA256哈希）
 * @returns {Promise<string>} 下载结果信息
 */
export async function downloadFile(fileId) {
  try {
    console.info(`开始下载文件，文件ID: ${fileId}`)
    
    // 显示下载开始提示
    showToast(`开始下载文件...`, '#3b82f6')
    
    // 调用Rust下载命令
    const result = await invoke('download_file', { fileId })
    
    console.info(`文件下载成功: ${result}`)
    showToast(`文件下载完成`, '#10b981')
    
    return result
  } catch (error) {
    console.error(`文件下载失败: ${error}`)
    
    // 更详细的错误处理
    let errorMessage = '下载失败'
    if (error.includes('获取设备ID失败')) {
      errorMessage = '蓝牙设备连接失败，请检查设备连接'
    } else if (error.includes('获取TOTP失败')) {
      errorMessage = 'TOTP验证失败，请重试'
    } else if (error.includes('网络错误')) {
      errorMessage = '网络连接失败，请检查网络'
    } else if (error.includes('超时')) {
      errorMessage = '下载超时，请重试'
    }
    
    showToast(`${errorMessage}: ${error}`, '#ef4444')
    throw new Error(`下载失败: ${error}`)
  }
}

/**
 * 获取下载进度
 * 
 * 调用Rust端的get_download_progress命令
 * 获取指定文件的下载进度信息
 * 
 * 注意：现在支持真实的分片下载进度追踪
 * 
 * @param {string} fileId - 文件ID
 * @returns {Promise<object>} 下载进度信息
 */
export async function getDownloadProgress(fileId) {
  try {
    const progress = await invoke('get_download_progress', { fileId })
    
    // 添加格式化后的进度信息
    const formattedProgress = {
      ...progress,
      // 确保有进度百分比字段
      progress_percentage: progress.progress_percentage || 
        (progress.total_size > 0 ? 
          Math.round((progress.downloaded / progress.total_size) * 100) : 0),
      // 格式化文件大小显示
      formatted_total_size: progress.total_size > 0 ? 
        formatFileSize(progress.total_size) : '未知大小',
      formatted_downloaded: progress.downloaded > 0 ? 
        formatFileSize(progress.downloaded) : '0 B',
      // 分片信息
      chunks_info: progress.chunks_total > 0 ? 
        `分片 ${progress.chunks_completed}/${progress.chunks_total}` : '分片信息未知'
    }
    
    console.debug(`获取到下载进度: ${fileId} - ${formattedProgress.progress_percentage}%`)
    return formattedProgress
  } catch (error) {
    console.error(`获取下载进度失败: ${error}`)
    // 失败时返回一个默认的进度信息
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
 * 
 * 调用Rust端的pause_download命令
 * 暂停指定文件的下载
 * 
 * @param {string} fileId - 文件ID
 * @returns {Promise<void>}
 */
export async function pauseDownload(fileId) {
  try {
    await invoke('pause_download', { fileId })
    console.info(`已暂停下载: ${fileId}`)
    showToast(`下载已暂停`, '#f59e0b')
  } catch (error) {
    console.error(`暂停下载失败: ${error}`)
    // 暂停失败不抛出错误，因为可能已经暂停或完成了
  }
}

/**
 * 恢复下载
 * 
 * 调用Rust端的resume_download命令
 * 恢复指定文件的下载d
 * 
 * @param {string} fileId - 文件ID
 * @returns {Promise<void>}
 */
export async function resumeDownload(fileId) {
  try {
    await invoke('resume_download', { fileId })
    console.info(`已恢复下载: ${fileId}`)
    showToast(`下载已恢复`, '#3b82f6')
  } catch (error) {
    console.error(`恢复下载失败: ${error}`)
    throw new Error(`恢复下载失败: ${error}`)
  }
}

/**
 * 批量下载文件
 * 
 * 下载多个选中的文件
 * 会依次下载每个文件，显示总体进度
 * 
 * @param {Array<string>} fileIds - 文件ID数组
 * @returns {Promise<Array<string>>} 每个文件的下载结果
 */
export async function batchDownloadFiles(fileIds) {
  if (!fileIds || fileIds.length === 0) {
    showToast('请先选择要下载的文件', '#f59e0b')
    return []
  }
  
  console.info(`批量下载 ${fileIds.length} 个文件`)
  showToast(`开始批量下载 ${fileIds.length} 个文件...`, '#3b82f6')
  
  const results = []
  let successCount = 0
  let errorCount = 0
  
  for (let i = 0; i < fileIds.length; i++) {
    const fileId = fileIds[i]
    try {
      console.info(`下载第 ${i + 1}/${fileIds.length} 个文件: ${fileId}`)
      
      // 显示当前下载进度
      showToast(`下载中 (${i + 1}/${fileIds.length}): ${fileId}`, '#3b82f6')
      
      const result = await downloadFile(fileId)
      results.push({ fileId, success: true, result })
      successCount++
      
    } catch (error) {
      console.error(`文件 ${fileId} 下载失败:`, error)
      results.push({ fileId, success: false, error: error.message })
      errorCount++
    }
  }
  
  // 显示最终结果
  const message = successCount > 0 
    ? `下载完成：${successCount} 个成功，${errorCount} 个失败`
    : '所有文件下载失败'
  
  const color = successCount > 0 ? '#10b981' : '#ef4444'
  showToast(message, color)
  
  console.info(`批量下载完成：${successCount} 成功，${errorCount} 失败`)
  return results
}

// 工具函数：从文件信息中提取文件ID
// 根据后端API，文件ID可能是SHA256哈希或其他唯一标识
export function extractFileId(fileInfo) {
  // TODO: 这里需要根据实际的后端数据结构调整
  // 假设fileInfo有file_id字段，如果没有就使用path或其他唯一标识
  return fileInfo.file_id || fileInfo.path || fileInfo.name
}

/**
 * 格式化文件大小
 * 将字节数转换为可读的格式 (B, KB, MB, GB)
 * 
 * @param {number} bytes - 字节数
 * @returns {string} 格式化后的文件大小
 */
export function formatFileSize(bytes) {
  if (bytes === 0) return '0 B'
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  
  // 最多保留2位小数
  const size = (bytes / Math.pow(1024, i)).toFixed(2)
  return `${size} ${units[i]}`
}

export default {
  downloadFile,
  getDownloadProgress,
  pauseDownload,
  resumeDownload,
  batchDownloadFiles,
  extractFileId,
  formatFileSize
}
