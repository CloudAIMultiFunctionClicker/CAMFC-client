

import { ref } from 'vue'

const backendConfig = ref({
  base_url: 'https://camfc.seven-cloud.cn',
  port: 8005,
  full_url: 'https://camfc.seven-cloud.cn:8005'
})

const isConfigLoaded = ref(false)

export async function initBackendConfig() {
  if (isConfigLoaded.value) {
    console.log('后端配置已加载，跳过重复请求')
    return
  }

  try {
    // 方式 1: 尝试从 Tauri 后端获取配置（运行时环境变量）
    const { invoke } = await import('@tauri-apps/api/core')
    const config = await invoke('get_backend_config')
    
    backendConfig.value = {
      base_url: config.base_url,
      port: config.port,
      full_url: config.full_url
    }
    console.log('从 Rust 后端获取配置:', backendConfig.value.full_url)
    isConfigLoaded.value = true
    return
  } catch (error) {
    console.warn('从后端获取配置失败，尝试使用环境变量:', error)
  }

  // 方式 2: 使用前端环境变量（编译时）
  const envBase = import.meta.env.VITE_CAMFC_BASE
  const envPort = import.meta.env.VITE_CAMFC_PORT

  if (envBase) {
    let base_url = envBase
    if (!base_url.startsWith('http://') && !base_url.startsWith('https://')) {
      base_url = `http://${base_url}`
    }
    
    const port = envPort ? parseInt(envPort) : 8005
    
    backendConfig.value = {
      base_url,
      port,
      full_url: `${base_url}:${port}`
    }
    console.log('使用环境变量配置:', backendConfig.value.full_url)
    isConfigLoaded.value = true
    return
  }

  // 方式 3: 使用默认配置
  backendConfig.value = {
    base_url: 'https://camfc.seven-cloud.cn',
    port: 8005,
    full_url: 'https://camfc.seven-cloud.cn:8005'
  }
  console.log('使用默认配置:', backendConfig.value.full_url)
  isConfigLoaded.value = true
}

export function getBackendUrl() {
  return backendConfig.value.full_url
}

export function getBackendConfig() {
  return backendConfig.value
}
