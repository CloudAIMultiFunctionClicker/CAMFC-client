/**
 * CAMFC Client - 文件上传模块
 * 通过 Tauri 调用 Rust 端的上传功能
 *
 * 保留所有权利
 *
 * Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
 * Email: abc.cxh2009@foxmail.com
 *
 * Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
 * Email: 1220594170@qq.com
 *
 * Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
 * Email: admin@mc666.top
 *
 * 文件上传模块
 * 通过 Tauri 调用 Rust 端的上传功能
 */

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'
import { formatFileSize } from './download.js'
import { getActiveUploads, setActiveUploads } from './storage.js'

// ==================== 工具函数 ====================

/**
 * 格式化上传进度信息
 * 
 * 统一处理进度数据的格式化，添加计算字段
 * 
 * @param {object} progress - 原始进度数据
 * @returns {object} 格式化后的进度信息
 */
function formatProgressData(progress) {
  return {
    ...progress,
    // 进度百分比（Rust 可能没给，自己算一个）
    progress_percentage: progress.progress_percentage || 
      (progress.total_size > 0 ? 
        Math.round((progress.uploaded / progress.total_size) * 100) : 0),
    // 格式化文件大小
    formatted_total_size: progress.total_size > 0 ? 
      formatFileSize(progress.total_size) : '未知大小',
    formatted_uploaded: progress.uploaded > 0 ? 
      formatFileSize(progress.uploaded) : '0 B',
    // 分片信息
    chunks_info: progress.chunks_total > 0 ? 
      `分片 ${progress.chunks_completed}/${progress.chunks_total}` : '分片信息未知',
    // 剩余时间估算（简单搞搞，后面可以优化）
    estimated_remaining: '计算中...'
  }
}

/**
 * 统一的错误处理
 * 
 * 根据错误类型返回友好的错误消息
 * 
 * @param {string} error - 错误信息
 * @returns {string} 友好的错误提示
 */
function handleUploadError(error) {
  const errorMap = {
    '获取设备 ID 失败': '蓝牙设备连接失败，请检查设备连接',
    '获取 TOT P 失败': 'TOTP 验证失败，请重试',
    '网络错误': '网络连接失败，请检查网络',
    '创建上传任务失败': '创建上传任务失败，请重试'
  }
  
  for (const [key, msg] of Object.entries(errorMap)) {
    if (error.includes(key)) {
      return msg
    }
  }
  return '上传失败'
}

/**
 * 保存上传 ID 到本地存储
 * 
 * 用于在传输页面显示进度
 * 
 * @param {Array<string>} uploadIds - 上传 ID 列表
 */
async function saveUploadIds(uploadIds) {
  const stored = await getActiveUploads()
  for (const id of uploadIds) {
    if (!stored.includes(id)) {
      stored.push(id)
    }
  }
  await setActiveUploads(stored)
}

/**
 * 从文件路径中提取文件名
 * 
 * @param {string} filePath - 文件路径
 * @returns {string} 文件名
 */
export function extractFileName(filePath) {
  const parts = filePath.split(/[\\/]/)
  return parts[parts.length - 1] || filePath
}

// ==================== 核心上传功能 ====================

/**
 * 上传单个文件
 * 
 * 调用 Rust 端的 upload_file 命令
 * 支持分片上传和断点续传，分片大小为 256KB
 * 
 * @param {string} filePath - 本地文件路径
 * @returns {Promise<string>} 上传结果信息，包含 upload_id
 */
export async function uploadFile(filePath) {
  try {
    console.info(`开始上传：${filePath}`)
    showToast('开始上传文件...', '#3b82f6')
    
    const result = await invoke('upload_file', { filePath })
    
    console.info(`上传已开始：${result}`)
    showToast('文件上传已开始', '#10b981')
    
    return result
  } catch (error) {
    console.error(`上传失败：${error}`)
    const msg = handleUploadError(error)
    showToast(`${msg}: ${error}`, '#ef4444')
    throw new Error(`上传失败：${error}`)
  }
}

/**
 * 获取上传进度
 * 
 * 调用 Rust 端的 get_upload_progress 命令
 * 获取指定上传任务的进度信息
 * 
 * @param {string} uploadId - 上传会话 ID
 * @returns {Promise<object>} 上传进度信息
 */
export async function getUploadProgress(uploadId) {
  try {
    const progress = await invoke('get_upload_progress', { uploadId })
    const formattedProgress = formatProgressData(progress)
    
    console.debug(`获取进度：${uploadId} - ${formattedProgress.progress_percentage}%`)
    return formattedProgress
  } catch (error) {
    console.error(`获取进度失败：${error}`)
    // 失败时返回默认进度信息
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
 * 调用 Rust 端的 pause_upload 命令
 * 暂停指定上传任务
 * 
 * @param {string} uploadId - 上传会话 ID
 * @returns {Promise<void>}
 */
export async function pauseUpload(uploadId) {
  try {
    await invoke('pause_upload', { uploadId })
    console.info(`已暂停：${uploadId}`)
    showToast('上传已暂停', '#f59e0b')
  } catch (error) {
    console.error(`暂停失败：${error}`)
    // 暂停失败不抛错误，可能已经暂停或完成了
  }
}

/**
 * 恢复上传
 * 
 * 调用 Rust 端的 resume_upload 命令
 * 恢复指定上传任务
 * 
 * @param {string} uploadId - 上传会话 ID
 * @returns {Promise<void>}
 */
export async function resumeUpload(uploadId) {
  try {
    await invoke('resume_upload', { uploadId })
    console.info(`已恢复：${uploadId}`)
    showToast('上传已恢复', '#3b82f6')
  } catch (error) {
    console.error(`恢复失败：${error}`)
    throw new Error(`恢复上传失败：${error}`)
  }
}

// ==================== 文件选择与上传 ====================

/**
 * 选择文件并上传（支持指定目标路径）
 * 
 * 调用 Rust 端的 select_and_upload_file 命令
 * Rust 端会使用系统原生文件对话框选择文件，然后开始上传到指定目录
 * 
 * @param {string} targetPath - 目标路径（相对于用户存储目录），可选
 * @returns {Promise<object>} 上传结果信息
 */
export async function selectAndUploadFile(targetPath = '') {
  try {
    const targetPathArg = targetPath && targetPath.trim() !== '' ? targetPath : null
    
    console.info(`选择并上传，目标路径：${targetPathArg || '根目录'}`)
    
    const result = await invoke('select_and_upload_file', { targetPath: targetPathArg })
    
    if (!result.success) {
      if (result.cancelled) {
        console.info('用户取消了选择')
        return { success: false, cancelled: true }
      }
      throw new Error('文件选择失败')
    }
    
    console.info(`选择成功，upload_id: ${result.upload_id}`)
    showToast(`开始上传到 ${targetPath || '根目录'}: ${extractFileName(result.file_path)}`, '#3b82f6')
    
    await saveUploadIds([result.upload_id])
    
    return {
      success: true,
      uploadId: result.upload_id,
      filePath: result.file_path,
      targetPath: result.target_path || ''
    }
  } catch (error) {
    console.error('选择并上传失败:', error)
    throw error
  }
}

/**
 * 批量上传文件（从文件路径列表，上传到指定目录）
 * 
 * 调用 Rust 端的 upload_files_from_paths 命令
 * 前端提供文件路径列表和目标路径，后端依次上传每个文件到指定目录
 * 
 * @param {Array<string>} filePaths - 文件路径数组
 * @param {string} targetPath - 目标路径（相对于用户存储目录）
 * @returns {Promise<object>} 上传结果信息
 */
export async function uploadFilesFromPaths(filePaths, targetPath = '') {
  try {
    console.info(`批量上传 ${filePaths.length} 个文件到：${targetPath || '/'}`)
    
    if (!filePaths || filePaths.length === 0) {
      showToast('请先选择要上传的文件', '#f59e0b')
      return { success: false, message: '没有提供文件路径' }
    }
    
    const targetPathArg = targetPath && targetPath.trim() !== '' ? targetPath : null
    
    console.info(`调用 Rust 端，参数：filePaths.length=${filePaths.length}, targetPath=${targetPathArg}`)
    
    const result = await invoke('upload_files_from_paths', { 
      filePaths,
      targetPath: targetPathArg
    })
    
    if (!result.success) {
      throw new Error(result.message || '批量上传失败')
    }
    
    console.info(`批量上传任务已创建，共 ${result.count} 个文件`)
    showToast(`开始上传 ${result.count} 个文件到 ${targetPath || '根目录'}...`, '#3b82f6')
    
    await saveUploadIds(result.upload_ids)
    
    return {
      success: true,
      uploadIds: result.upload_ids,
      filePaths: result.file_paths,
      count: result.count,
      targetPath: targetPath
    }
  } catch (error) {
    console.error('批量上传失败:', error)
    const msg = handleUploadError(error)
    showToast(`${msg}: ${error}`, '#ef4444')
    throw new Error(`批量上传失败：${error}`)
  }
}

/**
 * 批量上传文件（前端逐个上传）
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
      console.info(`上传第 ${i + 1}/${filePaths.length} 个文件：${filePath}`)
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
  
  const message = successCount > 0 
    ? `上传完成：${successCount} 个成功，${errorCount} 个失败`
    : '所有文件上传失败'
  
  const color = successCount > 0 ? '#10b981' : '#ef4444'
  showToast(message, color)
  
  console.info(`批量上传完成：${successCount} 成功，${errorCount} 失败`)
  return results
}

// ==================== 多文件选择 ====================

/**
 * 选择多个文件并上传
 * 
 * 调用 Rust 端的 select_and_upload_multiple_files 命令
 * Rust 端会使用系统原生文件对话框选择多个文件，然后开始批量上传
 * 
 * @returns {Promise<Array<object>>} 上传结果信息数组
 */
export async function selectMultipleAndUploadFiles() {
  try {
    console.info('选择多个文件并上传')
    
    const result = await invoke('select_and_upload_multiple_files')
    
    if (!result.success) {
      if (result.cancelled) {
        console.info('用户取消了选择')
        return { success: false, cancelled: true }
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
    console.error('选择多个文件失败:', error)
    throw error
  }
}

/**
 * 选择多个文件（只选择，不上传）
 * 
 * 调用 Rust 端的 select_files 命令
 * Rust 端会使用系统原生文件对话框选择多个文件，返回文件路径列表
 * 这个命令只负责选择文件，不执行上传操作
 * 
 * @returns {Promise<object>} 文件选择结果
 */
export async function selectFiles() {
  try {
    console.info('选择多个文件')
    
    const result = await invoke('select_files')
    
    if (!result.success) {
      if (result.cancelled) {
        console.info('用户取消了选择')
        return { success: false, cancelled: true }
      }
      throw new Error('文件选择失败')
    }
    
    console.info(`选择了 ${result.count} 个文件`)
    return {
      success: true,
      files: result.files,
      count: result.count
    }
  } catch (error) {
    console.error('选择文件失败:', error)
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
  selectFiles,
  extractFileName
}
