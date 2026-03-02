import { invoke } from '@tauri-apps/api/core'

export async function loadAppData(key) {
  try {
    const value = await invoke('load_app_data', { key })
    return value
  } catch (error) {
    console.error(`加载数据失败 (${key}):`, error)
    return ''
  }
}

export async function saveAppData(key, value) {
  try {
    await invoke('save_app_data', { key, value })
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

export async function getNotes() {
  return loadAppData('notes')
}

export async function setNotes(notes) {
  return saveAppData('notes', notes)
}

export async function getTheme() {
  return loadAppData('theme')
}

export async function setTheme(theme) {
  return saveAppData('theme', theme)
}
