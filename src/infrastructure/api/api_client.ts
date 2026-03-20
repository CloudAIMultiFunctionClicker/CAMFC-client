import axios, { AxiosInstance, AxiosRequestConfig } from 'axios';

/**
 * API客户端
 * 封装axios，提供统一的API请求管理
 */
export class ApiClient {
  private client: AxiosInstance;

  constructor(baseUrl: string) {
    this.client = axios.create({
      baseURL: baseUrl,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json'
      }
    });

    // 请求拦截器
    this.client.interceptors.request.use(
      (config) => {
        // 可以在这里添加认证头
        return config;
      },
      (error) => {
        return Promise.reject(error);
      }
    );

    // 响应拦截器
    this.client.interceptors.response.use(
      (response) => {
        return response;
      },
      (error) => {
        // 统一错误处理
        if (error.response) {
          // 服务器返回错误
          const status = error.response.status;
          switch (status) {
            case 401:
              // 未授权，可能需要重新登录
              break;
            case 403:
              // 禁止访问
              break;
            case 404:
              // 资源不存在
              break;
            case 500:
              // 服务器错误
              break;
            default:
              break;
          }
        } else if (error.request) {
          // 请求已发出但没有收到响应
          console.error('网络连接失败');
        } else {
          // 请求配置出错
          console.error('请求配置错误:', error.message);
        }
        return Promise.reject(error);
      }
    );
  }

  /**
   * 设置基础URL
   * @param baseUrl 基础URL
   */
  setBaseUrl(baseUrl: string): void {
    this.client.defaults.baseURL = baseUrl;
  }

  /**
   * GET请求
   * @param url 请求路径
   * @param config 配置
   * @returns 响应数据
   */
  async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.get<T>(url, config);
    return response.data;
  }

  /**
   * POST请求
   * @param url 请求路径
   * @param data 请求数据
   * @param config 配置
   * @returns 响应数据
   */
  async post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.post<T>(url, data, config);
    return response.data;
  }

  /**
   * PUT请求
   * @param url 请求路径
   * @param data 请求数据
   * @param config 配置
   * @returns 响应数据
   */
  async put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.put<T>(url, data, config);
    return response.data;
  }

  /**
   * DELETE请求
   * @param url 请求路径
   * @param config 配置
   * @returns 响应数据
   */
  async delete<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.delete<T>(url, config);
    return response.data;
  }

  /**
   * 上传文件
   * @param url 请求路径
   * @param formData 表单数据
   * @param onProgress 进度回调
   * @returns 响应数据
   */
  async upload<T>(url: string, formData: FormData, onProgress?: (progress: number) => void): Promise<T> {
    const response = await this.client.post<T>(url, formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      },
      onUploadProgress: (progressEvent) => {
        if (progressEvent.total && onProgress) {
          const progress = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          onProgress(progress);
        }
      }
    });
    return response.data;
  }

  /**
   * 下载文件
   * @param url 请求路径
   * @param onProgress 进度回调
   * @returns 响应数据
   */
  async download<T>(url: string, onProgress?: (progress: number) => void): Promise<T> {
    const response = await this.client.get<T>(url, {
      responseType: 'blob',
      onDownloadProgress: (progressEvent) => {
        if (progressEvent.total && onProgress) {
          const progress = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          onProgress(progress);
        }
      }
    });
    return response.data;
  }
}

// 全局API客户端实例
let apiClient: ApiClient | null = null;

/**
 * 获取API客户端实例
 * @param baseUrl 基础URL
 * @returns API客户端实例
 */
export function getApiClient(baseUrl?: string): ApiClient {
  if (!apiClient && baseUrl) {
    apiClient = new ApiClient(baseUrl);
  } else if (apiClient && baseUrl) {
    apiClient.setBaseUrl(baseUrl);
  }
  
  if (!apiClient) {
    throw new Error('API client not initialized');
  }
  
  return apiClient;
}