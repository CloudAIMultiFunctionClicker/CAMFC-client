//! 常量定义模块
//!
//! 统一管理项目中使用的常量，避免魔法数字散落在代码中
//! 所有常量应该有清晰的命名和必要的注释

/// 默认分片大小：256KB
/// 
/// 用于文件上传和下载的分片大小
/// 选择 256KB 的原因：
/// - 太大会导致单个分片传输时间过长
/// - 太小会增加请求次数和开销
/// - 256KB 是一个平衡点
pub const CHUNK_SIZE: u64 = 256 * 1024; // 256KB

/// TOTP 缓存时间：30 秒
/// 
/// TOTP 的有效期为 30 秒，缓存相应的时间
pub const TOTP_CACHE_DURATION_SECONDS: u64 = 30;

/// TOTP 刷新阈值：25 秒
/// 
/// 当缓存过去 25 秒后（还剩 5 秒过期），就提前刷新
/// 这样可以保证 get_totp() 返回的总是新鲜的 TOTP
pub const TOTP_REFRESH_THRESHOLD_SECONDS: u64 = 25;

/// 蓝牙扫描持续时间：5000 毫秒（5 秒）
/// 
/// 扫描蓝牙设备的持续时间
pub const SCAN_DURATION_MS: u64 = 5000;

/// 最大重试次数
/// 
/// 网络请求和蓝牙操作的最大重试次数
pub const MAX_RETRIES: u32 = 3;

/// 重试延迟：1 秒
/// 
/// 重试前的等待时间
pub const RETRY_DELAY_MS: u64 = 1000;

/// 连接重试延迟：500 毫秒
/// 
/// 蓝牙连接重试前的等待时间
pub const CONNECTION_RETRY_DELAY_MS: u64 = 500;

/// 蓝牙连接最大重试次数
pub const CONNECTION_MAX_RETRIES: u32 = 3;

/// Cpen 设备服务 UUID
pub const CPEN_SERVICE_UUID: &str = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";

/// Cpen 设备特征 UUID
pub const CPEN_CHARACTERISTIC_UUID: &str = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";

/// 上传分片大小：4MB（与后端 API 一致）
/// 
/// 注意：上传和下载的分片大小可能不同
/// 上传使用 4MB 是为了匹配后端 API 的设计
pub const UPLOAD_CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4MB
