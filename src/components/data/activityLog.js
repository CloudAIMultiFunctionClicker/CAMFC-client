

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'

export async function getRecentActivities(options = {}) {
  try {
    const { userUuid, limit = 10, activityType } = options

    if (!userUuid) {
      throw new Error('缺少必需的参数：userUuid')
    }

    console.info(`获取最近活动记录，userUuid: ${userUuid}, limit: ${limit}, type: ${activityType || 'all'}`)

    const result = await invoke('get_recent_activities', {
      userUuid: userUuid,
      limit: limit > 100 ? 100 : limit,
      activityType: activityType || undefined
    })

    console.log(`获取到 ${result.activities.length} 条活动记录`)
    return result
  } catch (error) {
    console.error(`获取活动记录失败：${error}`)
    showToast(`获取活动记录失败：${error}`, '#ef4444')
    throw new Error(`获取活动记录失败：${error}`)
  }
}

export async function recordUploadActivity(userUuid, filePath, fileSize) {
  try {
    console.info(`记录上传活动：user=${userUuid}, path=${filePath}, size=${fileSize}`)

    await invoke('record_upload_activity', {
      userUuid: userUuid,
      filePath: filePath,
      fileSize: fileSize
    })

    console.info(`上传活动记录成功`)
  } catch (error) {
    console.error(`记录上传活动失败：${error}`)

  }
}

export async function recordDownloadActivity(userUuid, filePath, fileSize) {
  try {
    console.info(`记录下载活动：user=${userUuid}, path=${filePath}, size=${fileSize}`)

    await invoke('record_download_activity', {
      userUuid: userUuid,
      filePath: filePath,
      fileSize: fileSize
    })

    console.info(`下载活动记录成功`)
  } catch (error) {
    console.error(`记录下载活动失败：${error}`)

  }
}

export async function recordAccessActivity(userUuid, filePath, fileSize) {
  try {
    console.info(`记录访问活动：user=${userUuid}, path=${filePath}, size=${fileSize}`)

    await invoke('record_access_activity', {
      userUuid: userUuid,
      filePath: filePath,
      fileSize: fileSize
    })

    console.info(`访问活动记录成功`)
  } catch (error) {
    console.error(`记录访问活动失败：${error}`)

  }
}

export async function getRecentUploads(userUuid, limit = 10) {
  return getRecentActivities({ userUuid, limit, activityType: 'upload' })
}

export async function getRecentDownloads(userUuid, limit = 10) {
  return getRecentActivities({ userUuid, limit, activityType: 'download' })
}

export async function getRecentAccesses(userUuid, limit = 10) {
  return getRecentActivities({ userUuid, limit, activityType: 'access' })
}

export function formatActivity(activity) {
  const typeLabel = getTypeLabel(activity.type)
  const time = formatActivityTimestamp(activity.timestamp)
  const size = formatFileSize(activity.file_size)

  return `${typeLabel} - ${activity.file_name || activity.file_path} - ${size} - ${time}`
}

export function formatActivityTimestamp(timestamp) {
  try {
    const date = new Date(timestamp)
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    })
  } catch (e) {
    return timestamp
  }
}

export function getTypeLabel(type) {
  const labels = {
    'upload': '上传',
    'download': '下载',
    'access': '访问'
  }
  return labels[type] || type
}

export function getTypeColor(type) {
  const colors = {
    'upload': '#3fb850',
    'download': '#3178c6',
    'access': '#bc8cff'
  }
  return colors[type] || '#6e7681'
}

export function formatFileSize(bytes) {
  if (bytes === 0) return '0 B'

  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
}
