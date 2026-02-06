/*<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->*/

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
    
    backendConfig.value = {
      base_url: config.base_url,
      port: config.port,
      full_url: config.full_url
    }
    
    isConfigLoaded.value = true
    
    // 在 console 输出配置信息
    console.log('='.repeat(50))
    console.log('🎯 后端配置信息')
    console.log('='.repeat(50))
    console.log('Base URL:', backendConfig.value.base_url)
    console.log('Port:', backendConfig.value.port)
    console.log('Full URL:', backendConfig.value.full_url)
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
