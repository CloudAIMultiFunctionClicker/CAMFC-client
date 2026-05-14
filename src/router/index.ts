

import { createRouter, createWebHistory } from 'vue-router'

import { useBluetoothStore } from '../stores/bluetooth.js'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

const router = createRouter({

  history: createWebHistory(),

  routes: [
    {
      path: '/',
      name: 'initialView',
      component: ()=>import('../views/InitialView.vue'),
    },
    {
      path: '/welcome',
      name: 'welcome',
      component: () => import('../views/WelcomeView.vue'),
    },
    {path:'/fileView',
      name: 'fileView',
      component: () => import('../views/FileView.vue')
    },

    {
      path: '/about',
      name: 'about',

      component: () => import('../views/AboutView.vue')
    },
    {
      path: '/contact',
      name: 'contact',

      component: () => import('../views/ContactView.vue')
    },
    {
      path: '/settings',
      redirect: '/settings_cpen'
    },
    {
      path: '/settings_cpen',
      name: 'settingsCpen',
      component: () => import('../views/SettingsCpen.vue')
    },
    {
      path: '/settings_hardware',
      name: 'settingsHardware',
      component: () => import('../views/SettingsHardware.vue')
    },
    {
      path: '/settings_student',
      name: 'settingsStudent',
      component: () => import('../views/SettingsStudent.vue')
    },
    {
      path: '/settings_download',
      name: 'settingsDownload',
      component: () => import('../views/SettingsDownload.vue')
    },
    {
      path: '/settings_application',
      name: 'settingsApplication',
      component: () => import('../views/SettingsApplication.vue')
    },
    {
      path: '/settings_theme',
      name: 'settingsTheme',
      component: () => import('../views/SettingsTheme.vue')
    },
    {
      path: '/settings_help',
      name: 'settingsHelp',
      component: () => import('../views/SettingsHelp.vue')
    },
    {
      path: '/settings_about',
      name: 'settingsAbout',
      component: () => import('../views/SettingsAbout.vue')
    },

    {
      path: '/more-info',
      name: 'moreInfo',

      component: () => import('../views/MoreInfo.vue')
    },
    {
      path: '/notes',
      name: 'notes',

      component: () => import('../views/Notes.vue'),
      props: route => ({ defaultTab: route.query.tab || 'notes' })
    },

    {
      path: '/notes_notes',
      name: 'notesNotes',
      component: () => import('../views/Notes.vue'),
      props: () => ({ defaultTab: 'notes' })
    },
    {
      path: '/notes_meetings',
      name: 'notesMeetings',
      component: () => import('../views/Notes.vue'),
      props: () => ({ defaultTab: 'meetings' })
    },
    {
      path: '/transfer',
      name: 'transfer',

      component: () => import('../views/TransferView.vue')
    },
    {
      path: '/float',
      name: 'float',

      component: () => import('../views/FloatView.vue')
    },
    {
      path: '/float-normal',
      name: 'float-normal',

      component: () => import('../views/FloatNormalView.vue')
    },
    {
      path: '/screenshot',
      name: 'screenshot',

      component: () => import('../views/ScreenshotView.vue')
    },
    {
      path: '/note-editor',
      name: 'noteEditor',

      component: () => import('../views/NoteEditorWindow.vue')
    },
    {
      path: '/meeting-editor',
      name: 'meetingEditor',

      component: () => import('../views/MeetingEditorWindow.vue')
    },
    {
      path: '/recent-activities',
      name: 'recentActivities',

      component: () => import('../views/RecentActivities.vue')
    },
    {
      path: '/empty',
      name: 'empty',

      component: () => import('../views/Empty.vue')
    },
    {
      path: '/screenshot-window',
      name: 'screenshotWindow',

      component: () => import('../views/ScreenshotWindow.vue')
    },
    {
      path: '/float-normal-empty',
      name: 'float-normal-empty',

      component: () => import('../views/FloatNormalEmpty.vue')
    },
    {
      path: '/group-manager',
      name: 'groupManager',

      component: () => import('../views/GroupManager.vue'),
      props: route => ({ defaultTab: route.query.tab || 'groups' })
    },

    {
      path: '/group-manager_groups',
      name: 'groupManagerGroups',
      component: () => import('../views/GroupManager.vue'),
      props: () => ({ defaultTab: 'groups' })
    },
    {
      path: '/group-manager_applications',
      name: 'groupManagerApplications',
      component: () => import('../views/GroupManager.vue'),
      props: () => ({ defaultTab: 'applications' })
    },
    {
      path: '/group-detail',
      name: 'groupDetail',

      component: () => import('../views/GroupDetail.vue')
    },
    {
      path: '/agent-window',
      name: 'agentWindow',

      component: () => import('../views/AgentWindow.vue')
    },
    {
      path: '/note-viewer',
      name: 'noteViewer',

      component: () => import('../views/NoteViewerWindow.vue')
    }

  ]
})

router.beforeEach(async (to, _from, next) => {

  let windowLabel = ''
  try {
    const currentWindow = await getCurrentWebviewWindow()
    windowLabel = currentWindow?.label || ''
  } catch (e) {

    console.warn('[路由守卫] 获取窗口标签失败，按主窗口处理:', e)
    windowLabel = 'main'
  }

  console.log(`[路由守卫] 当前窗口：${windowLabel}, 目标路由：${to.path}`)

  if (windowLabel.startsWith('screenshot-')) {
    const allowedPaths = ['/screenshot-window', '/screenshot']
    if (!allowedPaths.includes(to.path)) {
      console.warn(`[路由守卫] 截图窗口禁止访问 ${to.path}，强制跳转到 /screenshot-window`)
      next('/screenshot-window')
      return
    }

    next()
    return
  }

  if (windowLabel.startsWith('note-editor-')) {
    const allowedPaths = ['/note-editor']
    if (!allowedPaths.includes(to.path)) {
      console.warn(`[路由守卫] 笔记编辑窗口禁止访问 ${to.path}，强制跳转到 /note-editor`)
      next('/note-editor')
      return
    }

    next()
    return
  }

  if (windowLabel.startsWith('meeting-editor-')) {
    const allowedPaths = ['/meeting-editor']
    if (!allowedPaths.includes(to.path)) {
      console.warn(`[路由守卫] 会议记录编辑窗口禁止访问 ${to.path}，强制跳转到 /meeting-editor`)
      next('/meeting-editor')
      return
    }

    next()
    return
  }

  if (to.path === '/float' || to.path === '/float-normal' || to.path === '/float-normal-empty') {
    next()
    return
  }

  if (to.path === '/empty') {
    next()
    return
  }

  if (windowLabel.startsWith('agent-')) {
    const allowedPaths = ['/agent-window']
    if (!allowedPaths.includes(to.path)) {
      console.warn(`[路由守卫] agent 窗口禁止访问 ${to.path}，强制跳转到 /agent-window`)
      next('/agent-window')
      return
    }

    next()
    return
  }

  if (windowLabel.startsWith('note-viewer-')) {
    const allowedPaths = ['/note-viewer']
    if (!allowedPaths.includes(to.path)) {
      console.warn(`[路由守卫] 笔记查看窗口禁止访问 ${to.path}，强制跳转到 /note-viewer`)
      next('/note-viewer')
      return
    }

    next()
    return
  }

  if (windowLabel === 'main') {
    const forbiddenPaths = ['/screenshot-window', '/note-editor', '/screenshot']
    if (forbiddenPaths.includes(to.path)) {
      console.warn(`[路由守卫] 主窗口禁止访问 ${to.path}，强制跳转到 /`)
      next('/')
      return
    }
  }

  if (to.path === '/') {
    next()
    return
  }

  if (to.path === '/welcome') {

    const store = useBluetoothStore()
    const connected = store.isConnected()
    if (connected) {
      next()
    } else {
      console.warn('[路由守卫] 蓝牙未连接，阻止访问欢迎页面')
      next('/')
    }
    return
  }

  if (to.path === '/screenshot') {
    next()
    return
  }

  if (to.path === '/notes' || to.path.startsWith('/notes_')) {
    next()
    return
  }

  if (to.path === '/group-manager' || to.path.startsWith('/group-manager_')) {
    next()
    return
  }

  if (to.path === '/settings' || to.path === '/settings_cpen' || to.path === '/settings_hardware') {
    next()
    return
  }

  if (to.path === '/note-editor') {
    next()
    return
  }

  if (to.path === '/meeting-editor') {
    next()
    return
  }

  if (to.path === '/screenshot-window') {
    next()
    return
  }

  const bluetoothStore = useBluetoothStore()

  const connected = bluetoothStore.isConnected()
  console.log(`[路由守卫] 目标：${to.path}, 蓝牙状态：${bluetoothStore.bluetoothStatus}, 连接：${connected}`)

  if (connected) {
    console.log('[路由守卫] 蓝牙已连接，允许跳转')
    next()
  } else {

    if (_from.path === '/' && bluetoothStore.bluetoothStatus === 'connected') {
      console.log('[路由守卫] 检测到状态同步延迟，等待 200ms 后重试')

      setTimeout(() => {
        const retryConnected = bluetoothStore.isConnected()
        if (retryConnected) {
          console.log('[路由守卫] 重试成功，蓝牙已连接，允许跳转')
          next()
        } else {
          console.warn('[路由守卫] 重试失败，蓝牙未连接，阻止跳转到:', to.path)
          next('/')
        }
      }, 200)
    } else {

      console.warn('[路由守卫] 蓝牙未连接，阻止跳转到:', to.path)
      next('/')
    }
  }
})

export default router
