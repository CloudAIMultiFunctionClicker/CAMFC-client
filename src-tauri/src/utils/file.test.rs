// 文件工具模块单元测试

use super::*;
use std::path::Path;

#[test]
async fn test_get_file_type_from_extension() {
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
    assert_eq!(get_file_type_from_extension("doc"), FileType::Document);
    assert_eq!(get_file_type_from_extension("txt"), FileType::Document);
    
    // 测试压缩包类型
    assert_eq!(get_file_type_from_extension("zip"), FileType::Archive);
    assert_eq!(get_file_type_from_extension("rar"), FileType::Archive);
    
    // 测试代码类型
    assert_eq!(get_file_type_from_extension("rs"), FileType::Code);
    assert_eq!(get_file_type_from_extension("js"), FileType::Code);
    
    // 测试其他类型
    assert_eq!(get_file_type_from_extension("unknown"), FileType::Other);
}

#[test]
async fn test_get_file_extension() {
    // 测试获取文件扩展名
    assert_eq!(get_file_extension(Path::new("file.txt")), Some("txt".to_string()));
    assert_eq!(get_file_extension(Path::new("image.jpg")), Some("jpg".to_string()));
    assert_eq!(get_file_extension(Path::new("document.pdf")), Some("pdf".to_string()));
    assert_eq!(get_file_extension(Path::new("no_extension")), None);
}

#[test]
async fn test_get_file_name() {
    // 测试获取文件名
    assert_eq!(get_file_name(Path::new("/path/to/file.txt")), Some("file.txt".to_string()));
    assert_eq!(get_file_name(Path::new("image.jpg")), Some("image.jpg".to_string()));
    assert_eq!(get_file_name(Path::new("/path/to/directory")), Some("directory".to_string()));
}
