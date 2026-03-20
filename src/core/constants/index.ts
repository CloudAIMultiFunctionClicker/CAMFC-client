// 常量定义

// API路径
export const API_PATHS = {
  LOGIN: '/auth/login',
  LOGOUT: '/auth/logout',
  USER_INFO: '/auth/user',
  FILES: '/files',
  FILE_DOWNLOAD: '/files/download',
  FILE_UPLOAD: '/files/upload',
  FILE_DIRECTORIES: '/files/directories'
};

// 蓝牙相关常量
export const BLUETOOTH = {
  SCAN_TIMEOUT: 10000, // 扫描超时时间（毫秒）
  CONNECT_TIMEOUT: 5000, // 连接超时时间（毫秒）
  RESPONSE_TIMEOUT: 3000, // 响应超时时间（毫秒）
  TOTP_CACHE_TTL: 30000, // TOTP缓存时间（毫秒）
  KEEP_ALIVE_INTERVAL: 30000 // 保活间隔（毫秒）
};

// 下载相关常量
export const DOWNLOAD = {
  CHUNK_SIZE: 4 * 1024 * 1024, // 分片大小（4MB）
  MAX_RETRIES: 3, // 最大重试次数
  RETRY_DELAY: 2000 // 重试延迟（毫秒）
};

// 上传相关常量
export const UPLOAD = {
  CHUNK_SIZE: 4 * 1024 * 1024, // 分片大小（4MB）
  MAX_RETRIES: 3, // 最大重试次数
  RETRY_DELAY: 2000 // 重试延迟（毫秒）
};

// 存储键名
export const STORAGE_KEYS = {
  USER_INFO: 'user_info',
  BACKEND_CONFIG: 'backend_config',
  DOWNLOAD_PATH: 'download_path',
  RECENT_DEVICES: 'recent_devices',
  SETTINGS: 'settings'
};

// 事件名称
export const EVENTS = {
  BLUETOOTH_CONNECTED: 'bluetooth:connected',
  BLUETOOTH_DISCONNECTED: 'bluetooth:disconnected',
  BLUETOOTH_ERROR: 'bluetooth:error',
  DOWNLOAD_STARTED: 'download:started',
  DOWNLOAD_PROGRESS: 'download:progress',
  DOWNLOAD_COMPLETED: 'download:completed',
  DOWNLOAD_ERROR: 'download:error',
  UPLOAD_STARTED: 'upload:started',
  UPLOAD_PROGRESS: 'upload:progress',
  UPLOAD_COMPLETED: 'upload:completed',
  UPLOAD_ERROR: 'upload:error',
  TOTP_REFRESHED: 'totp:refreshed'
};

// 错误消息
export const ERROR_MESSAGES = {
  BLUETOOTH_DISABLED: '蓝牙未开启，请先开启蓝牙',
  NO_DEVICE_FOUND: '未找到设备，请确保设备在附近且已开启',
  CONNECTION_FAILED: '连接失败，请检查设备状态',
  TOTP_ERROR: '获取TOTP失败，请重试',
  NETWORK_ERROR: '网络连接失败，请检查网络状态',
  AUTH_ERROR: '认证失败，请重新登录',
  FILE_ERROR: '文件操作失败',
  UNKNOWN_ERROR: '未知错误，请重试'
};

// 成功消息
export const SUCCESS_MESSAGES = {
  BLUETOOTH_CONNECTED: '蓝牙连接成功',
  TOTP_REFRESHED: 'TOTP已刷新',
  FILE_DOWNLOADED: '文件下载完成',
  FILE_UPLOADED: '文件上传完成',
  SETTINGS_SAVED: '设置已保存'
};