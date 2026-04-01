/**
 * 后端配置管理
 * 应用启动时加载一次，失败就用默认配置
 */

import { ref } from 'vue'

const backendConfig = ref({
  base_url: '',
  port: 0,
  full_url: ''
})

const isConfigLoaded = ref(false)

// 初始化配置（只调用一次）
export async function initBackendConfig() {
  if (isConfigLoaded.value) {
    return
  }

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const config = await invoke('get_backend_config')
    
    backendConfig.value = {
      base_url: config.base_url,
      port: config.port,
      full_url: config.full_url
    }
    
    console.log('后端配置:', backendConfig.value.full_url)
  } catch (error) {
    console.error('加载后端配置失败:', error)
    // 用默认配置
    backendConfig.value = {
      base_url: 'http://localhost',
      port: 8005,
      full_url: 'http://localhost:8005'
    }
  }
  
  isConfigLoaded.value = true
}

// 获取后端 URL
export function getBackendUrl() {
  return backendConfig.value.full_url
}

// 获取配置对象
export function getBackendConfig() {
  return backendConfig.value
}
