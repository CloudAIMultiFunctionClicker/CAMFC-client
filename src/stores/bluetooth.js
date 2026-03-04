/**
 * CAMFC Client - 蓝牙状态管理store（重构版）
 * 
 * Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
 * Email: abc.cxh2009@foxmail.com
 *
 * Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
 * Email: 1220594170@qq.com
 *
 * Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
 * Email: admin@mc666.top
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * 蓝牙状态管理store（重构版）
 * 只管理蓝牙连接状态，不存储TOTP和设备ID
 * 
 * 重构说明：TOTP和设备ID改为直接从Rust命令获取，不再通过store缓存
 * 这样保证数据实时性，简化状态管理
 * 
 * TODO: 考虑是否还需要deviceInfo，可能也改为直接从Rust获取？
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'
import { emit } from '@tauri-apps/api/event'

export const useBluetoothStore = defineStore('bluetooth', () => {
  // 蓝牙状态：连接中、已连接、断开
  const bluetoothStatus = ref('disconnected') // disconnected/connecting/connected
  
  // 设备信息（可能还需要，先保留）
  const deviceInfo = ref(null)
  
  // 错误信息，有错误才显示
  const error = ref(null)
  
  // 简单状态判断
  const isConnected = () => bluetoothStatus.value === 'connected'
  const isConnecting = () => bluetoothStatus.value === 'connecting'
  
  // 更新状态
  const setStatus = async (status) => {
    bluetoothStatus.value = status
    if (status !== 'error') {
      error.value = null // 状态正常就清掉错误
    }
    
    // 通知悬浮窗连接状态变化
    try {
      const isConnectedStatus = status === 'connected'
      await emit('connection-status', isConnectedStatus)
      console.log('已发送连接状态事件到悬浮窗:', isConnectedStatus)
    } catch (e) {
      console.log('发送连接状态事件失败（非Tauri环境）:', e)
    }
  }
  
  const setDeviceInfo = (info) => {
    deviceInfo.value = info
  }
  
  const setError = (err) => {
    error.value = err
    bluetoothStatus.value = 'error'
    
    // 通知悬浮窗连接状态变化
    try {
      emit('connection-status', false)
    } catch (e) {
      console.log('发送连接状态事件失败（非Tauri环境）:', e)
    }
  }
  
  // 重置状态（断开连接时用）
  const reset = () => {
    bluetoothStatus.value = 'disconnected'
    deviceInfo.value = null
    error.value = null
    console.log('蓝牙状态已重置：断开连接')
    
    // 通知悬浮窗连接状态变化
    try {
      emit('connection-status', false)
    } catch (e) {
      console.log('发送连接状态事件失败（非Tauri环境）:', e)
    }
  }
  
  return {
    // 状态
    bluetoothStatus,
    deviceInfo,
    error,
    
    // 计算属性
    isConnected,
    isConnecting,
    
    // 方法
    setStatus,
    setDeviceInfo,
    setError,
    reset
  }
})
