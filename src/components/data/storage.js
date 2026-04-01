/**
 * CAMFC Client - 存储管理模块
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
 */

import { invoke } from '@tauri-apps/api/core'

// 当前用户 ID，用于数据隔离
let currentUserId = null

/**
 * 设置当前用户 ID
 * @param {string} userId - 用户 ID
 */
export async function setCurrentUserId(userId) {
    currentUserId = userId
}

/**
 * 获取当前用户 ID
 * @returns {string|null} 当前用户 ID
 */
export async function getCurrentUserId() {
    return currentUserId
}

/**
 * 生成用户隔离的存储键名
 * 格式：userId:key，未设置 userId 时直接使用原 key
 * @param {string} key - 原始键名
 * @returns {string} 带用户前缀的键名
 */
function getUserKey(key) {
    if (!currentUserId) {
        return key
    }
    return `${currentUserId}:${key}`
}

/**
 * 打开文件（调用系统默认应用）
 * @param {string} filePath - 文件路径
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function openFile(filePath) {
    try {
        await invoke('open_file', { filePath })
        return true
    } catch (error) {
        console.error('打开文件失败:', error)
        return false
    }
}

/**
 * 打开文件夹（调用系统默认应用）
 * @param {string} filePath - 文件夹路径
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function openFolder(filePath) {
    try {
        await invoke('open_folder', { folderPath: filePath })
        return true
    } catch (error) {
        console.error('打开文件夹失败:', error)
        return false
    }
}

/**
 * 加载应用数据（按用户隔离存储）
 * @param {string} key - 数据键名
 * @returns {Promise<string>} 数据值，失败返回空字符串
 */
export async function loadAppData(key) {
    try {
        return await invoke('load_app_data', { key: getUserKey(key) })
    } catch (error) {
        console.error(`加载数据失败 (${key}):`, error)
        return ''
    }
}

/**
 * 保存应用数据（按用户隔离存储）
 * @param {string} key - 数据键名
 * @param {string} value - 数据值
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function saveAppData(key, value) {
    try {
        await invoke('save_app_data', { key: getUserKey(key), value })
        return true
    } catch (error) {
        console.error(`保存数据失败 (${key}):`, error)
        return false
    }
}

/**
 * 获取活跃的上传任务列表
 * @returns {Promise<Array>} 上传任务 ID 数组
 */
export async function getActiveUploads() {
    const value = await loadAppData('active_uploads')
    try {
        return JSON.parse(value || '[]')
    } catch {
        return []
    }
}

/**
 * 设置活跃的上传任务列表
 * @param {Array} uploadIds - 上传任务 ID 数组
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function setActiveUploads(uploadIds) {
    return saveAppData('active_uploads', JSON.stringify(uploadIds))
}

/**
 * 获取活跃的下载任务列表
 * @returns {Promise<Array>} 下载任务 ID 数组
 */
export async function getActiveDownloads() {
    const value = await loadAppData('active_downloads')
    try {
        return JSON.parse(value || '[]')
    } catch {
        return []
    }
}

/**
 * 设置活跃的下载任务列表
 * @param {Array} downloadIds - 下载任务 ID 数组
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function setActiveDownloads(downloadIds) {
    return saveAppData('active_downloads', JSON.stringify(downloadIds))
}

/**
 * 获取主题设置
 * @returns {Promise<string>} 主题名称
 */
export async function getTheme() {
    return loadAppData('theme')
}

/**
 * 设置主题
 * @param {string} theme - 主题名称
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function setTheme(theme) {
    return saveAppData('theme', theme)
}

/**
 * 获取上传历史记录
 * @returns {Promise<Array>} 上传历史数组
 */
export async function getUploadHistory() {
    const value = await loadAppData('upload_history')
    try {
        return JSON.parse(value || '[]')
    } catch {
        return []
    }
}

/**
 * 保存上传历史记录
 * @param {Array} history - 上传历史数组
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function saveUploadHistory(history) {
    return saveAppData('upload_history', JSON.stringify(history))
}

/**
 * 获取下载历史记录
 * @returns {Promise<Array>} 下载历史数组
 */
export async function getDownloadHistory() {
    const value = await loadAppData('download_history')
    try {
        return JSON.parse(value || '[]')
    } catch {
        return []
    }
}

/**
 * 保存下载历史记录
 * @param {Array} history - 下载历史数组
 * @returns {Promise<boolean>} 成功返回 true
 */
export async function saveDownloadHistory(history) {
    return saveAppData('download_history', JSON.stringify(history))
}
