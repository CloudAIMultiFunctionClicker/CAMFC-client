// 蓝牙工具模块
// 包含蓝牙相关的常量和通用功能

/// Cpen设备UUID常量
pub const CPEN_SERVICE_UUID: &str = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
pub const CPEN_CHARACTERISTIC_UUID: &str = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";

/// 蓝牙扫描时间常量（毫秒）
pub const SCAN_DURATION_MS: u64 = 5000;

/// 蓝牙操作重试次数
pub const MAX_RETRY_COUNT: u32 = 3;
/// 重试延迟时间（毫秒）
pub const RETRY_DELAY_MS: u64 = 500;

/// 检查设备名是否为Cpen设备
///
/// 根据设备名前缀判断是否为Cpen设备
/// 检查前4个字符是否为'cpen'（不区分大小写）
pub fn is_cpen_device(name: &str) -> bool {
    if name.chars().count() >= 4 {
        let prefix: String = name.chars().take(4).collect();
        prefix.to_lowercase() == "cpen"
    } else {
        false
    }
}
