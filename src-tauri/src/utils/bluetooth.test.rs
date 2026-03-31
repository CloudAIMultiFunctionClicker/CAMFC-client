// 蓝牙工具模块单元测试

use super::*;

#[test]
async fn test_is_cpen_device() {
    // 测试Cpen设备识别
    assert!(is_cpen_device("Cpen-1234"));
    assert!(is_cpen_device("cpen-5678"));
    assert!(is_cpen_device("CPEN-ABCD"));
    assert!(!is_cpen_device("Pen-1234"));
    assert!(!is_cpen_device("Cpe"));
    assert!(!is_cpen_device(""));
}
