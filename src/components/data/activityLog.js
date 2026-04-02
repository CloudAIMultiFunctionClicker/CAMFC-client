/**
 * CAMFC Client - 活动日志模块
 * 通过 Tauri 调用 Rust 端的活动日志功能
 *
 * 保留所有权利
 *
 * Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
 * Email: abc.cxh09@foxmail.com
 *
 * Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
 * Email: 1220594170@qq.com
 *
 * Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
 * Email: admin@mc666.top
 *
 * 活动日志模块
 * 记录用户的最近操作（上传、下载、文件访问）
 */

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../layout/showToast.js'

/**
 * 获取最近活动记录
 * 
 * 调用 Rust 端的 get_recent_activities 命令
 * 查询用户最近操作过的云端文件记录
 * 
 * @param {object} options - 查询选项
 * @param {string} options.userUuid - 用户 UUID（必需）
 * @param {number} [options.limit=10] - 返回最近 N 条记录，默认 10，最大 100
 * @param {string} [options.activityType] - 过滤活动类型
 *   * 'upload' - 上传操作
 *   * 'download' - 下载操作
 *   * 'access' - 文件访问（HEAD 请求）
 * @returns {Promise<object>} 活动记录响应
 */
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

/**
 * 记录上传活动
 * 
 * 调用 Rust 端的 record_upload_activity 命令
 * 在文件上传完成后自动调用
 * 
 * @param {string} userUuid - 用户 UUID
 * @param {string} filePath - 文件路径（相对于用户存储目录）
 * @param {number} fileSize - 文件大小（字节）
 * @returns {Promise<void>}
 */
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
    // 记录失败不抛出错误，避免影响主流程
  }
}

/**
 * 记录下载活动
 * 
 * 调用 Rust 端的 record_download_activity 命令
 * 在文件下载完成后自动调用
 * 
 * @param {string} userUuid - 用户 UUID
 * @param {string} filePath - 文件路径（相对于用户存储目录）
 * @param {number} fileSize - 文件大小（字节）
 * @returns {Promise<void>}
 */
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
    // 记录失败不抛出错误，避免影响主流程
  }
}

/**
 * 记录访问活动
 * 
 * 调用 Rust 端的 record_access_activity 命令
 * 在文件访问（HEAD 请求）时调用
 * 
 * @param {string} userUuid - 用户 UUID
 * @param {string} filePath - 文件路径（相对于用户存储目录）
 * @param {number} fileSize - 文件大小（字节）
 * @returns {Promise<void>}
 */
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
    // 记录失败不抛出错误，避免影响主流程
  }
}

/**
 * 获取最近上传记录
 * 
 * @param {string} userUuid - 用户 UUID
 * @param {number} [limit=10] - 返回最近 N 条记录
 * @returns {Promise<object>} 活动记录响应
 */
export async function getRecentUploads(userUuid, limit = 10) {
  return getRecentActivities({ userUuid, limit, activityType: 'upload' })
}

/**
 * 获取最近下载记录
 * 
 * @param {string} userUuid - 用户 UUID
 * @param {number} [limit=10] - 返回最近 N 条记录
 * @returns {Promise<object>} 活动记录响应
 */
export async function getRecentDownloads(userUuid, limit = 10) {
  return getRecentActivities({ userUuid, limit, activityType: 'download' })
}

/**
 * 获取最近访问记录
 * 
 * @param {string} userUuid - 用户 UUID
 * @param {number} [limit=10] - 返回最近 N 条记录
 * @returns {Promise<object>} 活动记录响应
 */
export async function getRecentAccesses(userUuid, limit = 10) {
  return getRecentActivities({ userUuid, limit, activityType: 'access' })
}

/**
 * 格式化活动记录
 * 
 * @param {object} activity - 活动记录对象
 * @returns {string} 格式化后的字符串
 */
export function formatActivity(activity) {
  const typeLabel = getTypeLabel(activity.type)
  const time = formatActivityTimestamp(activity.timestamp)
  const size = formatFileSize(activity.file_size)
  
  return `${typeLabel} - ${activity.file_name || activity.file_path} - ${size} - ${time}`
}

/**
 * 格式化活动时间戳
 * 
 * @param {string} timestamp - ISO 8601 格式的时间戳
 * @returns {string} 格式化后的时间字符串
 */
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

/**
 * 获取活动类型标签
 * 
 * @param {string} type - 活动类型
 * @returns {string} 类型标签
 */
export function getTypeLabel(type) {
  const labels = {
    'upload': '上传',
    'download': '下载',
    'access': '访问'
  }
  return labels[type] || type
}

/**
 * 获取活动类型颜色
 * 
 * @param {string} type - 活动类型
 * @returns {string} 颜色代码
 */
export function getTypeColor(type) {
  const colors = {
    'upload': '#3fb850',
    'download': '#3178c6',
    'access': '#bc8cff'
  }
  return colors[type] || '#6e7681'
}

/**
 * 格式化文件大小
 * 
 * @param {number} bytes - 文件大小（字节）
 * @returns {string} 格式化后的大小字符串
 */
export function formatFileSize(bytes) {
  if (bytes === 0) return '0 B'
  
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
}
