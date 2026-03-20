// 核心类型定义

// 设备信息
export interface DeviceInfo {
  name: string;
  address: string;
  services: string[];
}

// 连接状态
export enum ConnectionState {
  Disconnected = 'disconnected',
  Scanning = 'scanning',
  Connecting = 'connecting',
  Connected = 'connected',
  DisconnectedError = 'disconnected-error'
}

// 认证信息
export interface AuthInfo {
  deviceId: string;
  totp: string;
  userId?: string;
  expiresAt?: number;
}

// 文件信息
export interface FileInfo {
  name: string;
  path: string;
  size: number;
  isDirectory: boolean;
  modifiedTime?: number;
  createdTime?: number;
}

// 分页参数
export interface Pagination {
  page: number;
  pageSize: number;
  total?: number;
}

// 后端配置
export interface BackendConfig {
  baseUrl: string;
  port: number;
  fullUrl: string;
}

// API响应
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

// 下载进度
export interface DownloadProgress {
  fileId: string;
  fileName: string;
  totalSize: number;
  downloaded: number;
  status: string;
  chunksTotal: number;
  chunksCompleted: number;
  speedKbps: number;
  progressPercentage: number;
}

// 上传进度
export interface UploadProgress {
  uploadId: string;
  filename: string;
  totalSize: number;
  uploaded: number;
  status: string;
  chunksTotal: number;
  chunksCompleted: number;
  speedKbps: number;
  progressPercentage: number;
}