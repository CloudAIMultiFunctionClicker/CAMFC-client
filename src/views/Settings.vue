<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top

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
          <span>自动连接 Cpen 设备</span>
          <button 
            class="toggle-btn" 
            :class="{ active: cpenSettings.autoConnect }"
            @click="toggleAutoConnect"
          >
            <span class="toggle-slider"></span>
          </button>
        </div>
        <div class="setting-item">
          <span>设备名称</span>
          <span class="setting-value">{{ deviceId || '未连接' }}</span>
        </div>
      </div>

      <div v-else-if="activeNav === 'account'" class="settings-panel">
        <h3>账户</h3>
        <div class="setting-item">
          <span>登录状态</span>
          <span class="setting-value" :class="isFilesystemLoggedIn ? 'status-online' : 'status-offline'">
            {{ isFilesystemLoggedIn ? '已登录' : '未登录' }}
          </span>
        </div>
        <div class="setting-item">
          <span>用户名</span>
          <span class="setting-value">{{ deviceId || '未连接' }}</span>
        </div>
        <button class="action-btn danger" @click="logout">退出登录</button>
      </div>

      <div v-else-if="activeNav === 'ui'" class="settings-panel">
        <h3>界面设置</h3>
        <div class="placeholder-text">界面设置功能开发中...</div>
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
          <button class="toggle-btn active">
            <span class="toggle-slider"></span>
          </button>
        </div>
      </div>

      <div v-else-if="activeNav === 'storage'" class="settings-panel">
        <h3>储存空间管理</h3>
        <div class="storage-info">
          <div class="storage-bar">
            <div class="storage-used" style="width: 35%"></div>
          </div>
          <p class="storage-text">已使用 350 MB / 1 GB</p>
        </div>
        <button class="action-btn" @click="clearCache">清理缓存</button>
        <div class="setting-item">
          <span>自动清理缓存</span>
          <button class="toggle-btn">
            <span class="toggle-slider"></span>
          </button>
        </div>
      </div>

      <div v-else-if="activeNav === 'help'" class="settings-panel help-panel">
        <h3>帮助与反馈</h3>
        <p class="placeholder-text">帮助与反馈内容占位...</p>
      </div>

      <div v-else-if="activeNav === 'about'" class="settings-panel">
        <h3>关于</h3>
        <div class="about-info">
          <div class="app-icon">☁️</div>
          <h4>CAMFC Cloud</h4>
          <p class="version">版本 1.0.0</p>
          <p class="desc">云端多功能点击器客户端</p>
        </div>
        <div class="setting-item">
          <span>检查更新</span>
          <span class="setting-value">已是最新</span>
        </div>
        <button class="action-btn">查看更新日志</button>
      </div>
    </main>
  </div>
</template>

<script setup>
import { inject, ref, onMounted } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { disconnect, getDeviceId } from '../components/data/bluetooth.js'
import { ls } from '../components/data/fileSystem.js'

const theme = inject('theme')
const activeNav = ref('cpen')

const cpenSettings = ref({
  autoConnect: false
})



const deviceId = ref(null)
const isFilesystemLoggedIn = ref(false)

const navItems = [
  { id: 'cpen', label: 'Cpen 设置', icon: 'ri-settings-3-line' },
  { id: 'account', label: '账户', icon: 'ri-user-line' },
  { id: 'ui', label: '界面设置', icon: 'ri-layout-grid-line' },
  { id: 'theme', label: '深色模式', icon: 'ri-moon-line' },
  { id: 'storage', label: '储存空间管理', icon: 'ri-hard-drive-line' },
  { id: 'help', label: '帮助与反馈', icon: 'ri-question-line' },
  { id: 'about', label: '关于', icon: 'ri-information-line' }
]



const toggleAutoConnect = () => {
  cpenSettings.value.autoConnect = !cpenSettings.value.autoConnect
  const status = cpenSettings.value.autoConnect ? '已启用' : '已禁用'
  showToast(`自动连接 Cpen 设备：${status}`, '#3b82f6')
}

const checkFilesystemLogin = async () => {
  try {
    let id = null
    let cloudAccessible = false
    
    try {
      id = await getDeviceId()
      deviceId.value = id
    } catch (idError) {
      console.warn('获取设备ID失败:', idError)
    }
    
    if (id) {
      try {
        const result = await ls('/')
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





const clearCache = () => {
  showToast('正在清理缓存...', '#f59e0b')
  setTimeout(() => {
    showToast('缓存清理完成', '#10b981')
  }, 1000)
}

onMounted(() => {
  checkFilesystemLogin()
})
</script>

<style scoped>
.settings-page {
  display: flex;
  min-height: 100vh;
  background-color: #f5f5f5;
}

.settings-sidebar {
  width: 260px;
  background-color: #fff;
  border-right: 1px solid #eee;
  padding: 24px 16px;
  flex-shrink: 0;
}

.sidebar-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
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
  border-radius: 4px;
  color: #666;
  font-size: 14px;
  cursor: pointer;
  text-align: left;
}

.nav-item:hover {
  background-color: #f5f5f5;
  color: #333;
}

.nav-item.active {
  background-color: #333;
  color: white;
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
}

.settings-panel {
  max-width: 600px;
}

.settings-panel h3 {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 24px 0;
}

.placeholder-text {
  color: #999;
  font-size: 15px;
  line-height: 1.6;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background-color: #fff;
  border-radius: 4px;
  margin-bottom: 12px;
  color: #333;
  font-size: 15px;
}

.toggle-btn {
  position: relative;
  width: 48px;
  height: 26px;
  background-color: #ddd;
  border: none;
  border-radius: 13px;
  cursor: pointer;
}

.toggle-btn.active {
  background-color: #333;
}

.toggle-slider {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 20px;
  height: 20px;
  background-color: white;
  border-radius: 50%;
}

.toggle-btn.active .toggle-slider {
  transform: translateX(22px);
}

.setting-value {
  color: #999;
  font-size: 14px;
}

.setting-value.status-online {
  color: #2a7;
}

.setting-value.status-offline {
  color: #c00;
}

.action-btn {
  margin-top: 16px;
  padding: 12px 24px;
  background-color: #333;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
}

.action-btn:hover {
  background-color: #555;
}

.action-btn.secondary {
  background-color: #fff;
  color: #666;
  border: 1px solid #ddd;
}

.action-btn.danger {
  background-color: #fff;
  color: #c00;
  border: 1px solid #ddd;
}

.action-btn.danger:hover {
  background-color: #c00;
  color: white;
}

.storage-info {
  margin-bottom: 20px;
}

.storage-bar {
  height: 8px;
  background-color: #eee;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 8px;
}

.storage-used {
  height: 100%;
  background-color: #333;
  border-radius: 4px;
}

.storage-text {
  color: #999;
  font-size: 13px;
  margin: 0;
}

.about-info {
  text-align: center;
  padding: 32px;
  background-color: #fff;
  border-radius: 4px;
  margin-bottom: 24px;
}

.app-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.about-info h4 {
  font-size: 1.25rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 8px 0;
}

.about-info .version {
  color: #333;
  font-size: 14px;
  font-weight: 500;
  margin: 0 0 8px 0;
}

.about-info .desc {
  color: #999;
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
  border-radius: 4px;
  overflow: hidden;
  border: 1px solid #eee;
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
    border-bottom: 1px solid #eee;
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
</style>
