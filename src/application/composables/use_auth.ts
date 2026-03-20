import { ref, computed } from 'vue';
import { AppService } from '../services/app_service';
import { ERROR_MESSAGES } from '../../core';

/**
 * 认证组合式函数
 * 提供认证相关的状态和方法
 */
export function useAuth() {
  const appService = AppService.getInstance();
  const userInfo = ref<any>(null);
  const isLoggedIn = ref<boolean>(false);
  const isLoading = ref<boolean>(false);
  const error = ref<string | null>(null);

  /**
   * 登录
   * @param username 用户名
   * @param password 密码
   */
  const login = async (username: string, password: string) => {
    try {
      isLoading.value = true;
      error.value = null;
      await appService.login(username, password);
      isLoggedIn.value = true;
      await loadUserInfo();
    } catch (err) {
      error.value = ERROR_MESSAGES.AUTH_ERROR;
      console.error('Login error:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * 登出
   */
  const logout = async () => {
    try {
      isLoading.value = true;
      error.value = null;
      await appService.logout();
      isLoggedIn.value = false;
      userInfo.value = null;
    } catch (err) {
      console.error('Logout error:', err);
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * 加载用户信息
   */
  const loadUserInfo = async () => {
    try {
      userInfo.value = await appService.getUserInfo();
    } catch (err) {
      console.error('Load user info error:', err);
    }
  };

  /**
   * 修改密码
   * @param oldPassword 旧密码
   * @param newPassword 新密码
   */
  const changePassword = async (oldPassword: string, newPassword: string) => {
    try {
      isLoading.value = true;
      error.value = null;
      await appService.changePassword(oldPassword, newPassword);
    } catch (err) {
      error.value = ERROR_MESSAGES.AUTH_ERROR;
      console.error('Change password error:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  // 计算属性
  const hasUserInfo = computed(() => {
    return userInfo.value !== null;
  });

  return {
    userInfo,
    isLoggedIn,
    isLoading,
    error,
    hasUserInfo,
    login,
    logout,
    loadUserInfo,
    changePassword
  };
}