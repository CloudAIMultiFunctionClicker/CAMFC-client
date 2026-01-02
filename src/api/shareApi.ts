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

// 分享相关的 mock API
// TODO: 等后端上线后，这里换成真实 invoke

export interface ShareLink {
  id: string
  fileId: string
  fileName: string
  url: string
  password?: string
  expiresAt?: string // ISO 格式
  createdAt: string
  accessCount: number
  canEdit: boolean
}

// 模拟延迟，让用户觉得网络请求是真实的（但别太慢，否则会以为断网了）
function mockDelay(ms = 300): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

// 创建分享链接
export async function createShareLink(params: {
  fileId: string
  fileName: string
  withPassword?: boolean
  password?: string
  expiresInDays?: number
  allowEdit?: boolean
}): Promise<ShareLink> {
  await mockDelay(450)
  
  const expiresAt = params.expiresInDays 
    ? new Date(Date.now() + params.expiresInDays * 24 * 60 * 60 * 1000).toISOString()
    : undefined
  
  const shareLink: ShareLink = {
    id: `share_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    fileId: params.fileId,
    fileName: params.fileName,
    url: `https://cloud.example.com/share/${Math.random().toString(36).substr(2, 12)}`,
    password: params.withPassword ? params.password || '123456' : undefined,
    expiresAt,
    createdAt: new Date().toISOString(),
    accessCount: 0,
    canEdit: params.allowEdit || false
  }
  
  console.log(`创建了分享链接：${shareLink.url}，有效期：${params.expiresInDays || '永久'}`)
  return shareLink
}

// 获取文件的分享链接列表
export async function getFileShareLinks(fileId: string): Promise<ShareLink[]> {
  await mockDelay(320)
  
  // 模拟返回一些分享链接
  const mockLinks: ShareLink[] = [
    {
      id: 'share_1',
      fileId,
      fileName: '年度报告.pdf',
      url: 'https://cloud.example.com/share/abc123',
      expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
      createdAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
      accessCount: 42,
      canEdit: false
    },
    {
      id: 'share_2',
      fileId,
      fileName: '年度报告.pdf',
      url: 'https://cloud.example.com/share/def456',
      password: '******',
      createdAt: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
      accessCount: 18,
      canEdit: true
    }
  ]
  
  return mockLinks.filter(link => link.fileId === fileId)
}

// 取消分享（删除分享链接）
export async function deleteShareLink(shareId: string): Promise<boolean> {
  await mockDelay(280)
  console.log(`取消了分享链接 ${shareId}，这下别人打不开了`)
  return true
}

// 更新分享设置（比如修改密码、有效期等）
export async function updateShareLink(shareId: string, updates: {
  password?: string | null
  expiresInDays?: number | null
  allowEdit?: boolean
}): Promise<boolean> {
  await mockDelay(350)
  console.log(`更新了分享链接 ${shareId} 的设置：`, updates)
  return true
}

// 获取热门分享（统计数据）
export async function getPopularShares(limit = 10): Promise<ShareLink[]> {
  await mockDelay(400)
  
  const mockPopular: ShareLink[] = [
    {
      id: 'popular_1',
      fileId: '2',
      fileName: '年度报告.pdf',
      url: 'https://cloud.example.com/share/pop1',
      createdAt: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000).toISOString(),
      accessCount: 156,
      canEdit: false
    },
    {
      id: 'popular_2',
      fileId: '5',
      fileName: '产品需求.docx',
      url: 'https://cloud.example.com/share/pop2',
      expiresAt: new Date(Date.now() + 3 * 24 * 60 * 60 * 1000).toISOString(),
      createdAt: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(),
      accessCount: 89,
      canEdit: true
    }
  ]
  
  return mockPopular.slice(0, limit)
}
