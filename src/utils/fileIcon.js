// 文件类型图标映射
// 根据文件扩展名返回对应的图标类名

// 文件类型分类
const FILE_TYPES = {
  IMAGE: 'image',
  VIDEO: 'video',
  AUDIO: 'audio',
  DOCUMENT: 'document',
  ARCHIVE: 'archive',
  CODE: 'code',
  OTHER: 'other'
}

// 文件扩展名到类型的映射
const EXTENSION_MAP = {
  // 图片
  'jpg': FILE_TYPES.IMAGE,
  'jpeg': FILE_TYPES.IMAGE,
  'png': FILE_TYPES.IMAGE,
  'gif': FILE_TYPES.IMAGE,
  'bmp': FILE_TYPES.IMAGE,
  'webp': FILE_TYPES.IMAGE,
  'svg': FILE_TYPES.IMAGE,
  'ico': FILE_TYPES.IMAGE,
  
  // 视频
  'mp4': FILE_TYPES.VIDEO,
  'avi': FILE_TYPES.VIDEO,
  'mkv': FILE_TYPES.VIDEO,
  'mov': FILE_TYPES.VIDEO,
  'wmv': FILE_TYPES.VIDEO,
  'flv': FILE_TYPES.VIDEO,
  'webm': FILE_TYPES.VIDEO,
  'm4v': FILE_TYPES.VIDEO,
  
  // 音频
  'mp3': FILE_TYPES.AUDIO,
  'wav': FILE_TYPES.AUDIO,
  'flac': FILE_TYPES.AUDIO,
  'aac': FILE_TYPES.AUDIO,
  'ogg': FILE_TYPES.AUDIO,
  'm4a': FILE_TYPES.AUDIO,
  'wma': FILE_TYPES.AUDIO,
  
  // 文档
  'pdf': FILE_TYPES.DOCUMENT,
  'doc': FILE_TYPES.DOCUMENT,
  'docx': FILE_TYPES.DOCUMENT,
  'xls': FILE_TYPES.DOCUMENT,
  'xlsx': FILE_TYPES.DOCUMENT,
  'ppt': FILE_TYPES.DOCUMENT,
  'pptx': FILE_TYPES.DOCUMENT,
  'txt': FILE_TYPES.DOCUMENT,
  'rtf': FILE_TYPES.DOCUMENT,
  'odt': FILE_TYPES.DOCUMENT,
  'ods': FILE_TYPES.DOCUMENT,
  'odp': FILE_TYPES.DOCUMENT,
  
  // 压缩包
  'zip': FILE_TYPES.ARCHIVE,
  'rar': FILE_TYPES.ARCHIVE,
  '7z': FILE_TYPES.ARCHIVE,
  'tar': FILE_TYPES.ARCHIVE,
  'gz': FILE_TYPES.ARCHIVE,
  'bz2': FILE_TYPES.ARCHIVE,
  'xz': FILE_TYPES.ARCHIVE,
  
  // 代码
  'js': FILE_TYPES.CODE,
  'ts': FILE_TYPES.CODE,
  'html': FILE_TYPES.CODE,
  'css': FILE_TYPES.CODE,
  'json': FILE_TYPES.CODE,
  'xml': FILE_TYPES.CODE,
  'py': FILE_TYPES.CODE,
  'java': FILE_TYPES.CODE,
  'cpp': FILE_TYPES.CODE,
  'c': FILE_TYPES.CODE,
  'h': FILE_TYPES.CODE,
  'rs': FILE_TYPES.CODE,
  'go': FILE_TYPES.CODE,
  'php': FILE_TYPES.CODE,
  'rb': FILE_TYPES.CODE,
  'swift': FILE_TYPES.CODE,
  'kt': FILE_TYPES.CODE
}

// 文件类型到图标类名的映射（使用 Remix Icon）
const TYPE_ICON_MAP = {
  [FILE_TYPES.IMAGE]: 'ri-image-line',
  [FILE_TYPES.VIDEO]: 'ri-video-line',
  [FILE_TYPES.AUDIO]: 'ri-music-line',
  [FILE_TYPES.DOCUMENT]: 'ri-file-text-line',
  [FILE_TYPES.ARCHIVE]: 'ri-file-zip-line',
  [FILE_TYPES.CODE]: 'ri-code-line',
  [FILE_TYPES.OTHER]: 'ri-file-line'
}

// 从文件名中提取扩展名
function getExtension(filename) {
  if (!filename) return ''
  const parts = filename.split('.')
  if (parts.length < 2) return ''
  return parts[parts.length - 1].toLowerCase()
}

// 根据文件名获取文件类型
function getFileType(filename) {
  const ext = getExtension(filename)
  return EXTENSION_MAP[ext] || FILE_TYPES.OTHER
}

// 根据文件名获取图标类名
function getFileIcon(filename) {
  const fileType = getFileType(filename)
  return TYPE_ICON_MAP[fileType]
}

// 根据文件名获取文件类型名称（中文）
function getFileTypeName(filename) {
  const fileType = getFileType(filename)
  const typeNameMap = {
    [FILE_TYPES.IMAGE]: '图片',
    [FILE_TYPES.VIDEO]: '视频',
    [FILE_TYPES.AUDIO]: '音频',
    [FILE_TYPES.DOCUMENT]: '文档',
    [FILE_TYPES.ARCHIVE]: '压缩包',
    [FILE_TYPES.CODE]: '代码',
    [FILE_TYPES.OTHER]: '其他'
  }
  return typeNameMap[fileType]
}

export {
  FILE_TYPES,
  getFileType,
  getFileIcon,
  getFileTypeName,
  getExtension
}
