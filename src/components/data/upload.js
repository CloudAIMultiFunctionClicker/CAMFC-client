

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'
import { formatFileSize } from './download.js'
import { getActiveUploads, setActiveUploads } from './storage.js'

export async function uploadFile(filePath) {
  try {
    console.info(`开始上传文件，文件路径: ${filePath}`)

    showToast(`开始上传文件...`, '#3b82f6')

    const result = await invoke('upload_file', { filePath })

    console.info(`文件上传开始: ${result}`)
    showToast(`文件上传已开始`, '#10b981')

    return result
  } catch (error) {
    console.error(`文件上传失败: ${error}`)

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

export async function getUploadProgress(uploadId) {
  try {
    const progress = await invoke('get_upload_progress', { uploadId })

    const formattedProgress = {
      ...progress,

      progress_percentage: progress.progress_percentage ||
        (progress.total_size > 0 ?
          Math.round((progress.uploaded / progress.total_size) * 100) : 0),

      formatted_total_size: progress.total_size > 0 ?
        formatFileSize(progress.total_size) : '未知大小',
      formatted_uploaded: progress.uploaded > 0 ?
        formatFileSize(progress.uploaded) : '0 B',

      chunks_info: progress.chunks_total > 0 ?
        `分片 ${progress.chunks_completed}/${progress.chunks_total}` : '分片信息未知',

      estimated_remaining: '计算中...'
    }

    console.debug(`获取到上传进度: ${uploadId} - ${formattedProgress.progress_percentage}%`)
    return formattedProgress
  } catch (error) {
    console.error(`获取上传进度失败: ${error}`)

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

export async function pauseUpload(uploadId) {
  try {
    await invoke('pause_upload', { uploadId })
    console.info(`已暂停上传: ${uploadId}`)
    showToast(`上传已暂停`, '#f59e0b')
  } catch (error) {
    console.error(`暂停上传失败: ${error}`)

  }
}

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

export async function selectAndUploadFile(targetPath = '') {
  try {

    const targetPathArg = targetPath && targetPath.trim() !== '' ? targetPath : null

    console.info(`调用Rust端select_and_upload_file命令，目标路径: ${targetPathArg || '根目录'}`)

    const result = await invoke('select_and_upload_file', { targetPath: targetPathArg })

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

    console.info(`文件选择成功，upload_id: ${result.upload_id}，目标路径: ${result.target_path || '根目录'}`)
    showToast(`开始上传到 ${targetPath || '根目录'}: ${extractFileName(result.file_path)}`, '#3b82f6')

    const stored = await getActiveUploads()
    stored.push(result.upload_id)
    await setActiveUploads(stored)

    return {
      success: true,
      uploadId: result.upload_id,
      filePath: result.file_path,
      targetPath: result.target_path || ''
    }
  } catch (error) {
    console.error('选择并上传文件失败:', error)
    throw error
  }
}

export async function uploadFilesFromPaths(filePaths, targetPath = '') {
  try {
    console.info(`批量上传 ${filePaths.length} 个文件到目录: ${targetPath || '/'}`)

    if (!filePaths || filePaths.length === 0) {
      showToast('请先选择要上传的文件', '#f59e0b')
      return {
        success: false,
        message: '没有提供文件路径'
      }
    }

    const targetPathArg = targetPath && targetPath.trim() !== '' ? targetPath : null

    console.info(`调用Rust端upload_files_from_paths，参数: filePaths.length=${filePaths.length}, targetPath=${targetPathArg}`)

    const result = await invoke('upload_files_from_paths', {
      filePaths,
      targetPath: targetPathArg
    })

    if (!result.success) {
      throw new Error(result.message || '批量上传失败')
    }

    console.info(`批量上传任务已创建，共 ${result.count} 个文件，目标路径: ${targetPath || '/'}`)
    showToast(`开始上传 ${result.count} 个文件到 ${targetPath || '根目录'}...`, '#3b82f6')

    const stored = await getActiveUploads()
    for (const uploadId of result.upload_ids) {
      if (!stored.includes(uploadId)) {
        stored.push(uploadId)
      }
    }
    await setActiveUploads(stored)

    return {
      success: true,
      uploadIds: result.upload_ids,
      filePaths: result.file_paths,
      count: result.count,
      targetPath: targetPath
    }
  } catch (error) {
    console.error('批量上传文件失败:', error)

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

export function extractFileName(filePath) {

  const parts = filePath.split(/[\\/]/)
  return parts[parts.length - 1] || filePath
}

export async function selectMultipleAndUploadFiles() {
  try {
    console.info('调用Rust端select_and_upload_multiple_files命令')

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

export async function selectFiles() {
  try {
    console.info('调用Rust端select_files命令')

    const result = await invoke('select_files')

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