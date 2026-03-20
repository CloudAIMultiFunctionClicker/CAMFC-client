import { BackendConfig, STORAGE_KEYS } from '../../core';
import { StorageService } from '../storage/storage_service';
import { invoke } from '@tauri-apps/api/core';

/**
 * 配置管理服务
 * 管理应用配置，包括后端配置等
 */
export class ConfigService {
  private static instance: ConfigService;
  private backendConfig: BackendConfig | null = null;

  private constructor() {}

  /**
   * 获取配置服务实例
   * @returns 配置服务实例
   */
  static getInstance(): ConfigService {
    if (!ConfigService.instance) {
      ConfigService.instance = new ConfigService();
    }
    return ConfigService.instance;
  }

  /**
   * 初始化配置
   */
  async init(): Promise<void> {
    try {
      // 先从本地存储加载
      const storedConfig = StorageService.get<BackendConfig | null>(STORAGE_KEYS.BACKEND_CONFIG, null);
      if (storedConfig) {
        this.backendConfig = storedConfig;
        return;
      }

      // 从后端获取配置
      const config = await invoke<BackendConfig>('get_backend_config');
      this.backendConfig = config;
      
      // 保存到本地存储
      StorageService.set(STORAGE_KEYS.BACKEND_CONFIG, config);
    } catch (error) {
      console.error('Config init error:', error);
      // 使用默认配置
      this.backendConfig = {
        baseUrl: 'http://localhost',
        port: 8080,
        fullUrl: 'http://localhost:8080'
      };
    }
  }

  /**
   * 获取后端配置
   * @returns 后端配置
   */
  getBackendConfig(): BackendConfig {
    if (!this.backendConfig) {
      // 使用默认配置
      return {
        baseUrl: 'http://localhost',
        port: 8080,
        fullUrl: 'http://localhost:8080'
      };
    }
    return this.backendConfig;
  }

  /**
   * 获取后端完整URL
   * @returns 后端完整URL
   */
  getBackendUrl(): string {
    return this.getBackendConfig().fullUrl;
  }

  /**
   * 更新后端配置
   * @param config 新配置
   */
  updateBackendConfig(config: BackendConfig): void {
    this.backendConfig = config;
    StorageService.set(STORAGE_KEYS.BACKEND_CONFIG, config);
  }

  /**
   * 重新加载配置
   */
  async reload(): Promise<void> {
    await this.init();
  }
}