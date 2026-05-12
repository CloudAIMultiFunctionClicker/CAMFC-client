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

<script setup>
import { inject, ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

// 头部组件 - 现在加了主题切换功能
// 之前试过加点击事件，但好像会跟路由冲突？先放着不管
// FIXME: 云按钮点了没反应，得找时间加上去
// TODO: 按钮的状态管理还没做，比如上传中的 loading 状态

// 从 App.vue 注入的主题功能
const theme = inject('theme')

// 按钮状态
const buttonPressed = ref(false)

// 监听按钮状态变化
const handleButtonState = (event) => {
  buttonPressed.value = event.detail.pressed
}

onMounted(() => {
  window.addEventListener('button-state', handleButtonState)
})

onUnmounted(() => {
  window.removeEventListener('button-state', handleButtonState)
})

// 窗口控制功能
const currentWindow = getCurrentWindow()

// 窗口是否最大化状态
const isMaximized = ref(false)

// 初始化时检查窗口状态
const checkWindowState = async () => {
  try {
    isMaximized.value = await currentWindow.isMaximized()
  } catch (error) {
    console.error('检查窗口状态失败:', error)
  }
}

// 监听窗口大小变化，更新状态
let resizeObserver = null
const setupResizeObserver = () => {
  resizeObserver = new ResizeObserver(() => {
    checkWindowState()
  })
  resizeObserver.observe(document.body)
}

onMounted(() => {
  window.addEventListener('button-state', handleButtonState)
  checkWindowState()
  setupResizeObserver()
})

onUnmounted(() => {
  window.removeEventListener('button-state', handleButtonState)
  if (resizeObserver) {
    resizeObserver.disconnect()
  }
})

// 最小化窗口
const minimizeWindow = async () => {
  try {
    console.log('正在最小化窗口...')
    await currentWindow.minimize()
    console.log('窗口最小化成功')
  } catch (error) {
    console.error('最小化窗口失败:', error)
  }
}

// 最大化/还原窗口
const toggleMaximize = async () => {
  try {
    console.log('切换最大化状态...')
    if (isMaximized.value) {
      console.log('正在还原窗口...')
      await currentWindow.unmaximize()
      console.log('窗口已还原')
    } else {
      console.log('正在最大化窗口...')
      await currentWindow.maximize()
      console.log('窗口已最大化')
    }
    // 更新状态
    isMaximized.value = !isMaximized.value
  } catch (error) {
    console.error('切换最大化失败:', error)
  }
}

// 关闭应用（通过托盘退出）
const closeApp = async () => {
  try {
    await invoke('exit_app')
  } catch (error) {
    console.error('退出应用失败:', error)
  }
}

// 拖动窗口功能
const startWindowDrag = async (event) => {
  // 只在左键点击时拖动
  if (event.button !== 0) return
  
  // 防止在按钮上点击时拖动
  if (event.target.closest('button, a, .window-controls')) return
  
  try {
    await currentWindow.startDragging()
  } catch (error) {
    console.error('拖动窗口失败:', error)
  }
}
</script>


<template>
    <!-- 顶部工具栏容器 -->
    <header class="header">
        <div class="toolbar" @mousedown="startWindowDrag">
            <!-- 左侧：应用标题和云按钮 -->
            <h1>
                <span>CAMFC Cloud</span>
                <!-- 云按钮 - 现在使用 Remix Icon 云图标 -->
                <router-link to="/main">
                <button class="btn-cloud">
                    
                    <i class="ri-cloud-line"></i>
                </button></router-link>
            </h1>

            <!-- 右侧：操作按钮区域 -->
            <div class="operation">
                <!-- 主题切换按钮 -->
                <button class="btn-theme" @click="theme?.toggleTheme">
                    <!-- 亮色模式时显示月亮图标（切换到暗色），暗色模式时显示太阳图标（切换到亮色） -->
                    <i class="ri-moon-line" v-if="theme?.isLightMode.value"></i>
                    <i class="ri-sun-line" v-else></i>
                    <!-- 小屏幕时隐藏文字 -->
                    <span class="btn-text">{{ theme?.isLightMode.value ? '切换到暗色' : '切换到亮色' }}</span>
                </button>
                
                <!-- 窗口控制按钮组 -->
                <div class="window-controls">
                    <button class="window-control-btn" @click="minimizeWindow" title="最小化">
                        <i class="ri-subtract-line"></i>
                    </button>
                    <button class="window-control-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
                        <i :class="isMaximized ? 'ri-fullscreen-exit-line' : 'ri-fullscreen-line'"></i>
                    </button>
                    <button class="window-control-btn close-btn" @click="closeApp" title="关闭">
                        <i class="ri-close-line"></i>
                    </button>
                </div>
            </div>
        </div>
    </header>
</template>


<style scoped>
/* 头部样式 - 现在支持主题切换了 */
/* 之前用纯黑色太压抑了，试了几个渐变，这个看起来还行 */
/* TODO: 亮色模式的阴影可能需要调整，现在看起来还行 */

header {
    width: 100%;
    height: 48px;
    flex-shrink: 0;
    border-bottom: 1px solid var(--border-color, #30363d);
    background: var(--bg-header, #161b22);
    position: relative;
    z-index: 1000;
    transition: background 0.3s ease, border-color 0.3s ease;
}

/* 工具栏布局 */
.toolbar {
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    align-items: center;
    /* 两边留点空间 */
}
.toolbar>*:first-child {
    margin-left: 24px;
}
.toolbar>*:last-child {
    margin-right: 24px;

}

h1 {
    margin: 0;
    display: flex;
    align-items: center;
    gap: 12px;
    color: var(--text-primary, #f0f6fc);
    font-size: 1.2rem;
    font-weight: 450;
    letter-spacing: -0.025em;
    transition: color 0.3s ease;
}

/* 右侧按钮区域 */
.operation {
    display: flex;
    align-items: center;
    gap: 12px;
    /* 用gap代替margin-left/margin-right */
    flex-wrap: nowrap;
    /* 不换行 */
}

/* 按钮基础样式 - 统一一下 */
.btn-cloud,
.btn-theme,
.btn-dropdown,
.btn-upload,
.btn-share,
.btn-delete,
.btn-avatar,
.btn-button-state {
    border: none;
    border-radius: 2px;
    padding: 6px 12px;
    font-size: 13px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    font-weight: 500;
    transition: all 0.2s ease;
    height: 32px;
}

.btn-button-state {
    background-color: var(--hover-bg, #f3f4f6); 
    color: var(--text-secondary, #57606a);
    border: 1px solid var(--border-color, #d0d7de);
}

.btn-button-state.pressed {
    background-color: var(--accent-green, #2da44e);
    color: white;
    border-color: var(--accent-green, #2da44e);
}

.btn-button-state:hover {
    background-color: var(--bg-tertiary, #f6f8fa);
    border-color: var(--text-muted, #8c959f);
}

/* 主题切换按钮 - 放在第一个 */
.btn-theme {
    background-color: var(--bg-secondary, #f6f8fa);
    color: var(--text-primary, #24292f);
    border: 1px solid var(--border-color, #d0d7de);
}

.btn-theme:hover {
    background-color: var(--bg-tertiary, #f6f8fa);
    border-color: var(--text-muted, #8c959f);
}

/* 云按钮 - 就是个装饰性的 */
.btn-cloud {
    background: var(--bg-secondary, #f6f8fa);
    color: var(--text-primary, #24292f);
    padding: 6px;
    border-radius: 2px;
    width: 32px;
    height: 32px;
    border: 1px solid var(--border-color, #d0d7de);
}

a:hover { 
    text-decoration: none;
}

/* 下拉按钮 - 中性色 */
.btn-dropdown {
    background-color: var(--bg-secondary, #f6f8fa);
    color: var(--text-primary, #24292f);
    border: 1px solid var(--border-color, #d0d7de);
}

/* 上传按钮 - 主操作按钮，突出显示 */
.btn-upload {
    background: var(--accent-blue, #0969da);
    color: white;
    border: 1px solid rgba(9, 105, 218, 0.5);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

/* 分享按钮 - 深蓝色 */
.btn-share {
    background-color: var(--bg-secondary, #f6f8fa);
    color: var(--text-primary, #24292f);
    border: 1px solid var(--border-color, #d0d7de);
}

/* 删除按钮 - 红色警告色 */
.btn-delete {
  background-color: var(--danger-btn-bg, #212830);
  color: var(--danger-btn-text, #f85149);
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
}

.btn-delete:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, white);
  border-color: var(--danger-btn-hover-border, #f85149);
}

/* 删除按钮图标 - 继承按钮颜色 */
.btn-delete i,
.btn-delete svg {
  color: inherit;
}

/* 头像按钮 - 圆形 */
.btn-avatar {
    background-color: var(--bg-secondary, #f6f8fa);
    border: 1px solid var(--border-color, #d0d7de);
    border-radius: 2px;
    width: 32px;
    height: 32px;
    color: var(--text-primary, #24292f);
    padding: 0;
}

.btn-cloud:hover {
    background: var(--hover-bg, #f3f4f6);
    border-color: var(--text-muted, #8c959f);
}

.btn-dropdown:hover {
    background-color: var(--hover-bg, #f3f4f6);
    border-color: var(--text-muted, #8c959f);
}

/* 上传按钮 hover - 让它亮一点 */
.btn-upload:hover {
    background: var(--accent-blue-bright, #0550ae);
    border-color: rgba(9, 105, 218, 0.8);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
}

.btn-share:hover {
    background-color: var(--hover-bg, #f3f4f6);
    border-color: var(--text-muted, #8c959f);
}

.btn-avatar:hover {
    background-color: var(--selected-bg, #ddf4ff);
    border-color: var(--accent-blue, #0969da);
    color: var(--accent-blue, #0969da);
}

/* 图标统一样式 */
.btn-cloud i,
.btn-dropdown i,
.btn-upload i,
.btn-share i,
.btn-delete i,
.btn-avatar i,
.btn-theme i,
.btn-test i {
    font-size: 14px;
    /* 稍微减小图标大小以适应更窄的顶栏 */
    display: flex;
    align-items: center;
    justify-content: center;
}

/* 窗口控制按钮组 */
.window-controls {
    display: flex;
    align-items: center;
    gap: 0;
    margin-left: 16px;
    padding-left: 16px;
    border-left: 1px solid var(--border-color, #30363d);
}

/* 窗口控制按钮 */
.window-control-btn {
    width: 40px;
    height: 32px;
    border: none;
    background: transparent;
    color: var(--text-secondary, #57606a);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    font-size: 16px;
    border-radius: 2px;
}

.window-control-btn:hover {
    background-color: var(--hover-bg, #f3f4f6);
    color: var(--text-primary, #24292f);
}

.window-control-btn.close-btn:hover {
    background-color: var(--accent-red, #cf222e);
    color: white;
}

.window-control-btn i {
    font-size: 16px;
}

/* 按钮文字通用类 - 方便响应式隐藏 */
.btn-text {
    display: inline;
}

/* 响应式 - 小屏幕时按钮只显示图标 */
/* FIXME: 在小屏幕上图标按钮有点挤，可能需要调整 */
@media (max-width: 1024px) {
    .toolbar {
        padding: 0 16px;
        /* 内边距减小 */
    }

    .operation {
        gap: 8px;
        /* 间距减小 */
    }

    /* 隐藏按钮文字，只留图标 */
    .btn-text {
        display: none;
    }

    .btn-theme,
    .btn-dropdown,
    .btn-upload,
    .btn-share,
    .btn-delete {
        padding: 8px;
        width: 40px;
        /* 固定宽度 */
        justify-content: center;
    }

    /* TODO: 超小屏幕可能需要更多调整 */
}
</style>
