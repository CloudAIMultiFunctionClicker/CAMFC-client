/**
 * CAMFC Client - 笔记云盘存储模块
 * 笔记数据持久化存储到云盘 .note 文件夹
 */

import { invoke } from '@tauri-apps/api/core'
import { ls, mkdir, rm } from './fileSystem.js'
import { uploadFilesFromPaths } from './upload.js'
import { showToast } from '../layout/showToast.js'

const NOTE_FOLDER = '.note'
const METADATA_FILE = 'metadata.json'

/**
 * 创建临时文件（调用 Rust 命令）
 * @param {string} content - 文件内容
 * @param {string} filename - 文件名
 * @returns {Promise<string>} 临时文件路径
 */
export async function createTempFile(content, filename) {
  try {
    console.info(`[NoteStorage] 创建临时文件: ${filename}`)
    const filePath = await invoke('create_temp_file', { content, filename })
    console.info(`[NoteStorage] 临时文件创建成功: ${filePath}`)
    return filePath
  } catch (error) {
    console.error(`[NoteStorage] 创建临时文件失败: ${error}`)
    throw new Error(`创建临时文件失败: ${error}`)
  }
}

/**
 * 检查并创建 .note 文件夹
 * @returns {Promise<boolean>} 是否成功
 */
export async function ensureNoteFolder() {
  try {
    console.info(`[NoteStorage] 检查 .note 文件夹是否存在...`)
    const result = await ls('')

    const noteFolder = result?.find(item => item.name === NOTE_FOLDER)

    if (!noteFolder) {
      console.info(`[NoteStorage] .note 文件夹不存在，准备创建...`)
      const createResult = await mkdir('', NOTE_FOLDER)
      console.info(`[NoteStorage] .note 文件夹创建结果:`, createResult)
      showToast('已创建笔记存储文件夹', '#10b981')
      return true
    }

    console.info(`[NoteStorage] .note 文件夹已存在`)
    return true
  } catch (error) {
    console.error(`[NoteStorage] 检查/创建 .note 文件夹失败: ${error}`)
    showToast('检查笔记文件夹失败', '#ef4444')
    return false
  }
}

/**
 * 加载笔记元数据列表
 * @returns {Promise<Array>} 笔记元数据数组
 */
export async function loadNoteMetadata() {
  try {
    console.info(`[NoteStorage] 加载笔记元数据...`)
    const result = await ls(NOTE_FOLDER)

    const metadataFile = result?.find(item => item.name === METADATA_FILE)

    if (!metadataFile) {
      console.info(`[NoteStorage] 元数据文件不存在，返回空列表`)
      return []
    }

    console.info(`[NoteStorage] 找到元数据文件，准备下载...`)
    const downloadResult = await invoke('download_file', { fileId: metadataFile.id })
    console.info(`[NoteStorage] 元数据文件下载结果: ${downloadResult}`)

    const filePath = downloadResult
    const response = await fetch(`file://${filePath}`)
    const text = await response.text()
    const metadata = JSON.parse(text)

    console.info(`[NoteStorage] 成功加载 ${metadata.length} 条笔记元数据`)
    return metadata
  } catch (error) {
    console.error(`[NoteStorage] 加载笔记元数据失败: ${error}`)
    return []
  }
}

/**
 * 保存笔记元数据到云盘
 * @param {Array} notes - 笔记数组
 * @returns {Promise<boolean>} 是否成功
 */
export async function saveNoteMetadata(notes) {
  try {
    console.info(`[NoteStorage] 保存笔记元数据，共 ${notes.length} 条`)

    const metadata = notes.map(note => ({
      id: note.id,
      title: note.title,
      createdAt: note.createdAt,
      updatedAt: note.updatedAt,
      tags: note.tags || [],
      isPinned: note.isPinned || false
    }))

    const content = JSON.stringify(metadata, null, 2)
    const tempPath = await createTempFile(content, METADATA_FILE)

    console.info(`[NoteStorage] 元数据临时文件路径: ${tempPath}`)

    const existResult = await ls(NOTE_FOLDER)
    const metadataFile = existResult?.find(item => item.name === METADATA_FILE)

    if (metadataFile) {
      console.info(`[NoteStorage] 删除旧的元数据文件...`)
      await rm(`${NOTE_FOLDER}/${METADATA_FILE}`, true)
    }

    console.info(`[NoteStorage] 上传新的元数据文件...`)
    const uploadResult = await uploadFilesFromPaths([tempPath], NOTE_FOLDER)
    console.info(`[NoteStorage] 元数据上传结果:`, uploadResult)

    if (!uploadResult.success) {
      throw new Error('元数据上传失败')
    }

    console.info(`[NoteStorage] 笔记元数据保存成功`)
    return true
  } catch (error) {
    console.error(`[NoteStorage] 保存笔记元数据失败: ${error}`)
    showToast('保存笔记失败', '#ef4444')
    return false
  }
}

/**
 * 加载单个笔记内容
 * @param {number} noteId - 笔记ID
 * @returns {Promise<string>} 笔记内容
 */
export async function loadNoteContent(noteId) {
  try {
    const contentFileName = `content_${noteId}.json`
    console.info(`[NoteStorage] 加载笔记内容: ${contentFileName}`)

    const result = await ls(NOTE_FOLDER)
    const contentFile = result?.find(item => item.name === contentFileName)

    if (!contentFile) {
      console.info(`[NoteStorage] 笔记内容文件不存在，返回空内容`)
      return ''
    }

    console.info(`[NoteStorage] 找到内容文件，准备下载...`)
    const downloadResult = await invoke('download_file', { fileId: contentFile.id })
    console.info(`[NoteStorage] 内容文件下载结果: ${downloadResult}`)

    const filePath = downloadResult
    const response = await fetch(`file://${filePath}`)
    const text = await response.text()
    const contentData = JSON.parse(text)

    console.info(`[NoteStorage] 成功加载笔记内容，长度: ${contentData.content?.length || 0}`)
    return contentData.content || ''
  } catch (error) {
    console.error(`[NoteStorage] 加载笔记内容失败: ${error}`)
    return ''
  }
}

/**
 * 保存单个笔记内容到云盘
 * @param {number} noteId - 笔记ID
 * @param {string} content - 笔记内容
 * @returns {Promise<boolean>} 是否成功
 */
export async function saveNoteContent(noteId, content) {
  try {
    const contentFileName = `content_${noteId}.json`
    console.info(`[NoteStorage] 保存笔记内容: ${contentFileName}`)

    const contentData = {
      id: noteId,
      content: content
    }

    const jsonContent = JSON.stringify(contentData, null, 2)
    const tempPath = await createTempFile(jsonContent, contentFileName)

    console.info(`[NoteStorage] 内容临时文件路径: ${tempPath}`)

    const existResult = await ls(NOTE_FOLDER)
    const contentFile = existResult?.find(item => item.name === contentFileName)

    if (contentFile) {
      console.info(`[NoteStorage] 删除旧的笔记内容文件...`)
      await rm(`${NOTE_FOLDER}/${contentFileName}`, true)
    }

    console.info(`[NoteStorage] 上传新的笔记内容文件...`)
    const uploadResult = await uploadFilesFromPaths([tempPath], NOTE_FOLDER)
    console.info(`[NoteStorage] 内容上传结果:`, uploadResult)

    if (!uploadResult.success) {
      throw new Error('内容上传失败')
    }

    console.info(`[NoteStorage] 笔记内容保存成功`)
    return true
  } catch (error) {
    console.error(`[NoteStorage] 保存笔记内容失败: ${error}`)
    showToast('保存笔记内容失败', '#ef4444')
    return false
  }
}

/**
 * 上传整个笔记（元数据 + 内容）
 * @param {Object} note - 笔记对象
 * @returns {Promise<boolean>} 是否成功
 */
export async function uploadNote(note) {
  try {
    console.info(`[NoteStorage] 上传笔记: ${note.title} (id: ${note.id})`)

    const success1 = await saveNoteMetadata([note])
    if (!success1) {
      throw new Error('元数据保存失败')
    }

    const success2 = await saveNoteContent(note.id, note.content || '')
    if (!success2) {
      throw new Error('内容保存失败')
    }

    console.info(`[NoteStorage] 笔记上传成功: ${note.title}`)
    return true
  } catch (error) {
    console.error(`[NoteStorage] 上传笔记失败: ${error}`)
    return false
  }
}

/**
 * 从云盘删除笔记
 * @param {number} noteId - 笔记ID
 * @returns {Promise<boolean>} 是否成功
 */
export async function deleteNoteFromCloud(noteId) {
  try {
    const contentFileName = `content_${noteId}.json`
    console.info(`[NoteStorage] 从云盘删除笔记: ${noteId}`)

    const existResult = await ls(NOTE_FOLDER)
    const contentFile = existResult?.find(item => item.name === contentFileName)

    if (contentFile) {
      console.info(`[NoteStorage] 删除笔记内容文件...`)
      await rm(`${NOTE_FOLDER}/${contentFileName}`, true)
    }

    console.info(`[NoteStorage] 笔记删除成功`)
    return true
  } catch (error) {
    console.error(`[NoteStorage] 删除笔记失败: ${error}`)
    return false
  }
}

/**
 * 加载所有笔记（包含内容和元数据）
 * @returns {Promise<Array>} 笔记数组
 */
export async function loadAllNotes() {
  try {
    console.info(`[NoteStorage] 开始加载所有笔记...`)

    const folderReady = await ensureNoteFolder()
    if (!folderReady) {
      console.warn(`[NoteStorage] .note 文件夹准备失败`)
      return []
    }

    const metadata = await loadNoteMetadata()

    if (metadata.length === 0) {
      console.info(`[NoteStorage] 没有笔记，返回默认笔记`)
      return [{
        id: Date.now(),
        title: 'Hello',
        content: '你好！这是你的第一个笔记。\n\n开始记录你的想法吧！',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        tags: [],
        isPinned: false
      }]
    }

    console.info(`[NoteStorage] 加载 ${metadata.length} 条笔记的内容...`)
    const notes = []

    for (const meta of metadata) {
      const content = await loadNoteContent(meta.id)
      notes.push({
        ...meta,
        content: content
      })
    }

    console.info(`[NoteStorage] 所有笔记加载完成，共 ${notes.length} 条`)
    return notes
  } catch (error) {
    console.error(`[NoteStorage] 加载所有笔记失败: ${error}`)
    showToast('加载笔记失败', '#ef4444')
    return []
  }
}

/**
 * 保存所有笔记
 * @param {Array} notes - 笔记数组
 * @returns {Promise<boolean>} 是否成功
 */
export async function saveAllNotes(notes) {
  try {
    console.info(`[NoteStorage] 开始保存所有笔记，共 ${notes.length} 条`)

    const metadataSuccess = await saveNoteMetadata(notes)
    if (!metadataSuccess) {
      throw new Error('元数据保存失败')
    }

    for (const note of notes) {
      const contentSuccess = await saveNoteContent(note.id, note.content || '')
      if (!contentSuccess) {
        console.warn(`[NoteStorage] 笔记 ${note.id} 内容保存失败`)
      }
    }

    console.info(`[NoteStorage] 所有笔记保存完成`)
    showToast('笔记已保存', '#10b981')
    return true
  } catch (error) {
    console.error(`[NoteStorage] 保存所有笔记失败: ${error}`)
    showToast('保存笔记失败', '#ef4444')
    return false
  }
}

export default {
  ensureNoteFolder,
  loadNoteMetadata,
  saveNoteMetadata,
  loadNoteContent,
  saveNoteContent,
  uploadNote,
  deleteNoteFromCloud,
  loadAllNotes,
  saveAllNotes,
  createTempFile
}
