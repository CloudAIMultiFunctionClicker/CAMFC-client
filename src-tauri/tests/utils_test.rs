// 工具模块单元测试

use camfc_client_lib::utils::bluetooth::is_cpen_device;
use camfc_client_lib::utils::file::{
    get_file_extension, get_file_name, get_file_type_from_extension, FileType,
};
use std::path::Path;

#[test]
fn test_is_cpen_device() {
    // 测试Cpen设备识别
    assert!(is_cpen_device("Cpen-1234"));
    assert!(is_cpen_device("cpen-5678"));
    assert!(is_cpen_device("CPEN-ABCD"));

    // 测试非Cpen设备
    assert!(!is_cpen_device("Device-1234"));
    assert!(!is_cpen_device("Pen-1234"));
    assert!(!is_cpen_device("Cpe")); // 长度不足4个字符
    assert!(!is_cpen_device("")); // 空字符串
}

#[test]
fn test_get_file_type_from_extension() {
    // 测试图片类型
    assert_eq!(get_file_type_from_extension("jpg"), FileType::Image);
    assert_eq!(get_file_type_from_extension("png"), FileType::Image);
    assert_eq!(get_file_type_from_extension("webp"), FileType::Image);

    // 测试视频类型
    assert_eq!(get_file_type_from_extension("mp4"), FileType::Video);
    assert_eq!(get_file_type_from_extension("avi"), FileType::Video);

    // 测试音频类型
    assert_eq!(get_file_type_from_extension("mp3"), FileType::Audio);
    assert_eq!(get_file_type_from_extension("wav"), FileType::Audio);

    // 测试文档类型
    assert_eq!(get_file_type_from_extension("pdf"), FileType::Document);
    assert_eq!(get_file_type_from_extension("docx"), FileType::Document);
    assert_eq!(get_file_type_from_extension("txt"), FileType::Document);

    // 测试压缩包类型
    assert_eq!(get_file_type_from_extension("zip"), FileType::Archive);
    assert_eq!(get_file_type_from_extension("rar"), FileType::Archive);

    // 测试代码类型
    assert_eq!(get_file_type_from_extension("rs"), FileType::Code);
    assert_eq!(get_file_type_from_extension("js"), FileType::Code);
    assert_eq!(get_file_type_from_extension("py"), FileType::Code);

    // 测试其他类型
    assert_eq!(get_file_type_from_extension("unknown"), FileType::Other);
    assert_eq!(get_file_type_from_extension(""), FileType::Other);
}

#[test]
fn test_get_file_extension() {
    // 测试正常文件路径
    assert_eq!(
        get_file_extension(Path::new("test.jpg")),
        Some("jpg".to_string())
    );
    assert_eq!(
        get_file_extension(Path::new("path/to/file.txt")),
        Some("txt".to_string())
    );

    // 测试没有扩展名的文件
    assert_eq!(get_file_extension(Path::new("test")), None);
    assert_eq!(get_file_extension(Path::new("path/to/test")), None);

    // 测试空路径
    assert_eq!(get_file_extension(Path::new("")), None);
}

#[test]
fn test_get_file_name() {
    // 测试正常文件路径
    assert_eq!(
        get_file_name(Path::new("test.jpg")),
        Some("test.jpg".to_string())
    );
    assert_eq!(
        get_file_name(Path::new("path/to/file.txt")),
        Some("file.txt".to_string())
    );

    // 测试目录路径
    assert_eq!(get_file_name(Path::new("path/to/")), None);
    assert_eq!(get_file_name(Path::new("path")), Some("path".to_string()));

    // 测试空路径
    assert_eq!(get_file_name(Path::new("")), None);
}
