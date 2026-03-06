use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use image::ImageFormat;
use std::io::Cursor;
use xcap::Monitor;

pub struct ScreenshotResult {
    pub image_data: String,
    pub width: u32,
    pub height: u32,
}

pub fn capture_screen() -> Result<ScreenshotResult, String> {
    let monitors = Monitor::all().map_err(|e| format!("获取显示器失败: {}", e))?;
    
    if monitors.is_empty() {
        return Err("未检测到显示器".to_string());
    }
    
    let monitor = &monitors[0];
    
    let image = monitor.capture_image().map_err(|e| format!("截图失败: {}", e))?;
    
    let width = image.width();
    let height = image.height();
    
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| format!("编码图片失败: {}", e))?;
    
    let base64_data = BASE64.encode(buffer.into_inner());
    
    Ok(ScreenshotResult {
        image_data: base64_data,
        width,
        height,
    })
}

pub fn get_monitors() -> Result<Vec<MonitorInfo>, String> {
    let monitors = Monitor::all().map_err(|e| format!("获取显示器失败: {}", e))?;
    
    let mut monitor_infos: Vec<MonitorInfo> = Vec::new();
    
    for (i, m) in monitors.iter().enumerate() {
        let name = m.name().map_err(|e| format!("获取显示器名称失败: {}", e))?;
        let width = m.width().map_err(|e| format!("获取显示器宽度失败: {}", e))?;
        let height = m.height().map_err(|e| format!("获取显示器高度失败: {}", e))?;
        let x = m.x().map_err(|e| format!("获取显示器X坐标失败: {}", e))?;
        let y = m.y().map_err(|e| format!("获取显示器Y坐标失败: {}", e))?;
        let is_primary = m.is_primary().map_err(|e| format!("获取主显示器状态失败: {}", e))?;
        
        monitor_infos.push(MonitorInfo {
            id: i as u32,
            name,
            width,
            height,
            x,
            y,
            is_primary,
        });
    }
    
    Ok(monitor_infos)
}

#[derive(serde::Serialize)]
pub struct MonitorInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub is_primary: bool,
}
