import { BluetoothService } from '../../domain/bluetooth/bluetooth_service';
import { AuthService } from '../../domain/auth/auth_service';
import { FileService } from '../../domain/file/file_service';
import { ConfigService } from '../../infrastructure';
import { DeviceInfo, ConnectionState, FileInfo } from '../../core';

/**
 * 应用服务
 * 整合各个领域服务，提供统一的业务逻辑入口
 */
export class AppService {
  private static instance: AppService;
  private bluetoothService: BluetoothService;
  private authService: AuthService;
  private fileService: FileService;
  private configService: ConfigService;

  private constructor() {
    this.bluetoothService = BluetoothService.getInstance();
    this.authService = AuthService.getInstance();
    this.fileService = FileService.getInstance();
    this.configService = ConfigService.getInstance();
  }

  /**
   * 获取应用服务实例
   * @returns 应用服务实例
   */
  static getInstance(): AppService {
    if (!AppService.instance) {
      AppService.instance = new AppService();
    }
    return AppService.instance;
  }

  /**
   * 初始化应用
   */
  async init(): Promise<void> {
    // 初始化配置
    await this.configService.init();
    
    // 检查蓝牙连接状态
    const isConnected = await this.bluetoothService.isConnected();
    if (isConnected) {
      // 尝试获取当前设备信息
      try {
        const status = await this.bluetoothService.getConnectionStatus();
        console.log('Bluetooth status:', status);
      } catch (error) {
        console.error('Check connection status error:', error);
      }
    }
  }

  // 蓝牙相关方法

  /**
   * 扫描设备
   * @returns 设备列表
   */
  async scanDevices(): Promise<DeviceInfo[]> {
    return this.bluetoothService.scanDevices();
  }

  /**
   * 连接设备
   * @param address 设备地址
   * @returns 设备信息
   */
  async connectDevice(address: string): Promise<DeviceInfo> {
    return this.bluetoothService.connectDevice(address);
  }

  /**
   * 断开连接
   */
  async disconnect(): Promise<void> {
    return this.bluetoothService.disconnect();
  }

  /**
   * 获取TOTP
   * @returns TOTP字符串
   */
  async getTotp(): Promise<string> {
    return this.bluetoothService.getTotp();
  }

  /**
   * 获取设备ID
   * @returns 设备ID
   */
  async getDeviceId(): Promise<string> {
    return this.bluetoothService.getDeviceId();
  }

  /**
   * 获取连接状态
   * @returns 连接状态
   */
  async getConnectionStatus(): Promise<string> {
    return this.bluetoothService.getConnectionStatus();
  }

  /**
   * 检查是否已连接
   * @returns 是否已连接
   */
  async isConnected(): Promise<boolean> {
    return this.bluetoothService.isConnected();
  }

  /**
   * 获取当前设备
   * @returns 当前设备信息
   */
  getCurrentDevice(): DeviceInfo | null {
    return this.bluetoothService.getCurrentDevice();
  }

  /**
   * 获取连接状态枚举
   * @returns 连接状态
   */
  getConnectionState(): ConnectionState {
    return this.bluetoothService.getConnectionState();
  }

  // 认证相关方法

  /**
   * 登录
   * @param username 用户名
   * @param password 密码
   */
  async login(username: string, password: string): Promise<void> {
    return this.authService.login(username, password);
  }

  /**
   * 登出
   */
  async logout(): Promise<void> {
    return this.authService.logout();
  }

  /**
   * 获取用户信息
   * @returns 用户信息
   */
  async getUserInfo(): Promise<any> {
    return this.authService.getUserInfo();
  }

  /**
   * 修改密码
   * @param oldPassword 旧密码
   * @param newPassword 新密码
   */
  async changePassword(oldPassword: string, newPassword: string): Promise<void> {
    return this.authService.changePassword(oldPassword, newPassword);
  }

  /**
   * 检查是否已登录
   * @returns 是否已登录
   */
  isLoggedIn(): boolean {
    return this.authService.isLoggedIn();
  }

  // 文件相关方法

  /**
   * 列出文件
   * @param path 路径
   * @returns 文件列表
   */
  async listFiles(path: string = '/'): Promise<FileInfo[]> {
    return this.fileService.listFiles(path);
  }

  /**
   * 创建目录
   * @param path 路径
   */
  async createDirectory(path: string): Promise<void> {
    return this.fileService.createDirectory(path);
  }

  /**
   * 删除文件/目录
   * @param path 路径
   */
  async delete(path: string): Promise<void> {
    return this.fileService.delete(path);
  }

  /**
   * 重命名
   * @param oldPath 旧路径
   * @param newPath 新路径
   */
  async rename(oldPath: string, newPath: string): Promise<void> {
    return this.fileService.rename(oldPath, newPath);
  }

  /**
   * 移动
   * @param fromPath 源路径
   * @param toPath 目标路径
   */
  async move(fromPath: string, toPath: string): Promise<void> {
    return this.fileService.move(fromPath, toPath);
  }

  /**
   * 复制
   * @param fromPath 源路径
   * @param toPath 目标路径
   */
  async copy(fromPath: string, toPath: string): Promise<void> {
    return this.fileService.copy(fromPath, toPath);
  }

  /**
   * 下载文件
   * @param path 路径
   * @param onProgress 进度回调
   * @returns 文件数据
   */
  async download(path: string, onProgress?: (progress: number) => void): Promise<Blob> {
    return this.fileService.download(path, onProgress);
  }

  /**
   * 上传文件
   * @param path 路径
   * @param file 文件
   * @param onProgress 进度回调
   */
  async upload(path: string, file: File, onProgress?: (progress: number) => void): Promise<void> {
    return this.fileService.upload(path, file, onProgress);
  }

  // 配置相关方法

  /**
   * 获取后端URL
   * @returns 后端URL
   */
  getBackendUrl(): string {
    return this.configService.getBackendUrl();
  }

  /**
   * 重新加载配置
   */
  async reloadConfig(): Promise<void> {
    return this.configService.reload();
  }
}