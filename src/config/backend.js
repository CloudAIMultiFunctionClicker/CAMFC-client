

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

  backendConfig.value = {
    base_url: 'https://camfc.seven-cloud.cn',
    port: 8005,
    full_url: 'https://camfc.seven-cloud.cn:8005'
  }
  isConfigLoaded.value = true
  console.log('后端配置已加载:', backendConfig.value.full_url)

}

export function getBackendUrl() {
  return backendConfig.value.full_url
}

export function getBackendConfig() {
  return backendConfig.value
}
