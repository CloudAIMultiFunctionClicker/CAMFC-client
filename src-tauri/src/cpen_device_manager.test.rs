// Cpen设备管理器单元测试

use super::*;
use crate::utils::bluetooth::is_cpen_device;

#[tokio::test]
async fn test_is_cpen_device() {
    // 测试Cpen设备识别
    assert!(is_cpen_device("Cpen-1234"));
    assert!(is_cpen_device("cpen-5678"));
    assert!(is_cpen_device("CPEN-ABCD"));
    assert!(!is_cpen_device("Pen-1234"));
    assert!(!is_cpen_device("Cpe"));
    assert!(!is_cpen_device(""));
}

#[tokio::test]
async fn test_should_refresh_totp() {
    let mut manager = CpenDeviceManager::new();
    
    // 初始状态应该需要刷新
    assert!(manager.should_refresh_totp());
    
    // 设置缓存后，应该不需要刷新
    manager.update_totp_cache("123456".to_string());
    assert!(!manager.should_refresh_totp());
}

#[tokio::test]
async fn test_get_cached_totp() {
    let mut manager = CpenDeviceManager::new();
    
    // 初始状态应该没有缓存
    assert!(manager.get_cached_totp().is_none());
    
    // 设置缓存后，应该能获取到
    manager.update_totp_cache("123456".to_string());
    assert_eq!(manager.get_cached_totp(), Some("123456".to_string()));
}

#[tokio::test]
async fn test_is_valid_totp() {
    // 测试TOTP验证
    assert!(CpenDeviceManager::is_valid_totp("123456"));
    assert!(!CpenDeviceManager::is_valid_totp("12345"));
    assert!(!CpenDeviceManager::is_valid_totp("1234567"));
    assert!(!CpenDeviceManager::is_valid_totp("abcdef"));
    assert!(!CpenDeviceManager::is_valid_totp(""));
}
