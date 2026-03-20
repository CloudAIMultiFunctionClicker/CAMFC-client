import { AuthInfo } from '../../core';
import { invoke } from '@tauri-apps/api/core';

/**
 * 认证服务
 * 处理认证相关的业务逻辑
 */
export class AuthService {
  private static instance: AuthService;
  private authInfo: AuthInfo | null = null;

  private constructor() {}

  /**
   * 获取认证服务实例
   * @returns 认证服务实例
   */
  static getInstance(): AuthService {
    if (!AuthService.instance) {
      AuthService.instance = new AuthService();
    }
    return AuthService.instance;
  }

  /**
   * 登录
   * @param username 用户名
   * @param password 密码
   */
  async login(username: string, password: string): Promise<void> {
    try {
      await invoke('login', { username, password });
    } catch (error) {
      throw error;
    }
  }

  /**
   * 登出
   */
  async logout(): Promise<void> {
    try {
      await invoke('logout');
      this.authInfo = null;
    } catch (error) {
      console.error('Logout error:', error);
    }
  }

  /**
   * 获取用户信息
   * @returns 用户信息
   */
  async getUserInfo(): Promise<any> {
    try {
      return await invoke('get_user_info');
    } catch (error) {
      throw error;
    }
  }

  /**
   * 修改密码
   * @param oldPassword 旧密码
   * @param newPassword 新密码
   */
  async changePassword(oldPassword: string, newPassword: string): Promise<void> {
    try {
      await invoke('change_password', { oldPassword, newPassword });
    } catch (error) {
      throw error;
    }
  }

  /**
   * 获取认证信息
   * @returns 认证信息
   */
  getAuthInfo(): AuthInfo | null {
    return this.authInfo;
  }

  /**
   * 设置认证信息
   * @param authInfo 认证信息
   */
  setAuthInfo(authInfo: AuthInfo): void {
    this.authInfo = authInfo;
  }

  /**
   * 检查是否已登录
   * @returns 是否已登录
   */
  isLoggedIn(): boolean {
    return this.authInfo !== null;
  }
}