

<script setup>
import { inject, ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

const theme = inject('theme')

const buttonPressed = ref(false)

const handleButtonState = (event) => {
  buttonPressed.value = event.detail.pressed
}

onMounted(() => {
  window.addEventListener('button-state', handleButtonState)
})

onUnmounted(() => {
  window.removeEventListener('button-state', handleButtonState)
})

const currentWindow = getCurrentWindow()

const isMaximized = ref(false)

const checkWindowState = async () => {
  try {
    isMaximized.value = await currentWindow.isMaximized()
  } catch (error) {
    console.error('检查窗口状态失败:', error)
  }
}

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

const minimizeWindow = async () => {
  try {
    console.log('正在最小化窗口...')
    await currentWindow.minimize()
    console.log('窗口最小化成功')
  } catch (error) {
    console.error('最小化窗口失败:', error)
  }
}

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

    isMaximized.value = !isMaximized.value
  } catch (error) {
    console.error('切换最大化失败:', error)
  }
}

const closeApp = async () => {
  try {
    await invoke('exit_app')
  } catch (error) {
    console.error('退出应用失败:', error)
  }
}

const startWindowDrag = async (event) => {

  if (event.button !== 0) return

  if (event.target.closest('button, a, .window-controls')) return

  try {
    await currentWindow.startDragging()
  } catch (error) {
    console.error('拖动窗口失败:', error)
  }
}
</script>

<template>

    <header class="header">
        <div class="toolbar" @mousedown="startWindowDrag">

            <h1>
                <span>CAMFC Cloud</span>

                <router-link to="/main">
                <button class="btn-cloud">

                    <i class="ri-cloud-line"></i>
                </button></router-link>
            </h1>

            <div class="operation">

                <button class="btn-theme" @click="theme?.toggleTheme">

                    <i class="ri-moon-line" v-if="theme?.isLightMode.value"></i>
                    <i class="ri-sun-line" v-else></i>

                    <span class="btn-text">{{ theme?.isLightMode.value ? '切换到暗色' : '切换到亮色' }}</span>
                </button>

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

.toolbar {
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    align-items: center;

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

.operation {
    display: flex;
    align-items: center;
    gap: 12px;

    flex-wrap: nowrap;

}

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

.btn-theme {
    background-color: var(--bg-secondary, #f6f8fa);
    color: var(--text-primary, #24292f);
    border: 1px solid var(--border-color, #d0d7de);
}

.btn-theme:hover {
    background-color: var(--bg-tertiary, #f6f8fa);
    border-color: var(--text-muted, #8c959f);
}

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

.btn-dropdown {
    background-color: var(--bg-secondary, #f6f8fa);
    color: var(--text-primary, #24292f);
    border: 1px solid var(--border-color, #d0d7de);
}

.btn-upload {
    background: var(--accent-blue, #0969da);
    color: white;
    border: 1px solid rgba(9, 105, 218, 0.5);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.btn-share {
    background-color: var(--bg-secondary, #f6f8fa);
    color: var(--text-primary, #24292f);
    border: 1px solid var(--border-color, #d0d7de);
}

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

.btn-delete i,
.btn-delete svg {
  color: inherit;
}

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

.btn-cloud i,
.btn-dropdown i,
.btn-upload i,
.btn-share i,
.btn-delete i,
.btn-avatar i,
.btn-theme i,
.btn-test i {
    font-size: 14px;

    display: flex;
    align-items: center;
    justify-content: center;
}

.window-controls {
    display: flex;
    align-items: center;
    gap: 0;
    margin-left: 16px;
    padding-left: 16px;
    border-left: 1px solid var(--border-color, #30363d);
}

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

.btn-text {
    display: inline;
}

@media (max-width: 1024px) {
    .toolbar {
        padding: 0 16px;

    }

    .operation {
        gap: 8px;

    }

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

        justify-content: center;
    }

}
</style>
