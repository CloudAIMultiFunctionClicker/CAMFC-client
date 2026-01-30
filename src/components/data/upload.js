// 文件上传模块
// 通过Tauri调用Rust端的上传功能

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'
import { formatFileSize } from './download.js'

/**
 * 上传文件
 * 
 * 调用Rust端的upload_file命令
 * 支持分片上传和断点续传，分片大小为4MB
 * 
 * 注意：上传过程可能需要较长时间，特别是大文件
 * 
 * @param {string} filePath - 本地文件路径
 * @returns {Promise<string>} 上传结果信息，包含upload_id
 */
export async function uploadFile(filePath) {
  try {
    console.info(`开始上传文件，文件路径: ${filePath}`)
    
    // 显示上传开始提示
    showToast(`开始上传文件...`, '#3b82f6')
    
    // 调用Rust上传命令
    const result = await invoke('upload_file', { filePath })
    
    console.info(`文件上传开始: ${result}`)
    showToast(`文件上传已开始`, '#10b981')
    
    return result
  } catch (error) {
    console.error(`文件上传失败: ${error}`)
    
    // 更详细的错误处理
    let errorMessage = '上传失败'
    if (error.includes('获取设备ID失败')) {
      errorMessage = '蓝牙设备连接失败，请检查设备连接'
    } else if (error.includes('获取TOTP失败')) {
      errorMessage = 'TOTP验证失败，请重试'
    } else if (error.includes('网络错误')) {
      errorMessage = '网络连接失败，请检查网络'
    } else if (error.includes('创建上传任务失败')) {
      errorMessage = '创建上传任务失败，请重试'
    }
    
    showToast(`${errorMessage}: ${error}`, '#ef4444')
    throw new Error(`上传失败: ${error}`)
  }
}

/**
 * 获取上传进度
 * 
 * 调用Rust端的get_upload_progress命令
 * 获取指定上传任务的进度信息
 * 
 * @param {string} uploadId - 上传会话ID
 * @returns {Promise<object>} 上传进度信息
 */
export async function getUploadProgress(uploadId) {
  try {
    const progress = await invoke('get_upload_progress', { uploadId })
    
    // 添加格式化后的进度信息
    const formattedProgress = {
      ...progress,
      // 确保有进度百分比字段
      progress_percentage: progress.progress_percentage || 
        (progress.total_size > 0 ? 
          Math.round((progress.uploaded / progress.total_size) * 100) : 0),
      // 格式化文件大小显示
      formatted_total_size: progress.total_size > 0 ? 
        formatFileSize(progress.total_size) : '未知大小',
      formatted_uploaded: progress.uploaded > 0 ? 
        formatFileSize(progress.uploaded) : '0 B',
      // 分片信息
      chunks_info: progress.chunks_total > 0 ? 
        `分片 ${progress.chunks_completed}/${progress.chunks_total}` : '分片信息未知',
      // 估计剩余时间（简单估算）
      estimated_remaining: '计算中...'
    }
    
    console.debug(`获取到上传进度: ${uploadId} - ${formattedProgress.progress_percentage}%`)
    return formattedProgress
  } catch (error) {
    console.error(`获取上传进度失败: ${error}`)
    // 失败时返回一个默认的进度信息
    return {
      upload_id: uploadId,
      filename: '未知文件',
      total_size: 0,
      uploaded: 0,
      status: 'Error',
      chunks_total: 0,
      chunks_completed: 0,
      speed_kbps: 0,
      progress_percentage: 0,
      formatted_total_size: '未知大小',
      formatted_uploaded: '0 B',
      chunks_info: '分片信息未知',
      estimated_remaining: '未知'
    }
  }
}

/**
 * 暂停上传
 * 
 * 调用Rust端的pause_upload命令
 * 暂停指定上传任务
 * 
 * @param {string} uploadId - 上传会话ID
 * @returns {Promise<void>}
 */
export async function pauseUpload(uploadId) {
  try {
    await invoke('pause_upload', { uploadId })
    console.info(`已暂停上传: ${uploadId}`)
    showToast(`上传已暂停`, '#f59e0b')
  } catch (error) {
    console.error(`暂停上传失败: ${error}`)
    // 暂停失败不抛出错误，因为可能已经暂停或完成了
  }
}

/**
 * 恢复上传
 * 
 * 调用Rust端的resume_upload命令
 * 恢复指定上传任务
 * 
 * @param {string} uploadId - 上传会话ID
 * @returns {Promise<void>}
 */
export async function resumeUpload(uploadId) {
  try {
    await invoke('resume_upload', { uploadId })
    console.info(`已恢复上传: ${uploadId}`)
    showToast(`上传已恢复`, '#3b82f6')
  } catch (error) {
    console.error(`恢复上传失败: ${error}`)
    throw new Error(`恢复上传失败: ${error}`)
  }
}

/**
 * 选择文件并上传
 * 
 * 调用Rust端的select_and_upload_file命令
 * Rust端会使用系统原生文件对话框选择文件，然后开始上传
 * 
 * @returns {Promise<object>} 上传结果信息
 */
export async function selectAndUploadFile() {
  try {
    console.info('调用Rust端select_and_upload_file命令')
    
    // 调用Rust端的文件选择和上传命令
    const result = await invoke('select_and_upload_file')
    
    if (!result.success) {
      if (result.cancelled) {
        console.info('用户取消了文件选择')
        return {
          success: false,
          cancelled: true
        }
      }
      throw new Error('文件选择失败')
    }
    
    console.info(`文件选择成功，upload_id: ${result.upload_id}`)
    showToast(`开始上传: ${extractFileName(result.file_path)}`, '#3b82f6')
    
    return {
      success: true,
      uploadId: result.upload_id,
      filePath: result.file_path
    }
  } catch (error) {
    console.error('选择并上传文件失败:', error)
    throw error
  }
}

/**
 * 批量上传文件（从文件路径列表）
 * 
 * 调用Rust端的upload_files_from_paths命令
 * 前端提供文件路径列表，后端依次上传每个文件
 * 
 * @param {Array<string>} filePaths - 文件路径数组
 * @returns {Promise<object>} 上传结果信息
 */
export async function uploadFilesFromPaths(filePaths) {
  try {
    console.info(`批量上传 ${filePaths.length} 个文件`)
    
    if (!filePaths || filePaths.length === 0) {
      showToast('请先选择要上传的文件', '#f59e0b')
      return {
        success: false,
        message: '没有提供文件路径'
      }
    }
    
    // 调用Rust端的批量上传命令
    const result = await invoke('upload_files_from_paths', { filePaths })
    
    if (!result.success) {
      throw new Error(result.message || '批量上传失败')
    }
    
    console.info(`批量上传任务已创建，共 ${result.count} 个文件`)
    showToast(`开始上传 ${result.count} 个文件...`, '#3b82f6')
    
    return {
      success: true,
      uploadIds: result.upload_ids,
      filePaths: result.file_paths,
      count: result.count
    }
  } catch (error) {
    console.error('批量上传文件失败:', error)
    
    // 更详细的错误处理
    let errorMessage = '上传失败'
    if (error.includes('获取设备ID失败')) {
      errorMessage = '蓝牙设备连接失败，请检查设备连接'
    } else if (error.includes('获取TOTP失败')) {
      errorMessage = 'TOTP验证失败，请重试'
    } else if (error.includes('网络错误')) {
      errorMessage = '网络连接失败，请检查网络'
    } else if (error.includes('创建上传任务失败')) {
      errorMessage = '创建上传任务失败，请重试'
    }
    
    showToast(`${errorMessage}: ${error}`, '#ef4444')
    throw new Error(`批量上传失败: ${error}`)
  }
}

/**
 * 批量上传文件
 * 
 * 上传多个选中的文件
 * 会依次上传每个文件，显示总体进度
 * 
 * @param {Array<string>} filePaths - 文件路径数组
 * @returns {Promise<Array<string>>} 每个文件的上传结果
 */
export async function batchUploadFiles(filePaths) {
  if (!filePaths || filePaths.length === 0) {
    showToast('请先选择要上传的文件', '#f59e0b')
    return []
  }
  
  console.info(`批量上传 ${filePaths.length} 个文件`)
  showToast(`开始批量上传 ${filePaths.length} 个文件...`, '#3b82f6')
  
  const results = []
  let successCount = 0
  let errorCount = 0
  
  for (let i = 0; i < filePaths.length; i++) {
    const filePath = filePaths[i]
    try {
      console.info(`上传第 ${i + 1}/${filePaths.length} 个文件: ${filePath}`)
      
      // 显示当前上传进度
      showToast(`上传中 (${i + 1}/${filePaths.length}): ${extractFileName(filePath)}`, '#3b82f6')
      
      const result = await uploadFile(filePath)
      results.push({ filePath, success: true, result })
      successCount++
      
    } catch (error) {
      console.error(`文件 ${filePath} 上传失败:`, error)
      results.push({ filePath, success: false, error: error.message })
      errorCount++
    }
  }
  
  // 显示最终结果
  const message = successCount > 0 
    ? `上传完成：${successCount} 个成功，${errorCount} 个失败`
    : '所有文件上传失败'
  
  const color = successCount > 0 ? '#10b981' : '#ef4444'
  showToast(message, color)
  
  console.info(`批量上传完成：${successCount} 成功，${errorCount} 失败`)
  return results
}

/**
 * 从文件路径中提取文件名
 * 
 * @param {string} filePath - 文件路径
 * @returns {string} 文件名
 */
export function extractFileName(filePath) {
  // 简单实现：从路径中提取最后一部分作为文件名
  const parts = filePath.split(/[\\/]/)
  return parts[parts.length - 1] || filePath
}

/**
 * 选择多个文件并上传
 * 
 * 调用Rust端的select_and_upload_multiple_files命令
 * Rust端会使用系统原生文件对话框选择多个文件，然后开始批量上传
 * 
 * @returns {Promise<Array<object>>} 上传结果信息数组
 */
export async function selectMultipleAndUploadFiles() {
  try {
    console.info('调用Rust端select_and_upload_multiple_files命令')
    
    // 调用Rust端的多文件选择和上传命令
    const result = await invoke('select_and_upload_multiple_files')
    
    if (!result.success) {
      if (result.cancelled) {
        console.info('用户取消了文件选择')
        return {
          success: false,
          cancelled: true
        }
      }
      throw new Error('文件选择失败')
    }
    
    console.info(`选择了 ${result.count} 个文件，开始批量上传`)
    showToast(`准备上传 ${result.count} 个文件...`, '#3b82f6')
    
    return {
      success: true,
      uploadIds: result.upload_ids,
      filePaths: result.file_paths,
      count: result.count
    }
  } catch (error) {
    console.error('选择并上传多个文件失败:', error)
    throw error
  }
}

export default {
  uploadFile,
  getUploadProgress,
  pauseUpload,
  resumeUpload,
  selectAndUploadFile,
  selectMultipleAndUploadFiles,
  batchUploadFiles,
  uploadFilesFromPaths,
  extractFileName
}