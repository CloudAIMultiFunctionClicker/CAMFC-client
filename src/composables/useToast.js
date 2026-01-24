/**
 * Toast 提示钩子函数
 * 每个 Toast 只存在1秒，超时后自动向上移出屏幕，后面的补位
 */

import { ref } from 'vue'

// 全局样式 ID
const STYLE_ID = 'toast-styles'
// 容器 ID
const CONTAINER_ID = 'toast-container'

export function useToast() {
  const toasts = ref([])
  let toastId = 0

  // 检查并添加全局样式
  const ensureGlobalStyles = () => {
    if (!document.getElementById(STYLE_ID)) {
      const style = document.createElement('style')
      style.id = STYLE_ID
      style.textContent = `
        @keyframes toast-slide-in {
          from {
            transform: translateX(120%) scale(0.95);
            opacity: 0;
          }
          to {
            transform: translateX(0) scale(1);
            opacity: 1;
          }
        }

        @keyframes toast-slide-out {
          from {
            transform: translateX(0) scale(1);
            opacity: 1;
          }
          to {
            transform: translateX(120%) scale(0.9);
            opacity: 0;
          }
        }

        #${CONTAINER_ID} {
          position: fixed;
          top: 20px;
          right: 20px;
          z-index: 10000;
          display: flex;
          flex-direction: column;
          gap: 10px;
          pointer-events: none;
        }

        .vue-toast-item {
          background: var(--accent-blue, #3b82f6);
          color: white;
          padding: 12px 20px;
          border-radius: 12px;
          box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15);
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
          font-size: 14px;
          font-weight: 500;
          max-width: 320px;
          word-break: break-word;
          pointer-events: auto;
          opacity: 0;
          cursor: pointer;
          transition: transform 0.2s ease, top 0.3s ease;
          animation: toast-slide-in 0.3s ease forwards;
        }

        .vue-toast-item:hover {
          transform: translateY(-2px);
        }

        .vue-toast-item {
          color: var(--text-primary, white);
        }
      `
      document.head.appendChild(style)
    }
  }

  // 检查并创建容器
  const ensureContainer = () => {
    let container = document.getElementById(CONTAINER_ID)
    if (!container) {
      container = document.createElement('div')
      container.id = CONTAINER_ID
      document.body.appendChild(container)
    }
    return container
  }

  /**
   * 移除指定 toast
   * @param {HTMLElement} element toast 元素
   * @param {number} id toast ID
   */
  const removeToast = (element, id) => {
    // 播放移出动画
    element.style.animation = 'toast-slide-out 0.3s ease forwards'
    
    // 等动画播放完毕后移除元素
    setTimeout(() => {
      if (element.parentNode) {
        element.remove()
      }
      
      // 从列表中移除
      const index = toasts.value.findIndex(t => t.id === id)
      if (index > -1) {
        toasts.value.splice(index, 1)
      }
      
      // 重新排列后续的 toast
      repositionToasts()
    }, 300)
  }

  /**
   * 重新排列所有 toast 的位置
   */
  const repositionToasts = () => {
    const toastElements = document.querySelectorAll('.vue-toast-item')
    toastElements.forEach((el, index) => {
      const offsetTop = index * 60
      el.style.top = `${offsetTop}px`
    })
  }

  /**
   * 显示一个 Toast
   * @param {string|object} options 配置项或直接传字符串
   * @returns {number} toast ID
   */
  const show = (options) => {
    const opts = typeof options === 'string' ? { text: options } : options

    ensureGlobalStyles()
    const container = ensureContainer()

    const id = ++toastId
    const toastItem = { ...opts, id }

    const toastEl = document.createElement('div')
    toastEl.className = 'vue-toast-item'
    toastEl.textContent = opts.text
    toastEl.dataset.toastId = id

    const color = opts.color ?? 'var(--accent-blue)'
    toastEl.style.background = color

    // 设置初始位置
    const offsetTop = toasts.value.length * 60
    toastEl.style.top = `${offsetTop}px`

    container.appendChild(toastEl)
    toasts.value.push(toastItem)

    // 点击关闭
    toastEl.addEventListener('click', () => {
      removeToast(toastEl, id)
    })

    // 1秒后自动移除
    setTimeout(() => {
      if (toastEl.parentNode) {
        removeToast(toastEl, id)
      }
    }, 3000)

    return id
  }

  /**
   * 手动关闭指定 ID 的 toast
   * @param {number} id toast ID
   */
  const close = (id) => {
    const element = document.querySelector(`[data-toast-id="${id}"]`)
    if (element) {
      removeToast(element, id)
    }
  }

  /**
   * 关闭所有 toast
   */
  const closeAll = () => {
    const elements = document.querySelectorAll('.vue-toast-item')
    elements.forEach(el => {
      const element = el
      element.style.animation = 'toast-slide-out 0.2s ease forwards'
      setTimeout(() => element.remove(), 200)
    })
    toasts.value = []
  }

  return {
    show,
    close,
    closeAll,
    toasts
  }
}

// 兼容旧版调用方式
let globalToast = null

export function showToast(text, color, duration) {
  if (!globalToast) {
    globalToast = useToast()
  }
  return globalToast.show({ text, color, duration })
}