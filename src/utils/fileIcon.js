

const FILE_TYPES = {
  IMAGE: 'image',
  VIDEO: 'video',
  AUDIO: 'audio',
  DOCUMENT: 'document',
  ARCHIVE: 'archive',
  CODE: 'code',
  OTHER: 'other'
}

const EXTENSION_MAP = {

  'jpg': FILE_TYPES.IMAGE,
  'jpeg': FILE_TYPES.IMAGE,
  'png': FILE_TYPES.IMAGE,
  'gif': FILE_TYPES.IMAGE,
  'bmp': FILE_TYPES.IMAGE,
  'webp': FILE_TYPES.IMAGE,
  'svg': FILE_TYPES.IMAGE,
  'ico': FILE_TYPES.IMAGE,

  'mp4': FILE_TYPES.VIDEO,
  'avi': FILE_TYPES.VIDEO,
  'mkv': FILE_TYPES.VIDEO,
  'mov': FILE_TYPES.VIDEO,
  'wmv': FILE_TYPES.VIDEO,
  'flv': FILE_TYPES.VIDEO,
  'webm': FILE_TYPES.VIDEO,
  'm4v': FILE_TYPES.VIDEO,

  'mp3': FILE_TYPES.AUDIO,
  'wav': FILE_TYPES.AUDIO,
  'flac': FILE_TYPES.AUDIO,
  'aac': FILE_TYPES.AUDIO,
  'ogg': FILE_TYPES.AUDIO,
  'm4a': FILE_TYPES.AUDIO,
  'wma': FILE_TYPES.AUDIO,

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

  'zip': FILE_TYPES.ARCHIVE,
  'rar': FILE_TYPES.ARCHIVE,
  '7z': FILE_TYPES.ARCHIVE,
  'tar': FILE_TYPES.ARCHIVE,
  'gz': FILE_TYPES.ARCHIVE,
  'bz2': FILE_TYPES.ARCHIVE,
  'xz': FILE_TYPES.ARCHIVE,

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

const TYPE_ICON_MAP = {
  [FILE_TYPES.IMAGE]: 'ri-image-line',
  [FILE_TYPES.VIDEO]: 'ri-video-line',
  [FILE_TYPES.AUDIO]: 'ri-music-line',
  [FILE_TYPES.DOCUMENT]: 'ri-file-text-line',
  [FILE_TYPES.ARCHIVE]: 'ri-file-zip-line',
  [FILE_TYPES.CODE]: 'ri-code-line',
  [FILE_TYPES.OTHER]: 'ri-file-line'
}

function getExtension(filename) {
  if (!filename) return ''
  const parts = filename.split('.')
  if (parts.length < 2) return ''
  return parts[parts.length - 1].toLowerCase()
}

function getFileType(filename) {
  const ext = getExtension(filename)
  return EXTENSION_MAP[ext] || FILE_TYPES.OTHER
}

function getFileIcon(filename) {
  const fileType = getFileType(filename)
  return TYPE_ICON_MAP[fileType]
}

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
