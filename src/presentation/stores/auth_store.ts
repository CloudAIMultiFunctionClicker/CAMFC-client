import { defineStore } from 'pinia';
import { AppService } from '../../application';

/**
 * 认证状态管理
 */
export const useAuthStore = defineStore('auth', {
  state: () => ({
    userInfo: null as any,
    isLoggedIn: false,
    isLoading: false,
    error: null as string | null
  }),
  
  getters: {
    hasUserInfo: (state) => {
      return state.userInfo !== null;
    },
    username: (state) => {
      return state.userInfo?.username || '';
    }
  },
  
  actions: {
    async login(username: string, password: string) {
      const appService = AppService.getInstance();
      try {
        this.isLoading = true;
        this.error = null;
        await appService.login(username, password);
        this.isLoggedIn = true;
        await this.loadUserInfo();
      } catch (error) {
        this.error = '登录失败';
        console.error('Login error:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },
    
    async logout() {
      const appService = AppService.getInstance();
      try {
        this.isLoading = true;
        this.error = null;
        await appService.logout();
        this.isLoggedIn = false;
        this.userInfo = null;
      } catch (error) {
        console.error('Logout error:', error);
      } finally {
        this.isLoading = false;
      }
    },
    
    async loadUserInfo() {
      const appService = AppService.getInstance();
      try {
        this.userInfo = await appService.getUserInfo();
      } catch (error) {
        console.error('Load user info error:', error);
      }
    },
    
    async changePassword(oldPassword: string, newPassword: string) {
      const appService = AppService.getInstance();
      try {
        this.isLoading = true;
        this.error = null;
        await appService.changePassword(oldPassword, newPassword);
      } catch (error) {
        this.error = '修改密码失败';
        console.error('Change password error:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    }
  }
});