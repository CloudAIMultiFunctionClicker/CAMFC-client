import { defineStore } from 'pinia';
import { FileInfo } from '../../core';
import { AppService } from '../../application';

/**
 * 文件状态管理
 */
export const useFileStore = defineStore('file', {
  state: () => ({
    files: [] as FileInfo[],
    currentPath: '/',
    isLoading: false,
    error: null as string | null,
    uploadProgress: 0,
    downloadProgress: 0
  }),
  
  getters: {
    directories: (state) => {
      return state.files.filter(file => file.isDirectory);
    },
    regularFiles: (state) => {
      return state.files.filter(file => !file.isDirectory);
    },
    pathSegments: (state) => {
      const segments = state.currentPath.split('/').filter(segment => segment !== '');
      const paths = segments.map((segment, index) => {
        const path = '/' + segments.slice(0, index + 1).join('/');
        return { name: segment, path };
      });
      return [{ name: '根目录', path: '/' }, ...paths];
    }
  },
  
  actions: {
    async loadFiles(path: string = '/') {
      const appService = AppService.getInstance();
      try {
        this.isLoading = true;
        this.error = null;
        this.currentPath = path;
        this.files = await appService.listFiles(path);
      } catch (error) {
        this.error = '加载文件失败';
        console.error('Load files error:', error);
      } finally {
        this.isLoading = false;
      }
    },
    
    async createDirectory(name: string) {
      const appService = AppService.getInstance();
      try {
        this.isLoading = true;
        this.error = null;
        const path = this.currentPath === '/' ? `/${name}` : `${this.currentPath}/${name}`;
        await appService.createDirectory(path);
        await this.loadFiles(this.currentPath);
      } catch (error) {
        this.error = '创建目录失败';
        console.error('Create directory error:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },
    
    async deleteFile(path: string) {
      const appService = AppService.getInstance();
      try {
        this.isLoading = true;
        this.error = null;
        await appService.delete(path);
        await this.loadFiles(this.currentPath);
      } catch (error) {
        this.error = '删除文件失败';
        console.error('Delete file error:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },
    
    async renameFile(oldPath: string, newName: string) {
      const appService = AppService.getInstance();
      try {
        this.isLoading = true;
        this.error = null;
        const dirPath = oldPath.substring(0, oldPath.lastIndexOf('/'));
        const newPath = dirPath === '' ? `/${newName}` : `${dirPath}/${newName}`;
        await appService.rename(oldPath, newPath);
        await this.loadFiles(this.currentPath);
      } catch (error) {
        this.error = '重命名失败';
        console.error('Rename file error:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },
    
    async uploadFile(path: string, file: File) {
      const appService = AppService.getInstance();
      try {
        this.uploadProgress = 0;
        this.isLoading = true;
        this.error = null;
        
        await appService.upload(path, file, (progress) => {
          this.uploadProgress = progress;
        });
        
        await this.loadFiles(this.currentPath);
      } catch (error) {
        this.error = '上传文件失败';
        console.error('Upload file error:', error);
        throw error;
      } finally {
        this.isLoading = false;
        this.uploadProgress = 0;
      }
    },
    
    async downloadFile(path: string) {
      const appService = AppService.getInstance();
      try {
        this.downloadProgress = 0;
        this.isLoading = true;
        this.error = null;
        
        const blob = await appService.download(path, (progress) => {
          this.downloadProgress = progress;
        });
        
        // 创建下载链接
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = path.substring(path.lastIndexOf('/') + 1);
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
      } catch (error) {
        this.error = '下载文件失败';
        console.error('Download file error:', error);
        throw error;
      } finally {
        this.isLoading = false;
        this.downloadProgress = 0;
      }
    }
  }
});