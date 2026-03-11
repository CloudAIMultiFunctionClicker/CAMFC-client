/**
 * CAMFC Client - 存储管理模块
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
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { invoke } from '@tauri-apps/api/core'

export async function openFile(filePath) {
  try {
    console.log('打开文件:', filePath)
    await invoke('open_file', { filePath })
    console.log('文件打开命令已发送')
    return true
  } catch (error) {
    console.error('打开文件失败:', error)
    return false
  }
}

export async function openFolder(filePath) {
  try {
    console.log('打开文件夹:', filePath)
    await invoke('open_folder', { folderPath: filePath })
    console.log('文件夹打开命令已发送')
    return true
  } catch (error) {
    console.error('打开文件夹失败:', error)
    return false
  }
}

let currentUserId = null

export async function setCurrentUserId(userId) {
  currentUserId = userId
}

export async function getCurrentUserId() {
  return currentUserId
}

function getUserKey(key) {
  if (!currentUserId) {
    console.warn('当前用户ID未设置，使用默认存储')
    return key
  }
  return `${currentUserId}:${key}`
}

export async function loadAppData(key) {
  try {
    const value = await invoke('load_app_data', { key: getUserKey(key) })
    return value
  } catch (error) {
    console.error(`加载数据失败 (${key}):`, error)
    return ''
  }
}

export async function saveAppData(key, value) {
  try {
    await invoke('save_app_data', { key: getUserKey(key), value })
    return true
  } catch (error) {
    console.error(`保存数据失败 (${key}):`, error)
    return false
  }
}

export async function getActiveUploads() {
  const value = await loadAppData('active_uploads')
  try {
    return JSON.parse(value || '[]')
  } catch {
    return []
  }
}

export async function setActiveUploads(uploadIds) {
  return saveAppData('active_uploads', JSON.stringify(uploadIds))
}

export async function getActiveDownloads() {
  const value = await loadAppData('active_downloads')
  try {
    return JSON.parse(value || '[]')
  } catch {
    return []
  }
}

export async function setActiveDownloads(downloadIds) {
  return saveAppData('active_downloads', JSON.stringify(downloadIds))
}

export async function getTheme() {
  return loadAppData('theme')
}

export async function setTheme(theme) {
  return saveAppData('theme', theme)
}

export async function getUploadHistory() {
  const value = await loadAppData('upload_history')
  try {
    return JSON.parse(value || '[]')
  } catch {
    return []
  }
}

export async function saveUploadHistory(history) {
  return saveAppData('upload_history', JSON.stringify(history))
}

export async function getDownloadHistory() {
  const value = await loadAppData('download_history')
  try {
    return JSON.parse(value || '[]')
  } catch {
    return []
  }
}

export async function saveDownloadHistory(history) {
  return saveAppData('download_history', JSON.stringify(history))
}
