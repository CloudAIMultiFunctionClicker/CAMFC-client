<script setup lang="ts">
/*
 * Copyright (C) 2026 Jiale Xu (ANTmmmmm) (ant-cave)
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { computed, h, ref, inject } from 'vue'
import { 
  NBreadcrumb, 
  NBreadcrumbItem, 
  NButton, 
  NSpace, 
  NInput,
  NSelect,
  NDropdown,
  NIcon,
  NAvatar
} from 'naive-ui'
import { 
  SearchOutline,
  AddOutline,
  ShareOutline,
  TrashOutline,
  CloudUploadOutline,
  GridOutline,
  ListOutline,
  PersonCircleOutline,
  SettingsOutline,
  LogOutOutline,
  MoonOutline,
  SunnyOutline,
  DesktopOutline
} from '@vicons/ionicons5'
import { useFileStore } from '@/stores/useFileStore'

const fileStore = useFileStore()

// 从App.vue获取主题切换函数
const toggleTheme = inject<() => void>('toggleTheme')
const currentTheme = inject<'light' | 'dark' | null>('currentTheme')

// 计算面包屑路径
const breadcrumbs = computed(() => {
  const path = fileStore.currentPath
  if (path === '/') return [{ name: '全部文件', path: '/' }]
  
  const parts = path.split('/').filter(p => p)
  const result = [{ name: '全部文件', path: '/' }]
  
  let currentPath = ''
  for (const part of parts) {
    currentPath += '/' + part
    result.push({ name: decodeURIComponent(part), path: currentPath })
  }
  
  return result
})

// 搜索关键词
const searchQuery = ref('')

// 视图模式选项
const viewOptions = [
  { label: '网格视图', value: 'grid', icon: () => h(GridOutline) },
  { label: '列表视图', value: 'list', icon: () => h(ListOutline) }
]

// 用户菜单选项 - 使用计算属性来动态更新
const userMenuOptions = computed(() => {
  const options = [
    { label: '个人中心', key: 'profile', icon: () => h(PersonCircleOutline) },
    { label: '设置', key: 'settings', icon: () => h(SettingsOutline) },
    { 
      label: currentTheme === null ? '跟随系统' : (currentTheme === 'dark' ? '浅色模式' : '深色模式'), 
      key: 'theme', 
      icon: () => {
        if (currentTheme === null) return h(DesktopOutline)
        return h(currentTheme === 'dark' ? SunnyOutline : MoonOutline)
      }
    },
    { type: 'divider', key: 'divider' },
    { label: '退出登录', key: 'logout', icon: () => h(LogOutOutline) }
  ]
  return options
})

function handleSearch() {
  console.log('搜索:', searchQuery.value)
  // 实际项目中这里应该触发搜索
}

function handleUploadClick() {
  console.log('点击上传按钮')
  // 实际项目中这里应该触发文件选择
}

function handleShareClick() {
  console.log('点击分享按钮')
  // 实际项目中这里应该打开分享弹窗
}

function handleViewModeChange(mode: 'grid' | 'list') {
  fileStore.switchViewMode(mode)
}

function handleUserMenuSelect(key: string) {
  console.log('用户菜单选择:', key)
  switch (key) {
    case 'theme':
      // 切换主题
      if (toggleTheme) {
        toggleTheme()
      }
      break
    case 'logout':
      console.log('退出登录')
      break
  }
}

</script>

<template>
  <header class="app-header">
    <div class="header-content">
      <!-- 左侧：logo和面包屑 -->
      <div class="header-left">
        <div class="logo">
          <NIcon size="28" color="#0066ff">
            <CloudUploadOutline />
          </NIcon>
          <span class="logo-text">CAMFC Cloud</span>
        </div>
        
        <NBreadcrumb separator=">">
          <NBreadcrumbItem
            v-for="(crumb, index) in breadcrumbs"
            :key="crumb.path"
            :href="index < breadcrumbs.length - 1 ? '#' : undefined"
            @click="index < breadcrumbs.length - 1 && fileStore.navigateTo(crumb.path)"
          >
            {{ crumb.name }}
          </NBreadcrumbItem>
        </NBreadcrumb>
      </div>
      
      <!-- 右侧：搜索和操作按钮 -->
      <div class="header-right">
        <div class="search-box">
          <NInput
            v-model:value="searchQuery"
            placeholder="搜索文件或文件夹..."
            clearable
            @keyup.enter="handleSearch"
          >
            <template #prefix>
              <NIcon :component="SearchOutline" />
            </template>
          </NInput>
        </div>
        
        <NSpace :wrap="false" size="small">
          <NSelect
            v-model:value="fileStore.viewMode"
            :options="viewOptions"
            style="width: 120px"
            @update:value="handleViewModeChange"
          />
          
          <NButton type="primary" @click="handleUploadClick">
            <template #icon>
              <NIcon :component="AddOutline" :color="currentTheme === 'dark' ? '#ffffff' : undefined" />
            </template>
            上传
          </NButton>
          
          <NButton 
            type="info" 
            :disabled="fileStore.selectedFiles.length === 0"
            @click="handleShareClick"
          >
            <template #icon>
              <NIcon :component="ShareOutline" :color="currentTheme === 'dark' ? '#ffffff' : undefined" />
            </template>
            分享
          </NButton>
          
          <NButton 
            type="error" 
            :disabled="fileStore.selectedFiles.length === 0"
            @click="fileStore.selectedFiles.forEach(f => fileStore.removeFile(f.id))"
          >
            <template #icon>
              <NIcon :component="TrashOutline" :color="currentTheme === 'dark' ? '#ffffff' : undefined" />
            </template>
            删除
          </NButton>
          
          <NDropdown
            :options="userMenuOptions"
            @select="handleUserMenuSelect"
            trigger="click"
          >
            <NAvatar
              round
              size="medium"
              style="cursor: pointer; background-color: var(--n-color-info); border: 2px solid var(--n-color-info-hover)"
            >
              <NIcon :component="PersonCircleOutline" size="24" :color="currentTheme === 'dark' ? '#ffffff' : '#000000'" />
            </NAvatar>
          </NDropdown>
        </NSpace>
      </div>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  backdrop-filter: blur(20px);
  background: rgba(255, 255, 255, 0.7);
  border-bottom: 1px solid var(--n-border-color);
  padding: 0 24px;
  height: 64px;
  display: flex;
  align-items: center;
  position: sticky;
  top: 0;
  z-index: 1000;
}

.dark .app-header {
  background: rgba(30, 30, 30, 0.7);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  font-size: 18px;
  color: var(--n-text-color);
}

.logo-text {
  background: linear-gradient(135deg, #0066ff, #4caf50);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.search-box {
  width: 280px;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .app-header {
    padding: 0 16px;
  }
  
  .search-box {
    width: 200px;
  }
}

@media (max-width: 768px) {
  .header-left .logo-text {
    display: none;
  }
  
  .search-box {
    display: none;
  }
}
</style>
<!--  -->