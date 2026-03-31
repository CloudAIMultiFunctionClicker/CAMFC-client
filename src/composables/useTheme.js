import { ref, watch } from 'vue'

const getInitialTheme = () => {
  const savedTheme = localStorage.getItem('theme-preference')
  if (savedTheme === 'light' || savedTheme === 'dark') {
    return savedTheme === 'light'
  }
  const prefersLight = window.matchMedia('(prefers-color-scheme: light)').matches
  return prefersLight || false
}

const isLightMode = ref(getInitialTheme())

const updateBodyClass = () => {
  if (isLightMode.value) {
    document.body.classList.add('light-mode')
  } else {
    document.body.classList.remove('light-mode')
  }
}

const toggleTheme = async () => {
  isLightMode.value = !isLightMode.value
  updateBodyClass()
  localStorage.setItem('theme-preference', isLightMode.value ? 'light' : 'dark')

  try {
    const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
    const floatWindow = await WebviewWindow.getByLabel('float')
    if (floatWindow) {
      await floatWindow.emit('theme-changed', isLightMode.value ? 'light' : 'dark')
    }
  } catch (e) {
    console.log('发送主题变化事件失败:', e)
  }
}

const initTheme = () => {
  updateBodyClass()

  const lightMediaQuery = window.matchMedia('(prefers-color-scheme: light)')
  const handleSystemThemeChange = (e) => {
    const hasUserPreference = localStorage.getItem('theme-preference') !== null
    if (!hasUserPreference) {
      isLightMode.value = e.matches
      updateBodyClass()
    }
  }
  lightMediaQuery.addEventListener('change', handleSystemThemeChange)

  return () => {
    lightMediaQuery.removeEventListener('change', handleSystemThemeChange)
  }
}

export function useTheme() {
  return {
    isLightMode,
    toggleTheme,
    initTheme
  }
}

export { isLightMode, toggleTheme }
