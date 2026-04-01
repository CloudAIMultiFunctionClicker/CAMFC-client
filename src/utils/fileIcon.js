// 文件图标工具 - 根据文件扩展名返回对应图标

const EXT_TO_TYPE = {
  jpg: 'image', jpeg: 'image', png: 'image', gif: 'image',
  bmp: 'image', webp: 'image', svg: 'image', ico: 'image',
  mp4: 'video', avi: 'video', mkv: 'video', mov: 'video',
  wmv: 'video', flv: 'video', webm: 'video', m4v: 'video',
  mp3: 'audio', wav: 'audio', flac: 'audio', aac: 'audio',
  ogg: 'audio', m4a: 'audio', wma: 'audio',
  pdf: 'document', doc: 'document', docx: 'document',
  xls: 'document', xlsx: 'document', ppt: 'document', pptx: 'document',
  txt: 'document', rtf: 'document', odt: 'document',
  ods: 'document', odp: 'document',
  zip: 'archive', rar: 'archive',
  tar: 'archive', gz: 'archive', bz2: 'archive', xz: 'archive',
  '7z': 'archive',
  js: 'code', ts: 'code', html: 'code', css: 'code',
  json: 'code', xml: 'code', py: 'code', java: 'code',
  cpp: 'code', c: 'code', h: 'code', rs: 'code',
  go: 'code', php: 'code', rb: 'code', swift: 'code', kt: 'code'
}

const TYPE_TO_ICON = {
  image: 'ri-image-line',
  video: 'ri-video-line',
  audio: 'ri-music-line',
  document: 'ri-file-text-line',
  archive: 'ri-file-zip-line',
  code: 'ri-code-line',
  other: 'ri-file-line'
}

const TYPE_TO_NAME = {
  image: '图片',
  video: '视频',
  audio: '音频',
  document: '文档',
  archive: '压缩包',
  code: '代码',
  other: '其他'
}

function getExtension(filename) {
  if (!filename) return ''
  const parts = filename.split('.')
  if (parts.length < 2) return ''
  return parts[parts.length - 1].toLowerCase()
}

function getFileType(filename) {
  const ext = getExtension(filename)
  return EXT_TO_TYPE[ext] || 'other'
}

function getFileIcon(filename) {
  const fileType = getFileType(filename)
  return TYPE_TO_ICON[fileType]
}

function getFileTypeName(filename) {
  const fileType = getFileType(filename)
  return TYPE_TO_NAME[fileType]
}

export { getFileType, getFileIcon, getFileTypeName, getExtension }
