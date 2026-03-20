import { DeviceInfo, ConnectionState } from '../../core';
import { invoke } from '@tauri-apps/api/core';

/**
 * 蓝牙服务
 * 处理蓝牙相关的业务逻辑
 */
export class BluetoothService {
  private static instance: BluetoothService;
  private connectionState: ConnectionState = ConnectionState.Disconnected;
  private currentDevice: DeviceInfo | null = null;

  private constructor() {}

  /**
   * 获取蓝牙服务实例
   * @returns 蓝牙服务实例
   */
  static getInstance(): BluetoothService {
    if (!BluetoothService.instance) {
      BluetoothService.instance = new BluetoothService();
    }
    return BluetoothService.instance;
  }

  /**
   * 扫描设备
   * @returns 设备列表
   */
  async scanDevices(): Promise<DeviceInfo[]> {
    try {
      this.connectionState = ConnectionState.Scanning;
      const devices = await invoke<DeviceInfo[]>('scan_cpen_devices');
      this.connectionState = ConnectionState.Disconnected;
      return devices;
    } catch (error) {
      this.connectionState = ConnectionState.Disconnected;
      throw error;
    }
  }

  /**
   * 连接设备
   * @param address 设备地址
   * @returns 设备信息
   */
  async connectDevice(address: string): Promise<DeviceInfo> {
    try {
      this.connectionState = ConnectionState.Connecting;
      const deviceInfo = await invoke<DeviceInfo>('connect_cpen_device', { address });
      this.currentDevice = deviceInfo;
      this.connectionState = ConnectionState.Connected;
      return deviceInfo;
    } catch (error) {
      this.connectionState = ConnectionState.Disconnected;
      this.currentDevice = null;
      throw error;
    }
  }

  /**
   * 断开连接
   */
  async disconnect(): Promise<void> {
    try {
      await invoke('disconnect');
      this.connectionState = ConnectionState.Disconnected;
      this.currentDevice = null;
    } catch (error) {
      console.error('Disconnect error:', error);
      this.connectionState = ConnectionState.Disconnected;
      this.currentDevice = null;
    }
  }

  /**
   * 获取TOTP
   * @returns TOTP字符串
   */
  async getTotp(): Promise<string> {
    try {
      return await invoke<string>('get_totp');
    } catch (error) {
      throw error;
    }
  }

  /**
   * 获取设备ID
   * @returns 设备ID
   */
  async getDeviceId(): Promise<string> {
    try {
      return await invoke<string>('get_device_id');
    } catch (error) {
      throw error;
    }
  }

  /**
   * 获取连接状态
   * @returns 连接状态
   */
  async getConnectionStatus(): Promise<string> {
    try {
      return await invoke<string>('get_connection_status');
    } catch (error) {
      console.error('Get connection status error:', error);
      return '未连接';
    }
  }

  /**
   * 检查是否已连接
   * @returns 是否已连接
   */
  async isConnected(): Promise<boolean> {
    try {
      return await invoke<boolean>('is_connected');
    } catch (error) {
      console.error('Is connected error:', error);
      return false;
    }
  }

  /**
   * 获取当前设备
   * @returns 当前设备信息
   */
  getCurrentDevice(): DeviceInfo | null {
    return this.currentDevice;
  }

  /**
   * 获取连接状态枚举
   * @returns 连接状态
   */
  getConnectionState(): ConnectionState {
    return this.connectionState;
  }
}