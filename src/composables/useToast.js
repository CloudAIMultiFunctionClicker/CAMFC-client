

import { ref } from 'vue'

const STYLE_ID = 'toast-styles'

const CONTAINER_ID = 'toast-container'

export function useToast() {
  const toasts = ref([])
  let toastId = 0

  const ensureGlobalStyles = () => {
    let style = document.getElementById(STYLE_ID)
    if (!style) {
      style = document.createElement('style')
      style.id = STYLE_ID
      document.head.appendChild(style)
    }
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
          top: 80px;
          right: 20px;
          z-index: 10000;
          display: flex;
          flex-direction: column;
          gap: 10px;
          pointer-events: none;
        }

        .vue-toast-item {
          background: var(--accent-blue, #3b82f6);
          padding: 12px 20px;
          border-radius: 2px;
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
      `
  }

  const ensureContainer = () => {
    let container = document.getElementById(CONTAINER_ID)
    if (!container) {
      container = document.createElement('div')
      container.id = CONTAINER_ID
      document.body.appendChild(container)
    }
    return container
  }

  const removeToast = (element, id) => {

    element.style.animation = 'toast-slide-out 0.3s ease forwards'

    setTimeout(() => {
      if (element.parentNode) {
        element.remove()
      }

      const index = toasts.value.findIndex(t => t.id === id)
      if (index > -1) {
        toasts.value.splice(index, 1)
      }

      repositionToasts()
    }, 300)
  }

  const repositionToasts = () => {
    const toastElements = document.querySelectorAll('.vue-toast-item')
    toastElements.forEach((el, index) => {
      const offsetTop = index * 60
      el.style.top = `${offsetTop}px`
    })
  }

const getTextColor = (bgColor) => {

    if (!bgColor || bgColor.startsWith('var(')) return 'white'

    const hex = bgColor.replace('#', '')
    if (hex.length < 6) return 'white'
    const r = parseInt(hex.substring(0, 2), 16)
    const g = parseInt(hex.substring(2, 4), 16)
    const b = parseInt(hex.substring(4, 6), 16)

    const brightness = (r * 299 + g * 587 + b * 114) / 1000
    return brightness > 150 ? '#1a1a1a' : 'white'
}

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
    toastEl.style.color = getTextColor(color)

    const offsetTop = toasts.value.length * 60
    toastEl.style.top = `${offsetTop}px`

    container.appendChild(toastEl)
    toasts.value.push(toastItem)

    toastEl.addEventListener('click', () => {
      removeToast(toastEl, id)
    })

    setTimeout(() => {
      if (toastEl.parentNode) {
        removeToast(toastEl, id)
      }
    }, 3000)

    return id
  }

  const close = (id) => {
    const element = document.querySelector(`[data-toast-id="${id}"]`)
    if (element) {
      removeToast(element, id)
    }
  }

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

let globalToast = null

export function showToast(text, color, duration) {
  if (!globalToast) {
    globalToast = useToast()
  }
  return globalToast.show({ text, color, duration })
}