import { FileInfo, Pagination } from '../../core';
import { getApiClient } from '../../infrastructure';
import { BluetoothService } from '../bluetooth/bluetooth_service';

/**
 * 文件服务
 * 处理文件相关的业务逻辑
 */
export class FileService {
  private static instance: FileService;
  private bluetoothService: BluetoothService;

  private constructor() {
    this.bluetoothService = BluetoothService.getInstance();
  }

  /**
   * 获取文件服务实例
   * @returns 文件服务实例
   */
  static getInstance(): FileService {
    if (!FileService.instance) {
      FileService.instance = new FileService();
    }
    return FileService.instance;
  }

  /**
   * 获取认证头
   * @returns 认证头
   */
  private async getAuthHeader(): Promise<Record<string, string>> {
    const deviceId = await this.bluetoothService.getDeviceId();
    const totp = await this.bluetoothService.getTotp();
    
    return {
      'Id': deviceId,
      'Totp': totp
    };
  }

  /**
   * 列出文件
   * @param path 路径
   * @param pagination 分页参数
   * @returns 文件列表
   */
  async listFiles(path: string = '/', pagination: Pagination = { page: 1, pageSize: 20 }): Promise<FileInfo[]> {
    try {
      const apiClient = getApiClient();
      const authHeader = await this.getAuthHeader();
      
      const files = await apiClient.get<FileInfo[]>('/files', {
        params: {
          path,
          page: pagination.page,
          page_size: pagination.pageSize
        },
        headers: authHeader
      });
      
      return files;
    } catch (error) {
      throw error;
    }
  }

  /**
   * 创建目录
   * @param path 路径
   */
  async createDirectory(path: string): Promise<void> {
    try {
      const apiClient = getApiClient();
      const authHeader = await this.getAuthHeader();
      
      await apiClient.post('/files/directories', { path }, {
        headers: authHeader
      });
    } catch (error) {
      throw error;
    }
  }

  /**
   * 删除文件/目录
   * @param path 路径
   */
  async delete(path: string): Promise<void> {
    try {
      const apiClient = getApiClient();
      const authHeader = await this.getAuthHeader();
      
      await apiClient.delete(`/files/${encodeURIComponent(path)}`, {
        headers: authHeader
      });
    } catch (error) {
      throw error;
    }
  }

  /**
   * 重命名
   * @param oldPath 旧路径
   * @param newPath 新路径
   */
  async rename(oldPath: string, newPath: string): Promise<void> {
    try {
      const apiClient = getApiClient();
      const authHeader = await this.getAuthHeader();
      
      await apiClient.post('/files/rename', {
        old_path: oldPath,
        new_path: newPath
      }, {
        headers: authHeader
      });
    } catch (error) {
      throw error;
    }
  }

  /**
   * 移动
   * @param fromPath 源路径
   * @param toPath 目标路径
   */
  async move(fromPath: string, toPath: string): Promise<void> {
    try {
      const apiClient = getApiClient();
      const authHeader = await this.getAuthHeader();
      
      await apiClient.post('/files/move', {
        from_path: fromPath,
        to_path: toPath
      }, {
        headers: authHeader
      });
    } catch (error) {
      throw error;
    }
  }

  /**
   * 复制
   * @param fromPath 源路径
   * @param toPath 目标路径
   */
  async copy(fromPath: string, toPath: string): Promise<void> {
    try {
      const apiClient = getApiClient();
      const authHeader = await this.getAuthHeader();
      
      await apiClient.post('/files/copy', {
        from_path: fromPath,
        to_path: toPath
      }, {
        headers: authHeader
      });
    } catch (error) {
      throw error;
    }
  }

  /**
   * 下载文件
   * @param path 路径
   * @param onProgress 进度回调
   * @returns 文件数据
   */
  async download(path: string, onProgress?: (progress: number) => void): Promise<Blob> {
    try {
      const apiClient = getApiClient();
      await this.getAuthHeader();
      
      return await apiClient.download(`/files/download/${encodeURIComponent(path)}`, onProgress) as Blob;
    } catch (error) {
      throw error;
    }
  }

  /**
   * 上传文件
   * @param path 路径
   * @param file 文件
   * @param onProgress 进度回调
   */
  async upload(path: string, file: File, onProgress?: (progress: number) => void): Promise<void> {
    try {
      const apiClient = getApiClient();
      await this.getAuthHeader();
      
      const formData = new FormData();
      formData.append('file', file);
      
      await apiClient.upload(`/files/upload/${encodeURIComponent(path)}`, formData, onProgress);
    } catch (error) {
      throw error;
    }
  }
}