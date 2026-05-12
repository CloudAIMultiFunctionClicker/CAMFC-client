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
.settings-page {
  display: flex;
  height: 100%;
  background-color: var(--bg-primary, #ffffff);
  overflow: hidden;
}

.settings-content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
  background-color: var(--bg-primary, #ffffff);
  height: 100%;
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

.path-input {
  flex: 1;
  padding: 10px 14px;
  background-color: var(--input-bg, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
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

.toggle-btn {
  position: relative;
  width: 48px;
  height: 26px;
  background-color: var(--border-color, #d0d7de);
  border: none;
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
  overflow: hidden;
  margin-bottom: 8px;
}

.storage-used {
  height: 100%;
  background: var(--accent-blue, #3178c6);
  border-radius: 2px;
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
  border-radius: 2px;
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

/* 开源软件声明样式 */
.opensource-desc {
  font-size: 14px;
  color: var(--text-secondary, #57606a);
  margin: 0 0 16px 0;
}

.opensource-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.opensource-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.opensource-item:hover {
  border-color: var(--accent-blue, #0969da);
  background-color: var(--selected-bg, #ddf4ff);
}

.lib-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--accent-blue, #0969da);
  cursor: pointer;
  transition: color 0.2s ease;
}

.lib-name:hover {
  text-decoration: underline;
}

.lib-license {
  font-size: 12px;
  color: var(--text-muted, #8c959f);
  font-family: monospace;
  cursor: pointer;
  transition: color 0.2s ease;
}

.lib-license:hover {
  color: var(--accent-blue, #0969da);
  text-decoration: underline;
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
  border-radius: 2px;
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

.feedback-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.feedback-options {
  display: flex;
  flex-direction: row;
  gap: 12px;
  padding: 12px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  animation: fadeIn 0.3s ease-out;
  align-items: center;
}

.feedback-options .action-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  background-color: var(--bg-secondary, #ffffff);
  color: var(--text-primary, #333);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.feedback-options .action-btn:hover {
  background-color: var(--hover-bg, #f5f5f5);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.feedback-options .action-btn i {
  font-size: 14px;
}

.email-container {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 3px;
}

.email-address {
  font-size: 13px;
  color: var(--text-secondary, #57606a);
  font-family: inherit;
  white-space: nowrap;
  padding: 8px 0;
  margin-top: 13px;
  cursor: pointer;
  transition: color 0.2s ease;
}

.email-address:hover {
  color: var(--accent-blue, #0969da);
}

.copy-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  margin-top: 11px;
  background-color: transparent;
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  color: var(--text-secondary, #57606a);
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.copy-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
  border-color: var(--accent-blue, #0969da);
}

.copy-btn i {
  font-size: 14px;
}

.cancel-btn {
  width: 100%;
  background-color: transparent;
  border: 1px solid var(--border-color, #ddd);
  color: var(--text-secondary, #666);
  margin-top: 8px;
}

.cancel-btn:hover {
  background-color: var(--hover-bg, #f5f5f5);
  color: var(--text-primary, #333);
}

.feedback-container .action-btn {
  width: auto;
  max-width: 200px;
}

.feedback-actions {
  display: flex;
  justify-content: flex-start;
  gap: 12px;
  margin-top: 0;
}

.feedback-actions .action-btn {
  padding: 10px 20px;
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  width: auto;
  max-width: none;
  flex: none;
}

.feedback-actions .action-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
}

.feedback-actions .cancel-btn {
  background-color: transparent;
  color: var(--text-secondary, #666);
}

.feedback-actions .cancel-btn:hover {
  background-color: var(--hover-bg, #f5f5f5);
  color: var(--text-primary, #333);
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 学生认证设置样式 */
.setting-actions {
  display: flex;
  gap: 12px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color, #d0d7de);
}

.setting-tip {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin-top: 16px;
  padding: 12px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border-radius: 2px;
  border: 1px solid var(--border-color, #d0d7de);
  font-size: 13px;
  color: var(--text-secondary, #57606a);
}

.setting-tip i {
  font-size: 16px;
  color: var(--accent-blue, #0969da);
  flex-shrink: 0;
  margin-top: 1px;
}

.text-input {
  width: 100%;
  padding: 10px 14px;
  background-color: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  color: var(--text-primary, #24292f);
  font-size: 14px;
  transition: all 0.2s ease;
}

.text-input:focus {
  outline: none;
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.text-input::placeholder {
  color: var(--text-muted, #8c959f);
}
</style>
