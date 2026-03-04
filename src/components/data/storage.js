import { invoke } from '@tauri-apps/api/core'
import { openPath } from '@tauri-apps/plugin-opener'

const NOTE_DIR = '.note'
const NOTE_FILE = 'notes.json'

async function waitForDownloadComplete(fileId, maxWaitMs = 30000) {
  const startTime = Date.now()
  
  while (Date.now() - startTime < maxWaitMs) {
    try {
      const progress = await invoke('get_download_progress', { fileId })
      console.log('下载进度:', progress)
      
      if (progress.status === 'Completed') {
        return true
      }
      if (progress.status && progress.status.startsWith('Error')) {
        console.error('下载失败:', progress.status)
        return false
      }
    } catch (e) {
      console.error('获取进度失败:', e)
    }
    
    await new Promise(resolve => setTimeout(resolve, 1000))
  }
  
  console.warn('下载等待超时')
  return false
}

async function waitForUploadComplete(uploadId, maxWaitMs = 30000) {
  const startTime = Date.now()
  
  while (Date.now() - startTime < maxWaitMs) {
    try {
      const progress = await invoke('get_upload_progress', { uploadId })
      console.log('上传进度:', progress)
      
      if (progress.status === 'Completed') {
        return true
      }
      if (progress.status && progress.status.startsWith('Error')) {
        console.error('上传失败:', progress.status)
        return false
      }
    } catch (e) {
      console.error('获取进度失败:', e)
    }
    
    await new Promise(resolve => setTimeout(resolve, 1000))
  }
  
  console.warn('上传等待超时')
  return false
}

export async function getNotes() {
  try {
    const fileId = `${NOTE_DIR}/${NOTE_FILE}`
    await invoke('download_file', { fileId })
    
    const downloaded = await waitForDownloadComplete(fileId)
    if (!downloaded) {
      console.warn('下载未完成，尝试直接读取')
    }
    
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    const content = await invoke('read_notes_content')
    console.log('笔记内容:', content ? '有内容' : '空内容', content.substring(0, 50))
    return content || ''
  } catch (error) {
    console.error('下载笔记失败:', error)
    return ''
  }
}

export async function setNotes(notes) {
  try {
    await invoke('write_notes_content', { content: notes })
    
    const tempPath = await invoke('get_notes_temp_path')
    console.log('笔记保存到本地:', tempPath)
    
    const result = await invoke('upload_files_from_paths', {
      filePaths: [tempPath],
      targetPath: NOTE_DIR
    })
    
    console.log('上传结果:', result)
    
    if (result.upload_ids && result.upload_ids.length > 0) {
      await waitForUploadComplete(result.upload_ids[0])
    }
    
    console.log('笔记保存成功')
    return true
  } catch (error) {
    console.error('保存笔记到云盘失败:', error)
    return false
  }
}

export async function openFile(filePath) {
  try {
    await openPath(filePath)
    return true
  } catch (error) {
    console.error('打开文件失败:', error)
    return false
  }
}

export async function openFolder(filePath) {
  try {
    const folderPath = filePath.substring(0, filePath.lastIndexOf('\\')) || filePath.substring(0, filePath.lastIndexOf('/'))
    await openPath(folderPath)
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

// Note缓存相关函数
export async function getCachedNotes() {
  try {
    const cachedData = await loadAppData('cached_notes')
    if (cachedData) {
      return JSON.parse(cachedData)
    }
  } catch (error) {
    console.log('读取缓存笔记失败:', error)
  }
  return null
}

export async function setCachedNotes(notes) {
  try {
    // 只缓存前9个笔记
    const notesToCache = notes.slice(0, 9)
    await saveAppData('cached_notes', JSON.stringify(notesToCache))
    return true
  } catch (error) {
    console.log('保存缓存笔记失败:', error)
    return false
  }
}

export async function clearCachedNotes() {
  try {
    await saveAppData('cached_notes', '')
    return true
  } catch (error) {
    console.log('清理缓存笔记失败:', error)
    return false
  }
}
