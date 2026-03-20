import { ref, computed } from 'vue';
import { AppService } from '../services/app_service';
import { FileInfo, ERROR_MESSAGES } from '../../core';

/**
 * 文件组合式函数
 * 提供文件相关的状态和方法
 */
export function useFile() {
  const appService = AppService.getInstance();
  const files = ref<FileInfo[]>([]);
  const currentPath = ref<string>('/');
  const isLoading = ref<boolean>(false);
  const error = ref<string | null>(null);
  const uploadProgress = ref<number>(0);
  const downloadProgress = ref<number>(0);

  /**
   * 加载文件列表
   * @param path 路径
   */
  const loadFiles = async (path: string = '/') => {
    try {
      isLoading.value = true;
      error.value = null;
      currentPath.value = path;
      files.value = await appService.listFiles(path);
    } catch (err) {
      error.value = ERROR_MESSAGES.FILE_ERROR;
      console.error('Load files error:', err);
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * 创建目录
   * @param name 目录名称
   */
  const createDirectory = async (name: string) => {
    try {
      isLoading.value = true;
      error.value = null;
      const path = currentPath.value === '/' ? `/${name}` : `${currentPath.value}/${name}`;
      await appService.createDirectory(path);
      await loadFiles(currentPath.value);
    } catch (err) {
      error.value = ERROR_MESSAGES.FILE_ERROR;
      console.error('Create directory error:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * 删除文件/目录
   * @param path 路径
   */
  const deleteFile = async (path: string) => {
    try {
      isLoading.value = true;
      error.value = null;
      await appService.delete(path);
      await loadFiles(currentPath.value);
    } catch (err) {
      error.value = ERROR_MESSAGES.FILE_ERROR;
      console.error('Delete file error:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * 重命名
   * @param oldPath 旧路径
   * @param newName 新名称
   */
  const renameFile = async (oldPath: string, newName: string) => {
    try {
      isLoading.value = true;
      error.value = null;
      const dirPath = oldPath.substring(0, oldPath.lastIndexOf('/'));
      const newPath = dirPath === '' ? `/${newName}` : `${dirPath}/${newName}`;
      await appService.rename(oldPath, newPath);
      await loadFiles(currentPath.value);
    } catch (err) {
      error.value = ERROR_MESSAGES.FILE_ERROR;
      console.error('Rename file error:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * 上传文件
   * @param path 目标路径
   * @param file 文件
   */
  const uploadFile = async (path: string, file: File) => {
    try {
      uploadProgress.value = 0;
      isLoading.value = true;
      error.value = null;
      
      await appService.upload(path, file, (progress) => {
        uploadProgress.value = progress;
      });
      
      await loadFiles(currentPath.value);
    } catch (err) {
      error.value = ERROR_MESSAGES.FILE_ERROR;
      console.error('Upload file error:', err);
      throw err;
    } finally {
      isLoading.value = false;
      uploadProgress.value = 0;
    }
  };

  /**
   * 下载文件
   * @param path 文件路径
   */
  const downloadFile = async (path: string) => {
    try {
      downloadProgress.value = 0;
      isLoading.value = true;
      error.value = null;
      
      const blob = await appService.download(path, (progress) => {
        downloadProgress.value = progress;
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
    } catch (err) {
      error.value = ERROR_MESSAGES.FILE_ERROR;
      console.error('Download file error:', err);
      throw err;
    } finally {
      isLoading.value = false;
      downloadProgress.value = 0;
    }
  };

  // 计算属性
  const directories = computed(() => {
    return files.value.filter(file => file.isDirectory);
  });

  const regularFiles = computed(() => {
    return files.value.filter(file => !file.isDirectory);
  });

  const pathSegments = computed(() => {
    const segments = currentPath.value.split('/').filter(segment => segment !== '');
    const paths = segments.map((segment, index) => {
      const path = '/' + segments.slice(0, index + 1).join('/');
      return { name: segment, path };
    });
    return [{ name: '根目录', path: '/' }, ...paths];
  });

  return {
    files,
    currentPath,
    isLoading,
    error,
    uploadProgress,
    downloadProgress,
    directories,
    regularFiles,
    pathSegments,
    loadFiles,
    createDirectory,
    deleteFile,
    renameFile,
    uploadFile,
    downloadFile
  };
}