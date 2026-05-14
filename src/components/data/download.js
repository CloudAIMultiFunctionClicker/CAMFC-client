

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'
import { getActiveDownloads, setActiveDownloads } from './storage.js'

export async function downloadFile(fileId) {
  try {
    console.info(`开始下载文件，文件 ID: ${fileId}`)

    const result = await invoke('download_file', { fileId })

    const stored = await getActiveDownloads()
    stored.push(fileId)
    await setActiveDownloads(stored)

    console.info(`文件下载成功：${result}`)

    return result
  } catch (error) {
    console.error(`文件下载失败: ${error}`)

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

export async function getDownloadProgress(fileId) {
  try {
    const progress = await invoke('get_download_progress', { fileId })

    const formattedProgress = {
      ...progress,

      progress_percentage: progress.progress_percentage ||
        (progress.total_size > 0 ?
          Math.round((progress.downloaded / progress.total_size) * 100) : 0),

      formatted_total_size: progress.total_size > 0 ?
        formatFileSize(progress.total_size) : '未知大小',
      formatted_downloaded: progress.downloaded > 0 ?
        formatFileSize(progress.downloaded) : '0 B',

      chunks_info: progress.chunks_total > 0 ?
        `分片 ${progress.chunks_completed}/${progress.chunks_total}` : '分片信息未知'
    }

    console.debug(`获取到下载进度: ${fileId} - ${formattedProgress.progress_percentage}%`)
    return formattedProgress
  } catch (error) {
    console.error(`获取下载进度失败: ${error}`)

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

export async function pauseDownload(fileId) {
  try {
    await invoke('pause_download', { fileId })
    console.info(`已暂停下载: ${fileId}`)
    showToast(`下载已暂停`, '#f59e0b')
  } catch (error) {
    console.error(`暂停下载失败: ${error}`)

  }
}

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

      let downloadComplete = false
      let checkCount = 0
      const maxChecks = 30

      while (!downloadComplete && checkCount < maxChecks) {
        await new Promise(resolve => setTimeout(resolve, 500))
        const progress = await getDownloadProgress(fileId)

        if (progress.status === 'Completed' && progress.progress_percentage >= 100) {
          downloadComplete = true
          console.info(`文件 ${fileId} 下载完成并校验通过`)
        } else if (progress.status === 'Error') {
          throw new Error(`下载失败：${progress.status}`)
        }

        checkCount++
      }

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

  const message = successCount > 0
    ? `下载完成：${successCount} 个成功，${errorCount} 个失败`
    : '所有文件下载失败'

  const color = successCount > 0 ? '#10b981' : '#ef4444'
  showToast(message, color)

  console.info(`批量下载完成：${successCount} 成功，${errorCount} 失败`)
  return results
}

export function extractFileId(fileInfo) {

  return fileInfo.file_id || fileInfo.path || fileInfo.name
}

export function formatFileSize(bytes) {
  if (bytes === 0) return '0 B'

  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))

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
