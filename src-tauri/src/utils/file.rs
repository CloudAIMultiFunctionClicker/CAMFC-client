// 文件工具模块
// 包含文件相关的常量和通用功能

use std::path::Path;

/// 文件类型分类
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Image,    // 图片
    Video,    // 视频
    Audio,    // 音频
    Document, // 文档
    Archive,  // 压缩包
    Code,     // 代码
    Other,    // 其他
}

/// 文件类型对应的文件夹名称
impl FileType {
    pub fn folder_name(&self) -> &'static str {
        match self {
            FileType::Image => "图片",
            FileType::Video => "视频",
            FileType::Audio => "音频",
            FileType::Document => "文档",
            FileType::Archive => "压缩包",
            FileType::Code => "代码",
            FileType::Other => "其他",
        }
    }
}

/// 根据文件扩展名判断文件类型
pub fn get_file_type_from_extension(ext: &str) -> FileType {
    let ext_lower = ext.to_lowercase();

    match ext_lower.as_str() {
        // 图片
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" => FileType::Image,

        // 视频
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => FileType::Video,

        // 音频
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" => FileType::Audio,

        // 文档
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "odt"
        | "ods" | "odp" => FileType::Document,

        // 压缩包
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => FileType::Archive,

        // 代码
        "js" | "ts" | "html" | "css" | "json" | "xml" | "py" | "java" | "cpp" | "c" | "h"
        | "rs" | "go" | "php" | "rb" | "swift" | "kt" => FileType::Code,

        // 其他
        _ => FileType::Other,
    }
}

/// 从文件路径获取扩展名
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_string())
}

/// 从文件路径获取文件名
pub fn get_file_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
}
