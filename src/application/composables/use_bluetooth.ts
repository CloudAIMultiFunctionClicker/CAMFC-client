import { ref, computed, onMounted } from 'vue';
import { AppService } from '../services/app_service';
import { ConnectionState, DeviceInfo, ERROR_MESSAGES } from '../../core';

/**
 * 蓝牙组合式函数
 * 提供蓝牙相关的状态和方法
 */
export function useBluetooth() {
  const appService = AppService.getInstance();
  const devices = ref<DeviceInfo[]>([]);
  const currentDevice = ref<DeviceInfo | null>(null);
  const connectionState = ref<ConnectionState>(ConnectionState.Disconnected);
  const connectionStatus = ref<string>('未连接');
  const isScanning = ref<boolean>(false);
  const error = ref<string | null>(null);

  /**
   * 扫描设备
   */
  const scanDevices = async () => {
    try {
      isScanning.value = true;
      error.value = null;
      devices.value = await appService.scanDevices();
    } catch (err) {
      error.value = ERROR_MESSAGES.NO_DEVICE_FOUND;
      console.error('Scan devices error:', err);
    } finally {
      isScanning.value = false;
    }
  };

  /**
   * 连接设备
   * @param address 设备地址
   */
  const connectDevice = async (address: string) => {
    try {
      error.value = null;
      connectionState.value = ConnectionState.Connecting;
      const device = await appService.connectDevice(address);
      currentDevice.value = device;
      connectionState.value = ConnectionState.Connected;
      connectionStatus.value = await appService.getConnectionStatus();
    } catch (err) {
      error.value = ERROR_MESSAGES.CONNECTION_FAILED;
      connectionState.value = ConnectionState.Disconnected;
      console.error('Connect device error:', err);
    }
  };

  /**
   * 断开连接
   */
  const disconnect = async () => {
    try {
      error.value = null;
      await appService.disconnect();
      currentDevice.value = null;
      connectionState.value = ConnectionState.Disconnected;
      connectionStatus.value = '未连接';
    } catch (err) {
      console.error('Disconnect error:', err);
    }
  };

  /**
   * 获取TOTP
   * @returns TOTP字符串
   */
  const getTotp = async (): Promise<string> => {
    try {
      error.value = null;
      return await appService.getTotp();
    } catch (err) {
      error.value = ERROR_MESSAGES.TOTP_ERROR;
      console.error('Get TOTP error:', err);
      throw err;
    }
  };

  /**
   * 获取设备ID
   * @returns 设备ID
   */
  const getDeviceId = async (): Promise<string> => {
    try {
      error.value = null;
      return await appService.getDeviceId();
    } catch (err) {
      error.value = ERROR_MESSAGES.TOTP_ERROR;
      console.error('Get device ID error:', err);
      throw err;
    }
  };

  /**
   * 检查连接状态
   */
  const checkConnection = async () => {
    try {
      const isConnected = await appService.isConnected();
      if (isConnected) {
        connectionState.value = ConnectionState.Connected;
        connectionStatus.value = await appService.getConnectionStatus();
        currentDevice.value = appService.getCurrentDevice();
      } else {
        connectionState.value = ConnectionState.Disconnected;
        connectionStatus.value = '未连接';
        currentDevice.value = null;
      }
    } catch (err) {
      console.error('Check connection error:', err);
    }
  };

  // 计算属性
  const isConnected = computed(() => {
    return connectionState.value === ConnectionState.Connected;
  });

  const isConnecting = computed(() => {
    return connectionState.value === ConnectionState.Connecting;
  });

  // 生命周期
  onMounted(() => {
    checkConnection();
  });

  return {
    devices,
    currentDevice,
    connectionState,
    connectionStatus,
    isScanning,
    error,
    isConnected,
    isConnecting,
    scanDevices,
    connectDevice,
    disconnect,
    getTotp,
    getDeviceId,
    checkConnection
  };
}