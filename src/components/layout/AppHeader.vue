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
    /* 防止被压缩 */
    border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
    /* 使用主题边框色 */
    background: var(--bg-header, linear-gradient(135deg, #0f172a 0%, #1e293b 100%));
    /* 使用主题头部背景 */
    position: relative;
    z-index: 1000;
    /* 确保在最上面 */
    transition: background 0.3s ease, border-color 0.3s ease;
    /* 主题切换过渡效果 */
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
    /* flex gap 真好用 */
    color: var(--text-primary, #f8fafc);
    /* 使用主题主要文字色 */
    font-size: 1.2rem;
    /* 减小字体大小 */
    font-weight: 450;
    /* 改成中等粗细 */
    letter-spacing: -0.025em;
    /* 字距收紧一点感觉更现代？ */
    transition: color 0.3s ease;
    /* 文字颜色过渡 */
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
.btn-test,
.btn-button-state {
    border: none;
    border-radius: 8px;
    /* 圆角大一点现代感强 */
    padding: 6px 12px;
    /* 减小 padding 以适应更窄的顶栏 */
    font-size: 13px;
    /* 稍微减小字体 */
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    /* 图标和文字的间距 */
    font-weight: 500;
    transition: all 0.2s ease;
    /* 过渡效果，hover 用 */
    height: 32px;
    /* 统一高度，从 40px 减小到 32px */
}

/* 按钮状态指示器 */
.btn-button-state {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.08)); 
    color: var(--text-secondary, #cbd5e1);
    border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.btn-button-state.pressed {
    background-color: #10b981;
    color: white;
    border-color: #10b981;
}

.btn-button-state:hover {
    background-color: var(--accent-blue, #3b82f6);
    color: white;
    border-color: var(--accent-blue, #3b82f6);
}

/* 主题切换按钮 - 放在第一个 */
.btn-theme {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.08)); 
    color: var(--text-secondary, #cbd5e1);
    border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.btn-theme:hover {
    background-color: var(--accent-blue, #3b82f6);
    color: white;
    border-color: var(--accent-blue, #3b82f6);
}

/* 云按钮 - 就是个装饰性的 */
.btn-cloud {
    background: var(--hover-bg, rgba(255, 255, 255, 0.08));
    /* 使用主题 hover 背景色 */
    color: var(--text-muted, #94a3b8);
    /* 使用主题次要文字色 */
    padding: 6px;
    border-radius: 50%;
    /* 圆形 */
    width: 32px;
    height: 32px;
}

a {
   text-decoration: none;
}
a:hover { 
    text-decoration: none;
}

/* 下拉按钮 - 中性色 */
.btn-dropdown {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
    /* 使用主题hover背景色 */
    color: var(--text-secondary, #cbd5e1);
    /* 使用主题次要文字色 */
    border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

/* 上传按钮 - 主操作按钮，突出显示 */
.btn-upload {
    background: linear-gradient(135deg, var(--accent-blue, #3b82f6) 0%, #1d4ed8 100%);
    /* 使用主题蓝色 */
    color: white;
    border: none;
    box-shadow: 0 2px 10px rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
    /* 使用主题蓝色发光 */
}

/* 分享按钮 - 深蓝色 */
.btn-share {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.2);
    /* 使用主题蓝色，半透明 */
    color: white;
    border: 1px solid rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
}

/* 删除按钮 - 红色警告色 */
.btn-delete {
    background-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.8);
    /* 使用主题红色，半透明 */
    color: white;
    border: 1px solid rgba(var(--accent-red-rgb, 220, 53, 69), 0.3);
}

/* 头像按钮 - 圆形 */
.btn-avatar {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.1);
    border: 2px solid rgba(var(--accent-blue-rgb, 59, 130, 246), 0.5);
    /* 蓝色边框 */
    border-radius: 50%;
    width: 32px;
    height: 32px;
    color: var(--accent-blue, #3b82f6);
    /* 蓝色图标 */
    padding: 0;
}

/* ====== HOVER 效果 ====== */

.btn-cloud:hover {
    background: var(--accent-blue, #3b82f6);
    /* hover时用主题蓝色 */
    color: white;
}

.btn-dropdown:hover {
    background-color: var(--accent-blue, #3b82f6);
    /* hover时用主题蓝色 */
    color: white;
    border-color: var(--accent-blue, #3b82f6);
}

/* 上传按钮hover - 让它亮一点 */
.btn-upload:hover {
    background: linear-gradient(135deg, #4a94ff 0%, #2563eb 100%);
    box-shadow: 0 4px 15px rgba(var(--accent-blue-rgb, 59, 130, 246), 0.4);
    /* hover时阴影强一点 */
}

.btn-share:hover {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
    /* hover时更不透明 */
    border-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.5);
}

.btn-delete:hover {
    background-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.95);
    border-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.5);
}

.btn-avatar:hover {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.2);
    border-color: var(--accent-blue, #3b82f6);
    color: #60a5fa;
    /* 亮一点的蓝 */
}



.btn-test:hover {
    background-color: #16a34a;
    border-color: #16a34a;
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
    border-left: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

/* 窗口控制按钮 */
.window-control-btn {
    width: 40px;
    /* 减小宽度以适应更窄的顶栏 */
    height: 32px;
    /* 减小高度 */
    border: none;
    background: transparent;
    color: var(--text-secondary, #cbd5e1);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    font-size: 16px;
    /* 稍微减小图标大小 */
    border-radius: 8px;
}

.window-control-btn:hover {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
    color: var(--text-primary, #f8fafc);
}

.window-control-btn.close-btn:hover {
    background-color: #ef4444;
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
