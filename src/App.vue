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

import { computed, ref, provide } from 'vue'
import { NConfigProvider, darkTheme, useOsTheme, NLoadingBarProvider, NMessageProvider, NDialogProvider } from 'naive-ui'
import { RouterView } from 'vue-router'
import { useFileStore } from './stores/useFileStore'

const osTheme = useOsTheme()
const fileStore = useFileStore()

// 用户选择的主题（null表示跟随系统）
const userTheme = ref<'light' | 'dark' | null>(null)

// 实际使用的主题
const currentTheme = computed(() => {
  if (userTheme.value === 'dark') return darkTheme
  if (userTheme.value === 'light') return undefined
  return osTheme.value === 'dark' ? darkTheme : undefined
})

// 当前主题名称（用于CSS类）
const themeClass = computed(() => {
  if (userTheme.value) return userTheme.value
  return osTheme.value
})

// 切换主题的函数
function toggleTheme() {
  if (userTheme.value === 'dark') {
    userTheme.value = 'light'
  } else if (userTheme.value === 'light') {
    userTheme.value = null // 恢复跟随系统
  } else {
    // 当前跟随系统，切换到相反的主题
    userTheme.value = osTheme.value === 'dark' ? 'light' : 'dark'
  }
}

// 提供主题切换函数给子组件使用
provide('toggleTheme', toggleTheme)
provide('currentTheme', userTheme)

const themeOverrides = {
  common: {
    primaryColor: '#0066ff',
    primaryColorHover: '#2980ff',
    primaryColorPressed: '#0052cc',
    primaryColorSuppl: '#2980ff',
    
    infoColor: '#0066ff',
    successColor: '#4caf50',
    warningColor: '#ff9800',
    errorColor: '#ff6b6b',
    
    borderRadius: '12px',
    borderRadiusSmall: '8px',
    borderRadiusLarge: '16px',
    
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
  }
}

// 初始化文件存储
fileStore.init()
</script>

<template>
  <NConfigProvider 
    :theme="currentTheme" 
    :theme-overrides="themeOverrides"
    class="config-provider"
  >
    <NLoadingBarProvider>
      <NMessageProvider>
        <NDialogProvider>
          <div class="app" :class="themeClass">
            <RouterView />
          </div>
        </NDialogProvider>
      </NMessageProvider>
    </NLoadingBarProvider>
  </NConfigProvider>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --n-color-body: #f5f5f7;
  --n-color-modal: #ffffff;
  --n-color-hover: #f0f0f0;
  --n-color-info-hover: #e6f2ff;
}

.dark {
  --n-color-body: #1a1a1a;
  --n-color-modal: #363636;
  --n-color-hover: #404040;
  --n-color-info-hover: #1a3a5f;
  
  /* 提高文字对比度 - 确保所有文字都是浅色 */
  --file-text-color: #ffffff;
  --file-text-secondary: #b0b0b0;
  
  /* 确保Naive UI的文本颜色变量在暗色模式下也是浅色 */
  --n-text-color: #ffffff;
  --n-text-color-disabled: #b0b0b0;
  --n-text-color-secondary: #cccccc;
  
  /* 图标颜色 */
  --n-color-info: #66aaff;
  --n-color-success: #4caf50;
  --n-color-warning: #ff9800;
  --n-color-error: #ff6b6b;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: var(--n-color-body);
  color: var(--n-text-color);
  overflow: hidden;
}

#app {
  height: 100vh;
  overflow: hidden;
}

.config-provider {
  height: 100%;
}

.app {
  height: 100%;
  transition: background-color 0.3s ease;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.dark ::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
}

.dark ::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>
