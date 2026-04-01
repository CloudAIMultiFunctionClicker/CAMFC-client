//! HTTP API 客户端模块
//!
//! 提供与后端 API 交互的功能，包括：
//! - 文件上传
//! - 文件下载
//! - 认证处理

// 实际的 API 实现（公共模块，供外部访问）
pub mod upload;
pub mod download;

// 重新导出公共类型
pub use upload::{UploadTask, UploadProgress, UploadStatus};
pub use download::{DownloadTask, DownloadProgress, DownloadStatus, AuthInfo};
