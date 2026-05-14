

use tauri::{AppHandle, Manager};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, MoveWindow};

#[tauri::command]
pub fn set_window_size_by_label(app_handle: AppHandle, label: String, width: i32, height: i32, x: Option<i32>, y: Option<i32>) -> Result<(), String> {

    let window = app_handle.get_webview_window(&label)
        .ok_or_else(|| format!("窗口 {} 不存在", label))?;

    let hwnd = window.hwnd()
        .map_err(|e| format!("获取窗口句柄失败：{}", e))?;

    unsafe {

        let (final_x, final_y) = if let (Some(x_val), Some(y_val)) = (x, y) {
            (x_val, y_val)
        } else {

            let mut rect = RECT::default();
            let result = GetWindowRect(HWND(hwnd.0), &mut rect);
            if result.is_err() {
                return Err(format!("GetWindowRect 调用失败：{:?}", result.err()));
            }
            (rect.left, rect.top)
        };

        use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        let clamped_x = final_x.max(0).min(screen_width as i32 - width);
        let clamped_y = final_y.max(0).min(screen_height as i32 - height);

        let result = MoveWindow(
            HWND(hwnd.0),
            clamped_x,
            clamped_y,
            width,
            height,
            true,
        );

        if result.is_err() {
            return Err(format!("MoveWindow 调用失败：{:?}", result.err()));
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_window_size_by_label(app_handle: AppHandle, label: String) -> Result<serde_json::Value, String> {

    let window = app_handle.get_webview_window(&label)
        .ok_or_else(|| format!("窗口 {} 不存在", label))?;

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
