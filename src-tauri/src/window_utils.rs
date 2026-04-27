// 保留所有权利
//
// Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
// Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
//
// Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
// Email: abc.cxh2009@foxmail.com
//
// Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
// Email: 1220594170@qq.com
//
// Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
// Email: admin@mc666.top

use tauri::{AppHandle, Manager};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, MoveWindow};

/// 通过 WinAPI 设置窗口大小和位置
/// 
/// 参数：
/// - label: 窗口标签（例如 "float-normal-empty"）
/// - width, height: 窗口宽度和高度（必须）
/// - x, y: 窗口左上角坐标（可选，不提供则保持原位置）
#[tauri::command]
pub fn set_window_size_by_label(app_handle: AppHandle, label: String, width: i32, height: i32, x: Option<i32>, y: Option<i32>) -> Result<(), String> {

    // 获取窗口
    let window = app_handle.get_webview_window(&label)
        .ok_or_else(|| format!("窗口 {} 不存在", label))?;
    
    // 获取窗口的 HWND
    let hwnd = window.hwnd()
        .map_err(|e| format!("获取窗口句柄失败：{}", e))?;
    
    unsafe {
        // 如果需要设置位置，获取当前坐标；否则获取当前坐标保持不变
        let (final_x, final_y) = if let (Some(x_val), Some(y_val)) = (x, y) {
            (x_val, y_val)
        } else {
            // 获取当前窗口位置
            let mut rect = RECT::default();
            let result = GetWindowRect(HWND(hwnd.0), &mut rect);
            if result.is_err() {
                return Err(format!("GetWindowRect 调用失败：{:?}", result.err()));
            }
            (rect.left, rect.top)
        };
        
        // 获取屏幕尺寸，确保窗口不会移出屏幕
        use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        
        // 限制窗口位置，确保完整显示在屏幕内
        let clamped_x = final_x.max(0).min(screen_width as i32 - width);
        let clamped_y = final_y.max(0).min(screen_height as i32 - height);
        
        // 使用 MoveWindow 设置窗口大小和位置
        let result = MoveWindow(
            HWND(hwnd.0),
            clamped_x,
            clamped_y,
            width,
            height,
            true, // 重绘窗口
        );
        
        if result.is_err() {
            return Err(format!("MoveWindow 调用失败：{:?}", result.err()));
        }
    }
    
    Ok(())
}

/// 获取窗口当前大小
#[tauri::command]
pub fn get_window_size_by_label(app_handle: AppHandle, label: String) -> Result<serde_json::Value, String> {

    
    // 获取窗口
    let window = app_handle.get_webview_window(&label)
        .ok_or_else(|| format!("窗口 {} 不存在", label))?;
    
    // 获取窗口的 HWND
    let hwnd = window.hwnd()
        .map_err(|e| format!("获取窗口句柄失败：{}", e))?;
    
    unsafe {
        let mut rect = RECT::default();
        let result = GetWindowRect(HWND(hwnd.0), &mut rect);
        
        if result.is_err() {
            return Err(format!("GetWindowRect 调用失败：{:?}", result.err()));
        }
        
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        
        tracing::info!("窗口 {} 当前大小：{}x{}", label, width, height);
        
        Ok(serde_json::json!({
            "x": rect.left,
            "y": rect.top,
            "width": width,
            "height": height
        }))
    }
}
