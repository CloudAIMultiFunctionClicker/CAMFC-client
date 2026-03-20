/**
 * 存储服务
 * 封装localStorage，提供统一的存储管理
 */
export class StorageService {
  /**
   * 保存数据
   * @param key 键名
   * @param value 值
   */
  static set<T>(key: string, value: T): void {
    try {
      const serializedValue = JSON.stringify(value);
      localStorage.setItem(key, serializedValue);
    } catch (error) {
      console.error('Storage set error:', error);
    }
  }

  /**
   * 获取数据
   * @param key 键名
   * @param defaultValue 默认值
   * @returns 存储的值或默认值
   */
  static get<T>(key: string, defaultValue: T): T {
    try {
      const serializedValue = localStorage.getItem(key);
      if (serializedValue === null) {
        return defaultValue;
      }
      return JSON.parse(serializedValue) as T;
    } catch (error) {
      console.error('Storage get error:', error);
      return defaultValue;
    }
  }

  /**
   * 删除数据
   * @param key 键名
   */
  static remove(key: string): void {
    try {
      localStorage.removeItem(key);
    } catch (error) {
      console.error('Storage remove error:', error);
    }
  }

  /**
   * 清空所有数据
   */
  static clear(): void {
    try {
      localStorage.clear();
    } catch (error) {
      console.error('Storage clear error:', error);
    }
  }

  /**
   * 检查键是否存在
   * @param key 键名
   * @returns 是否存在
   */
  static has(key: string): boolean {
    try {
      return localStorage.getItem(key) !== null;
    } catch (error) {
      console.error('Storage has error:', error);
      return false;
    }
  }

  /**
   * 获取所有键
   * @returns 键名数组
   */
  static keys(): string[] {
    try {
      const keys: string[] = [];
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        if (key) {
          keys.push(key);
        }
      }
      return keys;
    } catch (error) {
      console.error('Storage keys error:', error);
      return [];
    }
  }
}

/**
 * 安全存储服务
 * 用于存储敏感信息
 */
export class SecureStorageService {
  /**
   * 保存敏感数据
   * @param key 键名
   * @param value 值
   */
  static set<T>(key: string, value: T): void {
    // 这里可以添加加密逻辑
    StorageService.set(key, value);
  }

  /**
   * 获取敏感数据
   * @param key 键名
   * @param defaultValue 默认值
   * @returns 存储的值或默认值
   */
  static get<T>(key: string, defaultValue: T): T {
    // 这里可以添加解密逻辑
    return StorageService.get(key, defaultValue);
  }

  /**
   * 删除敏感数据
   * @param key 键名
   */
  static remove(key: string): void {
    StorageService.remove(key);
  }
}