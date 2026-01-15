/**
 * Toast 提示钩子函数
 * 从原来的 showToast.js 改造而来，更「Vue」的风格
 * 
 * TODO: 可以加个队列功能，防止多个toast同时出现时重叠
 * FIXME: 动画样式直接写死在函数里，其实应该抽到全局CSS里，但为了方便先这样
 * 
 */

import { ref } from 'vue'

// 全局样式ID，防止重复添加
const STYLE_ID = 'toast-styles'
// 容器ID
const CONTAINER_ID = 'toast-container'

export function useToast() {
  // 用ref管理toast列表（虽然现在还是操作DOM，但保留这个结构方便以后改成虚拟渲染）
  const toasts = ref([])
  // ID计数器
  let toastId = 0
  
  // 检查并添加全局样式
  const ensureGlobalStyles = () => {
    if (!document.getElementById(STYLE_ID)) {
      const style = document.createElement('style')
      style.id = STYLE_ID
      // 这里用CSS变量了！之前是硬编码的颜色，现在用项目定义的颜色变量
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
        @keyframes toast-fade-out {
          from {
            opacity: 1;
            transform: translateX(0) scale(1);
          }
          to {
            opacity: 0;
            transform: translateX(120%) scale(0.9);
          }
        }
        
        /* 容器样式 - 用CSS变量控制位置和间距 */
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
        
        /* Toast 卡片样式 */
        .vue-toast-item {
          /* 背景色用传进来的，如果没传就用项目的主色调 */
          background: var(--accent-blue, #3b82f6); /* 默认值 */
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
          transition: transform 0.2s ease;
        }
        
        .vue-toast-item:hover {
          transform: translateY(-2px);
        }
        
        /* 暗色/亮色模式适配 - 文字颜色用项目变量 */
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
   * 移除指定toast
   * @param {HTMLElement} element toast元素
   * @param {number} id toast ID
   */
  const removeToast = (element, id) => {
    // 播放消失动画
    element.style.animation = 'toast-fade-out 0.3s ease forwards'
    
    // 等动画播完再移除
    setTimeout(() => {
      if (element.parentNode) {
        element.remove()
      }
      
      // 从列表中移除
      const index = toasts.value.findIndex(t => t.id === id)
      if (index > -1) {
        toasts.value.splice(index, 1)
      }
    }, 300)
  }
  
  /**
   * 显示一个 Toast
   * @param {string|object} options 配置项或直接传字符串
   * @returns {number} toast ID
   */
  const show = (options) => {
    // 参数重载：允许直接传字符串
    const opts = typeof options === 'string' 
      ? { text: options }
      : options
    
    // 确保样式和容器存在
    ensureGlobalStyles()
    const container = ensureContainer()
    
    // 生成唯一ID
    const id = ++toastId
    const toastItem = { ...opts, id }
    
    // 创建toast元素
    const toastEl = document.createElement('div')
    toastEl.className = 'vue-toast-item'
    toastEl.textContent = opts.text
    
    // 设置动画 - 用CSS变量控制动画时长
    const duration = opts.duration ?? 3000
    const color = opts.color ?? 'var(--accent-blue)' // 默认用CSS变量
    
    toastEl.style.background = color
    toastEl.style.animation = `toast-slide-in 0.3s ease forwards, 
                             toast-fade-out 0.4s ease forwards ${duration > 0 ? duration : '999999'}ms`
    
    // 添加到容器
    container.appendChild(toastEl)
    
    // 添加到响应式列表（虽然现在还是操作DOM，但保留这个结构）
    toasts.value.push(toastItem)
    
    // 自动消失（如果设置了duration）
    if (duration > 0) {
      setTimeout(() => {
        if (toastEl.parentNode) {
          removeToast(toastEl, id)
        }
      }, duration)
    }
    
    // 点击关闭 - 这个功能挺好用的
    toastEl.addEventListener('click', () => {
      removeToast(toastEl, id)
    })
    
    return id
  }
  
  /**
   * 手动关闭指定ID的toast
   * @param {number} id toast ID
   */
  const close = (id) => {
    const element = document.querySelector(`[data-toast-id="${id}"]`)
    if (element) {
      removeToast(element, id)
    }
  }
  
  /**
   * 关闭所有toast
   */
  const closeAll = () => {
    const elements = document.querySelectorAll('.vue-toast-item')
    elements.forEach(el => {
      const element = el
      element.style.animation = 'toast-fade-out 0.2s ease forwards'
      setTimeout(() => element.remove(), 200)
    })
    toasts.value = []
  }
  
  // 组件卸载时清理所有toast？这个看需求，先不加
  
  return {
    show,
    close,
    closeAll,
    toasts // 暴露toasts列表，虽然现在用处不大，但万一有组件想监听呢
  }
}

// 为了兼容原来的用法，导出一个默认的useToast实例
let globalToast = null

/**
 * 全局toast函数，类似原来的showToast
 * 这样原来调用showToast的地方可以改成import { showToast } from '@/composables/useToast'
 * @param {string} text 显示的文字
 * @param {string} [color] 背景色
 * @param {number} [duration] 多久自动消失（毫秒）
 * @returns {number} toast ID
 */
export function showToast(text, color, duration) {
  if (!globalToast) {
    // 这里其实有点问题：useToast应该在一个Vue组件内调用
    // 但为了兼容性，先这样实现
    // 如果出现bug，可能需要改成在App.vue里初始化
    globalToast = useToast()
  }
  
  return globalToast.show({ text, color, duration })
}
