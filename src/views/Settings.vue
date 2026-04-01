<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<template>
  <div class="settings-page">
    <aside class="settings-sidebar">
      <h2 class="sidebar-title">设置</h2>
      <nav class="settings-nav">
        <button
          v-for="item in navItems"
          :key="item.id"
          class="nav-item"
          :class="{ active: activeNav === item.id }"
          @click="activeNav = item.id"
        >
          <i :class="item.icon"></i>
          <span>{{ item.label }}</span>
        </button>
      </nav>
    </aside>

    <main class="settings-content">
      <div v-if="activeNav === 'cpen'" class="settings-panel">
        <h3>Cpen 设置</h3>
        <div class="setting-item">
          <span>设备名称</span>
          <span class="setting-value">{{ deviceName || '未连接' }}</span>
        </div>
        <div class="setting-item">
          <span>设备 ID</span>
          <span class="setting-value">{{ deviceId || '未连接' }}</span>
        </div>
        <button class="action-btn danger" @click="disconnectDevice">断开设备</button>
      </div>

      <div v-else-if="activeNav === 'hardware'" class="settings-panel">
        <h3>连接设置</h3>
        <div class="setting-card">
          <div class="setting-item">
            <div class="setting-label">
              <div class="label-with-tooltip">
                <span class="label-text">心跳包</span>
                <div class="tooltip-wrapper">
                  <i class="ri-question-line"></i>
                  <span class="tooltip-text">设备之间会定期发送心跳包，以确定被连接设备的状态</span>
                </div>
              </div>
              <span class="label-desc">保持蓝牙连接的心跳包，过短可能影响电量</span>
            </div>
            <div class="setting-control">
              <input 
                type="number" 
                v-model.number="hardwareSettings.keepAliveInterval"
                class="number-input"
                min="1"
                max="300"
                @change="saveKeepAliveInterval"
              />
              <span class="unit">秒</span>
            </div>
          </div>
        </div>
        
        <div class="setting-card">
          <h4 class="card-title">蓝牙版本信息</h4>
          <div class="info-grid">
            <div class="info-item">
              <span class="info-label">Cpen 硬件蓝牙版本</span>
              <span class="info-value">{{ cpenBluetoothVersion }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">本地蓝牙版本</span>
              <span class="info-value">{{ localBluetoothVersion }}</span>
            </div>
          </div>
          <div class="refresh-tip">
            <i class="ri-refresh-line"></i>
            <span>连接设备后自动获取版本信息</span>
          </div>
        </div>
      </div>

      <div v-else-if="activeNav === 'download'" class="settings-panel">
        <h3>下载设置</h3>
        <div class="setting-card">
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">自定义下载路径</span>
              <span class="label-desc">文件将下载到指定目录，留空使用系统默认下载目录</span>
            </div>
            <div class="path-control">
              <input 
                type="text" 
                v-model="downloadPath" 
                class="path-input" 
                placeholder="点击右侧按钮选择目录"
                readonly
              />
              <button class="action-btn small" @click="selectDownloadPath">选择</button>
              <button 
                v-if="downloadPath" 
                class="action-btn small danger" 
                @click="clearDownloadPath"
              >
                清除
              </button>
            </div>
          </div>
          <div class="path-actions">
            <button class="action-btn secondary" @click="openDownloadFolder">
              <i class="ri-folder-open-line"></i>
              打开下载目录
            </button>
          </div>
        </div>
      </div>

      <div v-else-if="activeNav === 'application'" class="settings-panel">
        <h3>应用设置</h3>
        <div class="setting-card">
          <h4>窗口关闭行为</h4>
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">点击关闭按钮时</span>
              <span class="label-desc">选择点击窗口右上角关闭按钮时的行为</span>
            </div>
            <div class="close-behavior-options">
              <button 
                class="behavior-option" 
                :class="{ active: closeBehavior === 'minimize' }"
                @click="setCloseBehavior('minimize')"
              >
                <i class="ri-window-line"></i>
                <span>隐藏到托盘</span>
              </button>
              <button 
                class="behavior-option" 
                :class="{ active: closeBehavior === 'exit' }"
                @click="setCloseBehavior('exit')"
              >
                <i class="ri-close-circle-line"></i>
                <span>完全关闭</span>
              </button>
              <button 
                class="behavior-option" 
                :class="{ active: closeBehavior === 'ask' }"
                @click="setCloseBehavior('ask')"
              >
                <i class="ri-question-line"></i>
                <span>每次询问</span>
              </button>
            </div>
          </div>
        </div>
        
        <div class="setting-card">
          <h4>悬浮窗功能</h4>
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">启用悬浮窗</span>
              <span class="label-desc">显示可拖动的悬浮窗，提供快速访问功能</span>
            </div>
            <div class="setting-control">
              <button 
                class="toggle-btn" 
                :class="{ active: floatWindowEnabled }"
                @click="toggleFloatWindow"
              >
                <span class="toggle-slider"></span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div v-else-if="activeNav === 'theme'" class="settings-panel">
        <h3>深色模式</h3>
        <div class="setting-item">
          <span>启用深色模式</span>
          <button 
            class="toggle-btn" 
            :class="{ active: !theme?.isLightMode.value }" 
            @click="theme?.toggleTheme()"
          >
            <span class="toggle-slider"></span>
          </button>
        </div>
        <div class="setting-item">
          <span>跟随系统主题</span>
          <button 
            class="toggle-btn" 
            :class="{ active: storageSettings.followSystemTheme }"
            @click="toggleFollowSystemTheme"
          >
            <span class="toggle-slider"></span>
          </button>
        </div>
      </div>

      <div v-else-if="activeNav === 'help'" class="settings-panel help-panel">
        <h3>帮助与反馈</h3>
        <button class="action-btn" @click="openIssue">提交问题或反馈</button>
        <button class="action-btn" @click="showFaq = true">常见问题 (FAQ)</button>
      </div>

      <div v-else-if="activeNav === 'about'" class="settings-panel">
        <h3>关于</h3>
        <div class="about-info">
          <Cloud :size="64" class="app-icon" />
          <h4>CAMFC Cloud</h4>
          <p class="version">版本 1.0.0</p>
          <p class="desc">云端多功能点击器客户端</p>
        </div>
        <div class="setting-item">
          <span>检查更新</span>
          <span class="setting-value">已是最新</span>
        </div>
        <button class="action-btn" @click="openChangelog">查看更新日志</button>
      </div>
    </main>
  </div>
  
  <!-- FAQ iframe 弹窗 -->
  <Transition name="slide-right">
    <div v-if="showFaq" class="faq-overlay" @click="showFaq = false">
      <div class="faq-panel" :class="{ 'light-mode': isLightMode }" @click.stop>
        <div class="faq-header">
          <button class="close-btn" @click="showFaq = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
          </button>
          <button class="close-btn" @click="openFaqInBrowser" title="在浏览器中打开">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" x2="22" y1="12" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
          </button>
        </div>
        <div class="faq-body">
          <iframe 
            src="https://chatbot.weixin.qq.com/webapp/6h2QNMkoVgfSPjIqghfjRG17CxC9b7?isFloat=True&robotName=智能问答"
            frameborder="0"
            allowfullscreen
          ></iframe>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup>
import { inject, ref, onMounted, onUnmounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { showToast } from '../components/layout/showToast.js'
import { disconnect, getDeviceId } from '../components/data/bluetooth.js'
import { ls } from '../components/data/fileSystem.js'
import { loadAppData, saveAppData } from '../components/data/storage.js'
import { openUrl } from '@tauri-apps/plugin-opener'
import { open } from '@tauri-apps/plugin-dialog'
import { Cloud } from 'lucide-vue-next'

const theme = inject('theme')
const activeNav = ref('cpen')

const cpenSettings = ref({
  autoConnect: false,
  lastDeviceAddress: ''
})

const storageSettings = ref({
  autoCleanCache: false,
  followSystemTheme: false
})

const hardwareSettings = ref({
  keepAliveInterval: 30
})

const cpenBluetoothVersion = ref('未连接')
const localBluetoothVersion = ref('5.0')

// 保活定时器
let keepAliveTimer = null

const deviceId = ref(null)
const deviceName = ref(null)
const isFilesystemLoggedIn = ref(false)
const cacheSize = ref(0)
const downloadPath = ref('')
const closeBehavior = ref('ask') // 'minimize' | 'exit' | 'ask'
const showFaq = ref(false)
const isLightMode = ref(false)
const floatWindowEnabled = ref(true)

// 监听主题变化，同步 FAQ 悬浮窗
watch(() => theme?.isLightMode, (newVal) => {
  if (newVal !== undefined) {
    isLightMode.value = newVal
  }
}, { immediate: true })

const navItems = [
  { id: 'cpen', label: 'Cpen 设置', icon: 'ri-settings-3-line' },
  { id: 'hardware', label: '连接设置', icon: 'ri-link' },
  { id: 'download', label: '下载设置', icon: 'ri-download-line' },
  { id: 'application', label: '应用设置', icon: 'ri-apps-line' },
  { id: 'theme', label: '深色模式', icon: 'ri-moon-line' },
  { id: 'help', label: '帮助与反馈', icon: 'ri-question-line' },
  { id: 'about', label: '关于', icon: 'ri-information-line' }
]

const loadSettings = async () => {
  try {
    const savedCpen = await loadAppData('settings_cpen')
    if (savedCpen) {
      cpenSettings.value = JSON.parse(savedCpen)
    }

    const savedStorage = await loadAppData('settings_storage')
    if (savedStorage) {
      storageSettings.value = JSON.parse(savedStorage)
    }

    // 加载自定义下载路径
    const { invoke } = await import('@tauri-apps/api/core')
    try {
      const customPath = await invoke('get_custom_download_path')
      downloadPath.value = customPath || ''
    } catch (e) {
      console.warn('获取自定义下载路径失败:', e)
      downloadPath.value = ''
    }
    
    // 加载关闭行为偏好
    try {
      const savedCloseBehavior = await loadAppData('close_preference')
      if (savedCloseBehavior) {
        const pref = JSON.parse(savedCloseBehavior)
        closeBehavior.value = pref.preference || 'ask'
      }
    } catch (e) {
      console.warn('加载关闭偏好失败:', e)
      closeBehavior.value = 'ask'
    }
    
    // 加载悬浮窗启用状态
    try {
      const { getFloatWindowEnabled } = await import('../components/data/storage.js')
      floatWindowEnabled.value = await getFloatWindowEnabled()
    } catch (e) {
      console.warn('加载悬浮窗状态失败:', e)
      floatWindowEnabled.value = true
    }
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

const toggleAutoConnect = async () => {
  cpenSettings.value.autoConnect = !cpenSettings.value.autoConnect
  await saveAppData('settings_cpen', JSON.stringify(cpenSettings.value))
  const status = cpenSettings.value.autoConnect ? '已启用' : '已禁用'
  showToast(`自动连接 Cpen 设备：${status}`, '#3b82f6')
}

const toggleFollowSystemTheme = async () => {
  storageSettings.value.followSystemTheme = !storageSettings.value.followSystemTheme
  await saveAppData('settings_storage', JSON.stringify(storageSettings.value))
  
  if (storageSettings.value.followSystemTheme) {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    theme?.setTheme(!mediaQuery.matches)
    mediaQuery.addEventListener('change', handleSystemThemeChange)
  } else {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.removeEventListener('change', handleSystemThemeChange)
  }
}

const handleSystemThemeChange = (e) => {
  if (storageSettings.value.followSystemTheme) {
    theme?.setTheme(!e.matches)
  }
}

const toggleAutoCleanCache = async () => {
  storageSettings.value.autoCleanCache = !storageSettings.value.autoCleanCache
  await saveAppData('settings_storage', JSON.stringify(storageSettings.value))
  const status = storageSettings.value.autoCleanCache ? '已启用' : '已禁用'
  showToast(`自动清理缓存：${status}`, '#3b82f6')
}

const saveKeepAliveInterval = async () => {
  if (hardwareSettings.value.keepAliveInterval < 1) {
    hardwareSettings.value.keepAliveInterval = 1
  }
  if (hardwareSettings.value.keepAliveInterval > 300) {
    hardwareSettings.value.keepAliveInterval = 300
  }
  await saveAppData('hardware_settings', JSON.stringify(hardwareSettings.value))
  
  // 重启保活定时器
  stopKeepAliveTimer()
  if (hardwareSettings.value.keepAliveInterval > 0) {
    startKeepAliveTimer()
  }
}

const startKeepAliveTimer = () => {
  if (keepAliveTimer) {
    clearInterval(keepAliveTimer)
  }
  
  const interval = hardwareSettings.value.keepAliveInterval * 1000 // 转换为毫秒
  
  keepAliveTimer = setInterval(async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('send_keep_alive')
      console.log(`保活心跳包已发送（间隔：${hardwareSettings.value.keepAliveInterval}秒）`)
    } catch (e) {
      console.warn('发送保活心跳包失败:', e)
    }
  }, interval)
  
  console.log(`蓝牙保活定时器已启动，间隔：${hardwareSettings.value.keepAliveInterval}秒`)
}

const stopKeepAliveTimer = () => {
  if (keepAliveTimer) {
    clearInterval(keepAliveTimer)
    keepAliveTimer = null
    console.log('蓝牙保活定时器已停止')
  }
}

const loadHardwareSettings = async () => {
  try {
    const saved = await loadAppData('hardware_settings')
    if (saved) {
      hardwareSettings.value = JSON.parse(saved)
    }
  } catch (error) {
    console.error('加载硬件设置失败:', error)
  }
}

const fetchBluetoothVersions = async () => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    
    // 获取本地蓝牙版本
    try {
      const localVersion = await invoke('get_local_bluetooth_version')
      localBluetoothVersion.value = localVersion
    } catch (e) {
      console.warn('获取本地蓝牙版本失败:', e)
      localBluetoothVersion.value = '获取失败'
    }
    
    // 获取 Cpen 设备蓝牙版本
    try {
      const cpenVersion = await invoke('get_cpen_bluetooth_version')
      cpenBluetoothVersion.value = cpenVersion
    } catch (e) {
      console.warn('获取 Cpen 设备蓝牙版本失败:', e)
      cpenBluetoothVersion.value = '未连接'
    }
  } catch (e) {
    console.warn('导入 Tauri 模块失败:', e)
  }
}

const checkFilesystemLogin = async () => {
  try {
    let id = null
    let cloudAccessible = false
    let name = null
    
    try {
      id = await getDeviceId()
      deviceId.value = id
      
      // 获取设备名称
      const { invoke } = await import('@tauri-apps/api/core')
      const status = await invoke('get_connection_status')
      console.log('连接状态原始值:', status)
      // 从连接状态中提取设备名（去掉"已连接到设备："前缀）
      if (status && status.startsWith('已连接')) {
        // 找到第一个冒号（中文或英文）的位置
        const colonIndex = status.indexOf(':') !== -1 ? status.indexOf(':') : status.indexOf('：')
        if (colonIndex !== -1 && colonIndex < status.length - 1) {
          name = status.substring(colonIndex + 1).trim()
        } else {
          name = status
        }
      } else if (status && status !== '未连接') {
        name = status
      }
      console.log('提取的设备名:', name)
      deviceName.value = name
    } catch (idError) {
      console.warn('获取设备信息失败:', idError)
    }
    
    if (id) {
      try {
        const result = await ls('')
        cloudAccessible = result !== null
      } catch (lsError) {
        console.warn('访问云盘失败:', lsError)
        cloudAccessible = false
      }
    }
    
    isFilesystemLoggedIn.value = cloudAccessible || (id !== null)
  } catch (error) {
    console.warn('检查登录状态失败:', error)
    isFilesystemLoggedIn.value = false
    deviceId.value = null
    deviceName.value = null
  }
}

const disconnectDevice = async () => {
  showToast('正在断开设备...', '#f59e0b')
  await disconnect()
  deviceName.value = null
  deviceId.value = null
  showToast('已断开设备连接', '#10b981')
}

const toggleFloatWindow = async () => {
  floatWindowEnabled.value = !floatWindowEnabled.value
  try {
    const { setFloatWindowEnabled } = await import('../components/data/storage.js')
    await setFloatWindowEnabled(floatWindowEnabled.value)
    const status = floatWindowEnabled.value ? '已启用' : '已禁用'
    showToast(`悬浮窗功能：${status}`, '#3b82f6')
    console.log('[设置页面] 悬浮窗状态已更新:', floatWindowEnabled.value)
    
    // 广播悬浮窗状态变化事件
    try {
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
      console.log('[设置页面] 尝试获取悬浮窗...')
      const floatWindow = await WebviewWindow.getByLabel('float')
      console.log('[设置页面] 悬浮窗获取成功:', floatWindow)
      if (floatWindow) {
        console.log('[设置页面] 发送悬浮窗状态变化事件...')
        await floatWindow.emit('float-window-toggled', floatWindowEnabled.value)
        console.log('[设置页面] 事件发送成功')
        
        // 如果启用悬浮窗且窗口被隐藏，显示它
        if (floatWindowEnabled.value) {
          const isVisible = await floatWindow.isVisible()
          if (!isVisible) {
            console.log('[设置页面] 悬浮窗被隐藏，正在显示...')
            await floatWindow.show()
            await floatWindow.center()
          }
        }
      } else {
        console.log('[设置页面] 悬浮窗不存在')
        // 如果启用悬浮窗且窗口不存在，尝试创建
        if (floatWindowEnabled.value) {
          console.log('[设置页面] 悬浮窗不存在，尝试创建...')
          try {
            const newFloatWindow = new WebviewWindow('float', {
              url: '/float',
              width: 360,
              height: 40,
              x: 100,
              y: 100,
              alwaysOnTop: true,
              decorations: false,
              transparent: true,
              resizable: false,
              skipTaskbar: true
            })
            console.log('[设置页面] 悬浮窗创建成功')
          } catch (createError) {
            console.error('[设置页面] 悬浮窗创建失败:', createError)
          }
        }
      }
    } catch (e) {
      console.error('[设置页面] 广播悬浮窗状态失败:', e)
    }
  } catch (e) {
    console.error('切换悬浮窗状态失败:', e)
    showToast('保存设置失败', '#ef4444')
  }
}

const logout = async () => {
  showToast('正在退出登录...', '#f59e0b')
  await disconnect()
  showToast('已退出登录', '#10b981')
  setTimeout(() => {
    window.location.href = '/'
  }, 500)
}

const openFaqInBrowser = async () => {
  try {
    await openUrl('https://chatbot.weixin.qq.com/webapp/6h2QNMkoVgfSPjIqghfjRG17CxC9b7?isFloat=True&robotName=智能问答')
  } catch (error) {
    console.error('在浏览器中打开 FAQ 失败:', error)
    showToast('打开失败', '#ef4444')
  }
}

const clearCache = async () => {
  showToast('正在清理缓存...', '#f59e0b')
  cacheSize.value = 0
  showToast('缓存清理完成', '#10b981')
}

const getCacheSize = () => {
  cacheSize.value = 0
}

const openChangelog = () => {
  openUrl('https://github.com/CloudAIMultiFunctionClicker/CAMFC-client/releases/')
}

const openIssue = () => {
  openUrl('https://github.com/CloudAIMultiFunctionClicker/CAMFC-client/issues/')
}

const selectDownloadPath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择下载目录'
    })
    
    if (selected) {
      downloadPath.value = selected
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('set_custom_download_path', { path: selected })
      showToast('下载路径已设置为: ' + selected, '#10b981')
    }
  } catch (e) {
    console.error('选择下载目录失败:', e)
    showToast('选择下载目录失败', '#ef4444')
  }
}

const clearDownloadPath = async () => {
  try {
    downloadPath.value = ''
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('set_custom_download_path', { path: '' })
    showToast('已恢复使用系统默认下载目录', '#10b981')
  } catch (e) {
    console.error('清除下载路径失败:', e)
    showToast('清除下载路径失败', '#ef4444')
  }
}

const openDownloadFolder = async () => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    let targetPath = downloadPath.value
    
    if (!targetPath) {
      // 使用 Windows 的公共下载目录作为默认下载路径
      if (navigator.platform.indexOf('Win') > -1) {
        targetPath = 'C:\\Users\\Public\\Downloads'
      } else {
        showToast('无法确定下载目录', '#f59e0b')
        return
      }
    }
    
    if (targetPath) {
      await invoke('open_folder', { folderPath: targetPath })
      showToast('已打开下载目录', '#10b981')
    } else {
      showToast('无法确定下载目录', '#f59e0b')
    }
  } catch (e) {
    console.error('打开下载目录失败:', e)
    showToast('打开失败: ' + e.message, '#ef4444')
  }
}

const setCloseBehavior = async (behavior) => {
  try {
    closeBehavior.value = behavior
    await saveAppData('close_preference', JSON.stringify({ preference: behavior }))
    console.log('[设置页面] 保存关闭偏好:', behavior)
    const text = behavior === 'minimize' ? '隐藏到托盘' : behavior === 'exit' ? '完全关闭' : '每次询问'
    showToast(`关闭行为已设置为：${text}`, '#10b981')
  } catch (e) {
    console.error('[设置页面] 保存关闭偏好失败:', e)
    showToast('保存失败', '#ef4444')
  }
}

const clearCloseBehavior = async () => {
  try {
    closeBehavior.value = 'ask'
    await saveAppData('close_preference', JSON.stringify({ preference: 'ask' }))
    console.log('[设置页面] 清除关闭偏好')
    showToast('已清除关闭偏好，下次将重新询问', '#10b981')
  } catch (e) {
    console.error('[设置页面] 清除关闭偏好失败:', e)
    showToast('清除失败', '#ef4444')
  }
}

const formatSize = (bytes) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

onMounted(() => {
  loadSettings()
  loadHardwareSettings()
  fetchBluetoothVersions()
  checkFilesystemLogin()
  getCacheSize()
  
  // 启动保活定时器
  if (hardwareSettings.value.keepAliveInterval > 0) {
    startKeepAliveTimer()
  }
})

onUnmounted(() => {
  // 组件卸载时停止保活定时器
  stopKeepAliveTimer()
})
</script>

<style scoped>
.settings-page {
  display: flex;
  min-height: 100vh;
  background-color: var(--bg-primary, #ffffff);
}

.settings-sidebar {
  width: 260px;
  background-color: var(--bg-secondary, #ffffff);
  border-right: 1px solid var(--border-color, #d0d7de);
  padding: 24px 16px;
  flex-shrink: 0;
}

.sidebar-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 24px 8px;
  padding: 0 8px;
}

.settings-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 12px 16px;
  background: none;
  border: none;
  border-radius: .375rem;
  color: var(--text-secondary, #57606a);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
}

.nav-item:hover {
  background-color: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
}

.nav-item.active {
  background-color: var(--selected-bg, #ddf4ff);
  color: var(--accent-blue, #0969da);
  font-weight: 500;
}

.nav-item i {
  font-size: 18px;
  width: 20px;
  text-align: center;
}

.settings-content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
  background-color: var(--bg-primary, #ffffff);
}

.settings-panel {
  width: 100%;
  max-width: 800px;
}

.settings-panel h3 {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 24px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
}

.placeholder-text {
  color: var(--text-muted, #8c959f);
  font-size: 15px;
  line-height: 1.6;
}

.label-desc {
  font-size: 13px;
  color: var(--text-muted, #8c959f);
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.unit {
  font-size: 14px;
  color: var(--text-secondary, #57606a);
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  margin-bottom: 12px;
  color: var(--text-primary, #24292f);
  font-size: 15px;
}

.setting-card {
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  padding: 20px;
  margin-bottom: 16px;
}

.setting-card h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 16px 0;
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.label-text {
  font-size: 15px;
  font-weight: 500;
}

.label-with-tooltip {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.tooltip-wrapper {
  position: relative;
  display: inline-flex;
  align-items: center;
  cursor: help;
}

.tooltip-wrapper i {
  font-size: 16px;
  color: var(--text-muted, #64748b);
  transition: color 0.2s;
}

.tooltip-wrapper:hover i {
  color: var(--accent-blue, #3b82f6);
}

.tooltip-text {
  position: absolute;
  left: 0;
  bottom: 100%;
  background-color: var(--bg-primary, #ffffff);
  color: var(--text-primary, #24292f);
  font-size: 12px;
  padding: 8px 12px;
  border-radius: .375rem;
  border: 1px solid var(--border-color, #d0d7de);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  white-space: nowrap;
  opacity: 0;
  visibility: hidden;
  transition: all 0.2s ease;
  z-index: 1000;
  pointer-events: none;
}

.tooltip-text::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 20px;
  transform: translateX(-50%);
  border: 6px solid transparent;
  border-top-color: var(--bg-primary, #ffffff);
}

.tooltip-wrapper:hover .tooltip-text {
  opacity: 1;
  visibility: visible;
}

.label-desc {
  font-size: 13px;
  color: var(--text-muted, #64748b);
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.number-input {
  width: 80px;
  padding: 8px 12px;
  background-color: var(--input-bg, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  color: var(--text-primary, #24292f);
  font-size: 14px;
  text-align: center;
}

.number-input:focus {
  outline: none;
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.path-input {
  flex: 1;
  padding: 10px 14px;
  background-color: var(--input-bg, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  color: var(--text-primary, #24292f);
  font-size: 14px;
}

.path-input:focus {
  outline: none;
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.path-input::placeholder {
  color: var(--text-muted, #8c959f);
}

.unit {
  font-size: 14px;
  color: var(--text-secondary, #94a3b8);
}

.card-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f1f5f9);
  margin: 0 0 16px 0;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
  margin-bottom: 12px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 12px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border-radius: .375rem;
  border: 1px solid var(--border-color, #d0d7de);
}

.info-label {
  font-size: 12px;
  color: var(--text-muted, #8c959f);
  font-weight: 500;
}

.info-value {
  font-size: 15px;
  color: var(--text-primary, #24292f);
  font-weight: 600;
}

.refresh-tip {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 12px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border-radius: .375rem;
  border: 1px solid var(--border-color, #d0d7de);
  font-size: 12px;
  color: var(--text-secondary, #57606a);
}

.refresh-tip i {
  font-size: 14px;
  color: var(--accent-blue, #0969da);
}

.toggle-btn {
  position: relative;
  width: 48px;
  height: 26px;
  background-color: var(--border-color, #d0d7de);
  border: none;
  border-radius: 13px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.toggle-btn.active {
  background-color: var(--accent-blue, #0969da);
}

.toggle-slider {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 20px;
  height: 20px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.3s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.toggle-btn.active .toggle-slider {
  transform: translateX(22px);
}

.setting-value {
  color: var(--text-muted, #8c959f);
  font-size: 14px;
}

.behavior-option {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--text-primary, #24292f);
  font-size: 14px;
}

.behavior-option i {
  font-size: 24px;
  color: var(--text-muted, #8c959f);
  transition: color 0.2s ease;
}

.behavior-option:hover {
  border-color: var(--accent-blue, #0969da);
  background-color: var(--selected-bg, #ddf4ff);
}

.behavior-option:hover i {
  color: var(--accent-blue, #0969da);
}

.behavior-option.active {
  border-color: var(--accent-blue, #0969da);
  background-color: var(--selected-bg, #ddf4ff);
}

.behavior-option.active i {
  color: var(--accent-blue, #0969da);
}

.action-btn {
  margin-top: 16px;
  padding: 10px 20px;
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
}

.action-btn.secondary {
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
}

.action-btn.danger {
  background-color: var(--danger-btn-bg, #212830);
  color: var(--danger-btn-text, #f85149);
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
}

.action-btn.danger:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, white);
  border-color: var(--danger-btn-hover-border, #f85149);
}

/* 危险按钮图标 - 继承按钮颜色 */
.action-btn.danger i,
.action-btn.danger svg {
  color: inherit;
}

.action-btn.small {
  margin-top: 0;
  padding: 6px 14px;
  font-size: 13px;
  border-radius: .375rem;
}

.path-control {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 12px;
}

.path-input {
  flex: 1;
  padding: 10px 14px;
  background-color: var(--bg-primary, #0f172a);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: .375rem;
  color: var(--text-primary, #f1f5f9);
  font-size: 14px;
}

.path-input:focus {
  outline: none;
  border-color: var(--accent-blue, #3b82f6);
}

.path-input::placeholder {
  color: var(--text-muted, #64748b);
}

.path-actions {
  margin-top: 12px;
  display: flex;
  gap: 8px;
}

.setting-card h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f1f5f9);
  margin: 0 0 16px 0;
}

.close-behavior-options {
  display: flex;
  gap: 12px;
  margin-top: 12px;
}

.behavior-option {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background-color: var(--bg-primary, #0f172a);
  border: 2px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--text-primary, #f1f5f9);
  font-size: 14px;
}

.behavior-option i {
  font-size: 24px;
  color: var(--text-muted, #64748b);
  transition: color 0.2s ease;
}

.behavior-option:hover {
  border-color: var(--accent-blue, #3b82f6);
  background-color: var(--hover-bg, rgba(59, 130, 246, 0.1));
}

.behavior-option:hover i {
  color: var(--accent-blue, #3b82f6);
}

.behavior-option.active {
  border-color: var(--accent-blue, #3b82f6);
  background-color: rgba(59, 130, 246, 0.2);
}

.behavior-option.active i {
  color: var(--accent-blue, #3b82f6);
}

.setting-hint {
  margin: 8px 0 0 0;
  font-size: 13px;
  color: var(--text-muted, #64748b);
}



.storage-info {
  margin-bottom: 20px;
}

.storage-bar {
  height: 8px;
  background-color: var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: .375rem;
  overflow: hidden;
  margin-bottom: 8px;
}

.storage-used {
  height: 100%;
  background: var(--accent-blue, #3178c6);
  border-radius: .375rem;
}

.storage-text {
  color: var(--text-muted, #64748b);
  font-size: 13px;
  margin: 0;
}

.about-info {
  text-align: center;
  padding: 32px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  margin-bottom: 24px;
}

.about-info h4 {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 8px 0;
}

.about-info .version {
  color: var(--accent-blue, #0969da);
  font-size: 14px;
  font-weight: 500;
  margin: 0 0 8px 0;
}

.about-info .desc {
  color: var(--text-muted, #8c959f);
  font-size: 14px;
  margin: 0;
}

.help-panel {
  max-width: 100%;
  height: calc(100vh - 150px);
}

.help-panel h3 {
  margin-bottom: 16px;
}

.iframe-container {
  width: 100%;
  height: calc(100% - 40px);
  border-radius: .375rem;
  overflow: hidden;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.iframe-container iframe {
  width: 100%;
  height: 100%;
  border: none;
  background-color: white;
}

@media (max-width: 768px) {
  .settings-page {
    flex-direction: column;
  }

  .settings-sidebar {
    width: 100%;
    padding: 16px;
    border-right: none;
    border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  }

  .settings-nav {
    flex-direction: row;
    flex-wrap: wrap;
    gap: 8px;
  }

  .nav-item {
    padding: 8px 12px;
    font-size: 13px;
  }

  .nav-item span {
    display: none;
  }

  .settings-content {
    padding: 20px;
  }

  .help-panel {
    height: calc(100vh - 250px);
  }

  .scale-container {
    flex-direction: column;
    align-items: flex-start;
  }
}

/* FAQ 弹窗样式 */
.faq-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: flex-end;
  z-index: 9999;
  backdrop-filter: blur(4px);
}

.faq-panel {
  width: 60vw;
  max-width: 90vw;
  height: 100vh;
  background-color: var(--bg-primary, #ffffff);
  box-shadow: -4px 0 24px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  border-left: 1px solid var(--border-color, #d0d7de);
}

/* 亮色模式 */
.faq-panel.light-mode {
  background-color: var(--bg-primary, #ffffff);
}

.faq-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
}

.faq-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
}

.close-btn {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: none;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary, #57606a);
  transition: all 0.2s;
}

.close-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
}

.faq-body {
  flex: 1;
  overflow: hidden;
}

.faq-body iframe {
  width: 100%;
  height: 100%;
  border: none;
}

/* 从右侧滑入动画 */
.slide-right-enter-active {
  transition: all 0.3s ease-out;
}

.slide-right-leave-active {
  transition: all 0.3s ease-in;
}

.slide-right-enter-from,
.slide-right-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

.slide-right-enter-from .faq-overlay,
.slide-right-leave-to .faq-overlay {
  opacity: 0;
}
</style>
