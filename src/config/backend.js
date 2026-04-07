/**
 * 后端配置管理
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

import { ref } from 'vue'

// 后端配置
const backendConfig = ref({
  base_url: '',
  port: 0,
  full_url: ''
})

// 配置是否已加载
const isConfigLoaded = ref(false)

// 获取后端配置（只会在应用启动时调用一次）
export async function initBackendConfig() {
  if (isConfigLoaded.value) {
    console.log('后端配置已加载，跳过重复请求')
    return
  }

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const config = await invoke('get_backend_config')
    
    // 检查是否启用了 HTTPS
    let useHttps = false
    try {
      const { loadAppData } = await import('../components/data/storage.js')
      const saved = await loadAppData('use_https')
      if (saved) {
        const data = JSON.parse(saved)
        useHttps = data.useHttps || false
      }
    } catch (e) {
      console.warn('加载 HTTPS 设置失败:', e)
    }
    
    // 如果启用了 HTTPS，将 http://替换为 https://
    let baseUrl = config.base_url
    if (useHttps && baseUrl.startsWith('http://')) {
      baseUrl = baseUrl.replace('http://', 'https://')
      console.log('已启用 HTTPS 连接')
    }
    
    backendConfig.value = {
      base_url: baseUrl,
      port: config.port,
      full_url: `${baseUrl}:${config.port}`
    }
    
    isConfigLoaded.value = true
    
    // 在 console 输出配置信息
    console.log('='.repeat(50))
    console.log('后端配置信息')
    console.log('='.repeat(50))
    console.log('Base URL:', backendConfig.value.base_url)
    console.log('Port:', backendConfig.value.port)
    console.log('Full URL:', backendConfig.value.full_url)
    console.log('HTTPS:', useHttps ? '已启用' : '未启用')
    console.log('='.repeat(50))
  } catch (error) {
    console.error('加载后端配置失败:', error)
    // 使用默认配置
    backendConfig.value = {
      base_url: 'http://localhost',
      port: 8005,
      full_url: 'http://localhost:8005'
    }
    isConfigLoaded.value = true
    console.log('使用默认后端配置:', backendConfig.value.full_url)
  }
}

// 获取后端完整 URL
export function getBackendUrl() {
  return backendConfig.value.full_url
}

// 获取后端配置对象
export function getBackendConfig() {
  return backendConfig.value
}
