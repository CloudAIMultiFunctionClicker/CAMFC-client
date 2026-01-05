/*
 * Copyright (C) 2026 Jiale Xu (ANTmmmmm) (ant-cave)
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

// 文件相关的 mock API
// 注意：这里的所有函数都是假的，等后端上线后，这里换成真实 invoke

export interface FileItem {
  id: string
  name: string
  size: number // bytes
  type: 'file' | 'folder'
  extension?: string
  modifiedAt: string
  isStarred: boolean
  path: string
}

export interface FolderItem extends FileItem {
  children?: FileItem[]
}

// 模拟一些文件数据，假装这是一个云盘
const mockFiles: FileItem[] = [
  { id: '1', name: '项目文档', type: 'folder', size: 4096, modifiedAt: '2026-01-02 10:30:00', isStarred: true, path: '/项目文档' },
  { id: '2', name: '年度报告.pdf', type: 'file', extension: 'pdf', size: 2048000, modifiedAt: '2026-01-01 14:20:00', isStarred: true, path: '/年度报告.pdf' },
  { id: '3', name: '设计稿.sketch', type: 'file', extension: 'sketch', size: 5120000, modifiedAt: '2025-12-30 09:15:00', isStarred: false, path: '/设计稿.sketch' },
  { id: '4', name: '会议记录', type: 'folder', size: 81000092, modifiedAt: '2025-12-28 16:45:00', isStarred: false, path: '/会议记录' },
  { id: '5', name: '产品需求.docx', type: 'file', extension: 'docx', size: 307200, modifiedAt: '2025-12-25 11:00:00', isStarred: true, path: '/产品需求.docx' },
  { id: '6', name: '头像图片.png', type: 'file', extension: 'png', size: 102400, modifiedAt: '2025-12-20 13:30:00', isStarred: false, path: '/头像图片.png' },
  { id: '7', name: '视频素材', type: 'folder', size: 10485760, modifiedAt: '2025-12-15 18:20:00', isStarred: false, path: '/视频素材' },
  { id: '8', name: '配置文件.json', type: 'file', extension: 'json', size: 5120, modifiedAt: '2025-12-10 08:45:00', isStarred: false, path: '/配置文件.json' },
]

// 模拟文件夹树结构 
const mockFolderTree: FolderItem[] = [
  {
    id: 'root',
    name: '全部文件',
    type: 'folder',
    size: 0,
    modifiedAt: '',
    isStarred: false,
    path: '/',
    children: [
      { id: 'recent', name: '最近', type: 'folder', size: 0, modifiedAt: '', isStarred: false, path: '/最近' },
      { id: 'starred', name: '收藏', type: 'folder', size: 0, modifiedAt: '', isStarred: false, path: '/收藏' },
      { id: 'shared', name: '共享', type: 'folder', size: 0, modifiedAt: '', isStarred: false, path: '/共享' },
      { id: 'trash', name: '回收站', type: 'folder', size: 0, modifiedAt: '', isStarred: false, path: '/回收站' },
    ]
  }
]

// 模拟 API 调用延迟，让界面有点真实感（但别太久，用户会不耐烦）
function mockDelay(ms = 300): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

// 获取文件列表
export async function listFiles(path = '/'): Promise<FileItem[]> {
  await mockDelay(200 + Math.random() * 300)
  
  // 如果是根目录，返回所有文件
  if (path === '/') {
    return [...mockFiles]
  }
  
  // 简单模拟子目录
  return mockFiles.filter(file => file.path.startsWith(path))
}

// 获取文件夹树
export async function getFolderTree(): Promise<FolderItem[]> {
  await mockDelay(300)
  return [...mockFolderTree]
}

// 上传文件（mock 版本）
export async function uploadFile(file: File, targetPath = '/'): Promise<FileItem> {
  await mockDelay(500) // 假装上传需要时间
  
  const newFile: FileItem = {
    id: `upload_${Date.now()}`,
    name: file.name,
    type: 'file',
    size: file.size,
    extension: file.name.split('.').pop() || undefined,
    modifiedAt: new Date().toISOString().replace('T', ' ').substring(0, 19),
    isStarred: false,
    path: `${targetPath}/${file.name}`
  }
  
  // 这里理论上应该把文件添加到 mockFiles，但为了简单就不改了
  // mockFiles.push(newFile)
  
  return newFile
}

// 删除文件（移到回收站）
export async function deleteFile(fileId: string): Promise<boolean> {
  await mockDelay(250)
  console.log(`假装删除了文件 ${fileId}，实际上它只是被标记为删除`)
  return true
}

// 彻底删除（从回收站清除）
export async function purgeFile(fileId: string): Promise<boolean> {
  await mockDelay(350)
  console.log(`文件 ${fileId} 被永久删除了，再也找不回来了`)
  return true
}

// 恢复文件（从回收站救回来）
export async function rescueFileFromTrash(fileId: string): Promise<boolean> {
  await mockDelay(280)
  console.log(`把文件 ${fileId} 从回收站捞出来了，好险`)
  return true
}

// 重命名文件
export async function renameFile(fileId: string, newName: string): Promise<boolean> {
  await mockDelay(320)
  console.log(`把 ${fileId} 改名为 ${newName}，希望用户别后悔`)
  return true
}

// 收藏/取消收藏
export async function toggleStar(fileId: string): Promise<boolean> {
  await mockDelay(180)
  console.log(`切换了文件 ${fileId} 的收藏状态，用户的心意变得真快`)
  return true
}

// 创建文件夹
export async function createFolder(folderName: string, parentPath = '/'): Promise<FileItem> {
  await mockDelay(400)
  
  console.log(`创建文件夹: "${folderName}"，父路径: ${parentPath}`)
  
  const newFolder: FileItem = {
    id: `folder_${Date.now()}`,
    name: folderName,
    type: 'folder',
    size: 4096, // 默认文件夹大小
    modifiedAt: new Date().toISOString().replace('T', ' ').substring(0, 19),
    isStarred: false,
    path: parentPath === '/' ? `/${folderName}` : `${parentPath}/${folderName}`
  }
  
  // 在实际项目中，这里应该将新文件夹添加到mockFiles数组
  // mockFiles.push(newFolder)
  
  return newFolder
}
