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

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

// Naive UI
import {
  // 全局组件
  create,
  // 主题
  darkTheme,
  // 组件
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NLoadingBarProvider,
  NNotificationProvider
} from 'naive-ui'

// 创建Naive UI实例
const naive = create({
  components: [
    NConfigProvider,
    NMessageProvider,
    NDialogProvider,
    NLoadingBarProvider,
    NNotificationProvider
  ]
})

// 自定义主题
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
    fontSize: '14px',
    fontSizeMini: '12px',
    fontSizeSmall: '13px',
    fontSizeMedium: '14px',
    fontSizeLarge: '16px',
  },
  Card: {
    borderRadius: '12px',
    paddingMedium: '20px'
  },
  Button: {
    borderRadiusMedium: '10px',
    borderRadiusSmall: '8px',
    borderRadiusLarge: '12px'
  },
  Input: {
    borderRadius: '10px'
  },
  Modal: {
    borderRadius: '16px'
  },
  Tag: {
    borderRadius: '6px'
  },
  Alert: {
    borderRadius: '12px',
    paddingMedium: '16px'
  }
}

const app = createApp(App)

// 使用Pinia
const pinia = createPinia()
app.use(pinia)

// 使用Naive UI
app.use(naive)

// 使用路由
app.use(router)

// 挂载应用
app.mount('#app')

// 导出app实例，方便调试
export { app }
