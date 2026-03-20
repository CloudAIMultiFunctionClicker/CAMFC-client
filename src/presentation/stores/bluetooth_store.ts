import { defineStore } from 'pinia';
import { ConnectionState, DeviceInfo } from '../../core';
import { AppService } from '../../application';

/**
 * 蓝牙状态管理
 */
export const useBluetoothStore = defineStore('bluetooth', {
  state: () => ({
    devices: [] as DeviceInfo[],
    currentDevice: null as DeviceInfo | null,
    connectionState: ConnectionState.Disconnected,
    connectionStatus: '未连接',
    isScanning: false,
    error: null as string | null
  }),
  
  getters: {
    isConnected: (state) => {
      return state.connectionState === ConnectionState.Connected;
    },
    isConnecting: (state) => {
      return state.connectionState === ConnectionState.Connecting;
    },
    isDisconnected: (state) => {
      return state.connectionState === ConnectionState.Disconnected;
    }
  },
  
  actions: {
    async scanDevices() {
      const appService = AppService.getInstance();
      try {
        this.isScanning = true;
        this.error = null;
        this.devices = await appService.scanDevices();
      } catch (error) {
        this.error = '扫描设备失败';
        console.error('Scan devices error:', error);
      } finally {
        this.isScanning = false;
      }
    },
    
    async connectDevice(address: string) {
      const appService = AppService.getInstance();
      try {
        this.error = null;
        this.connectionState = ConnectionState.Connecting;
        const device = await appService.connectDevice(address);
        this.currentDevice = device;
        this.connectionState = ConnectionState.Connected;
        this.connectionStatus = await appService.getConnectionStatus();
      } catch (error) {
        this.error = '连接设备失败';
        this.connectionState = ConnectionState.Disconnected;
        console.error('Connect device error:', error);
      }
    },
    
    async disconnect() {
      const appService = AppService.getInstance();
      try {
        this.error = null;
        await appService.disconnect();
        this.currentDevice = null;
        this.connectionState = ConnectionState.Disconnected;
        this.connectionStatus = '未连接';
      } catch (error) {
        console.error('Disconnect error:', error);
      }
    },
    
    async getTotp() {
      const appService = AppService.getInstance();
      try {
        this.error = null;
        return await appService.getTotp();
      } catch (error) {
        this.error = '获取TOTP失败';
        console.error('Get TOTP error:', error);
        throw error;
      }
    },
    
    async getDeviceId() {
      const appService = AppService.getInstance();
      try {
        this.error = null;
        return await appService.getDeviceId();
      } catch (error) {
        this.error = '获取设备ID失败';
        console.error('Get device ID error:', error);
        throw error;
      }
    },
    
    async checkConnection() {
      const appService = AppService.getInstance();
      try {
        const isConnected = await appService.isConnected();
        if (isConnected) {
          this.connectionState = ConnectionState.Connected;
          this.connectionStatus = await appService.getConnectionStatus();
          this.currentDevice = appService.getCurrentDevice();
        } else {
          this.connectionState = ConnectionState.Disconnected;
          this.connectionStatus = '未连接';
          this.currentDevice = null;
        }
      } catch (error) {
        console.error('Check connection error:', error);
      }
    }
  }
});