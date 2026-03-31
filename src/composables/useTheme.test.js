import { describe, it, expect, beforeEach, afterEach } from 'vitest'
import { isLightMode, toggleTheme } from './useTheme.js'

describe('useTheme', () => {
  beforeEach(() => {
    localStorage.removeItem('theme-preference')
  })

  afterEach(() => {
    localStorage.removeItem('theme-preference')
  })

  it('toggleTheme应该切换主题状态', async () => {
    const initialValue = isLightMode.value
    await toggleTheme()
    expect(isLightMode.value).toBe(!initialValue)
  })

  it('toggleTheme应该保存用户偏好到localStorage', async () => {
    await toggleTheme()
    const saved = localStorage.getItem('theme-preference')
    expect(saved).toBe(isLightMode.value ? 'light' : 'dark')
  })
})
