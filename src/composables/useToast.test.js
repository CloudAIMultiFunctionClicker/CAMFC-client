import { describe, it, expect, beforeEach } from 'vitest'
import { showToast } from './useToast.js'

describe('useToast', () => {
  beforeEach(() => {
    const container = document.getElementById('toast-container')
    if (container) {
      container.remove()
    }
    const style = document.getElementById('toast-styles')
    if (style) {
      style.remove()
    }
  })

  it('showToast应该能接收字符串参数', () => {
    showToast('测试消息')
    const toast = document.querySelector('.vue-toast-item')
    expect(toast).toBeTruthy()
    expect(toast.textContent).toBe('测试消息')
  })

  it('showToast应该能接收分离的参数', () => {
    showToast('对象消息', '#ff0000')
    const toast = document.querySelector('.vue-toast-item')
    expect(toast).toBeTruthy()
    expect(toast.textContent).toBe('对象消息')
  })

  it('多个toast应该堆叠显示', () => {
    showToast('消息1')
    showToast('消息2')
    const toasts = document.querySelectorAll('.vue-toast-item')
    expect(toasts.length).toBe(2)
  })
})
