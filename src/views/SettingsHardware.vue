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
  <div class="settings-panel">
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
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData, saveAppData } from '../components/data/storage.js'

const hardwareSettings = ref({
  keepAliveInterval: 30
})

const cpenBluetoothVersion = ref('未连接')
const localBluetoothVersion = ref('5.0')

// 保活定时器
let keepAliveTimer = null

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

onMounted(() => {
  loadHardwareSettings()
  fetchBluetoothVersions()
  
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
  border-radius: 2px;
  margin-bottom: 12px;
  color: var(--text-primary, #24292f);
  font-size: 15px;
}

.setting-card {
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
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
  border-radius: 2px;
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

.number-input {
  width: 80px;
  padding: 8px 12px;
  background-color: var(--input-bg, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  color: var(--text-primary, #24292f);
  font-size: 14px;
  text-align: center;
}

.number-input:focus {
  outline: none;
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
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
  border-radius: 2px;
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
  border-radius: 2px;
  border: 1px solid var(--border-color, #d0d7de);
  font-size: 12px;
  color: var(--text-secondary, #57606a);
}

.refresh-tip i {
  font-size: 14px;
  color: var(--accent-blue, #0969da);
}
</style>
