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
  base_url: 'https://camfc.011420.xyz',
  port: 8005,
  full_url: 'https://camfc.011420.xyz'
})

// 配置是否已加载
const isConfigLoaded = ref(false)

// 获取后端配置（只会在应用启动时调用一次）
export async function initBackendConfig() {
  if (isConfigLoaded.value) {
    console.log('后端配置已加载，跳过重复请求')
    return
  }

  // 使用硬编码配置
  backendConfig.value = {
    base_url: 'https://camfc.011420.xyz',
    port: 8005,
    full_url: 'https://camfc.011420.xyz'
  }
  isConfigLoaded.value = true
  console.log('后端配置已加载:', backendConfig.value.full_url)
  
  // 以下为原配置加载逻辑（已禁用）
  /*
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const config = await invoke('get_backend_config')
    
    backendConfig.value = {
      base_url: config.base_url,
      port: config.port,
      full_url: `${config.base_url}:${config.port}`
    }
    
    isConfigLoaded.value = true
    
    console.log('后端配置已加载:', backendConfig.value.full_url)
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
  */
}

// 获取后端完整 URL
export function getBackendUrl() {
  return backendConfig.value.full_url
}

// 获取后端配置对象
export function getBackendConfig() {
  return backendConfig.value
}
