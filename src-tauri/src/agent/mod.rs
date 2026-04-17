mod api;
mod gui;
mod parser;
mod screenshot;

use anyhow::Result;
use chrono::Local;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use std::sync::OnceLock;

use api::ApiClient;
use gui::ComputerTools;
use parser::ToolCallParser;
use screenshot::{smart_resize, ScreenshotTool};

// 全局停止标志
static STOP_FLAG: OnceLock<Mutex<bool>> = OnceLock::new();

fn get_current_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn get_output_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("Desktop").join("gui_automation")
}

async fn check_stop_flag() -> bool {
    if let Some(flag) = STOP_FLAG.get() {
        let stop = flag.lock().await;
        *stop
    } else {
        false
    }
}

pub async fn run_gui_automation(instruction: &str, max_step: usize) -> Result<String> {
    let mut output_log = String::new();
    
    // 初始化停止标志
    STOP_FLAG.set(Mutex::new(false)).ok();
    
    // 检查服务端连接
    let log = "[检查服务端连接...]".to_string();
    output_log.push_str(&log);
    output_log.push('\n');
    
    let api_client = ApiClient::new();
    
    if !api_client.check_health().await? {
        let log = "[错误] 无法连接到服务端，请确保 server.py 已启动\n[提示] 请先运行：python server.py".to_string();
        output_log.push_str(&log);
        return Ok(output_log);
    }
    
    let log = "[服务端连接成功]\n".to_string();
    output_log.push_str(&log);

    // 初始化工具
    let screenshot_tool = ScreenshotTool::new();
    let mut computer_tools = ComputerTools::new();
    let parser = ToolCallParser::new();

    // 回到桌面
    computer_tools.reset();

    // 创建输出目录
    let output_dir = get_output_dir();
    std::fs::create_dir_all(&output_dir)?;

    let mut history: Vec<String> = Vec::new();

    let log = format!("[任务] {}\n[开始时间] {}\n{}", instruction, get_current_time(), "=".repeat(60));
    output_log.push_str(&log);
    output_log.push('\n');

    for step_id in 0..max_step {
        // 检查全局停止标志
        if check_stop_flag().await {
            let log = "\n[用户停止] 自动化已被用户停止".to_string();
            output_log.push_str(&log);
            break;
        }

        let log = format!("\n[步骤 {}]\n[上传数据时间] {}", step_id + 1, get_current_time());
        output_log.push_str(&log);
        output_log.push('\n');

        // 截图
        let screenshot_path = output_dir.join(format!("screenshot_{}.png", step_id));
        let screenshot_path_str = screenshot_path.to_str().unwrap();
        
        if let Err(e) = screenshot_tool.capture(screenshot_path_str) {
            let log = format!("截图失败：{}", e);
            output_log.push_str(&log);
            break;
        }

        // 转换为 base64
        let screenshot_base64 = match screenshot_tool.encode_to_base64(screenshot_path_str) {
            Ok(b64) => b64,
            Err(e) => {
                let log = format!("编码失败：{}", e);
                output_log.push_str(&log);
                break;
            }
        };

        // 发送到服务端获取预测
        let result = match api_client.predict(instruction, &screenshot_base64, &history).await {
            Ok(r) => r,
            Err(e) => {
                let log = format!("[错误] 无法获取 AI 预测结果：{}", e);
                output_log.push_str(&log);
                break;
            }
        };

        let output_text = result.output;
        let log = format!("[AI 回复时间] {}\n[模型输出]\n{}\n", result.timestamp, output_text);
        output_log.push_str(&log);

        // 解析工具调用
        let mut action_list = parser.extract_tool_calls(&output_text);
        if action_list.is_empty() {
            let log = "未提取到有效操作".to_string();
            output_log.push_str(&log);
            break;
        }

        // 获取实际屏幕分辨率
        let (screen_width, screen_height) = ComputerTools::get_screen_size();
        let log = format!("[屏幕分辨率] {}x{}", screen_width, screen_height);
        output_log.push_str(&log);
        output_log.push('\n');

        // 执行操作
        for action in &mut action_list {
            // 在每次操作前检查停止标志
            if check_stop_flag().await {
                let log = "\n[用户停止] 自动化已被用户停止".to_string();
                output_log.push_str(&log);
                return Ok(output_log);
            }
            
            // 坐标转换：从归一化坐标 (1000x1000) 转换到实际屏幕分辨率
            parser.transform_coordinates(action, screen_width as u32, screen_height as u32);

            let action_type = parser.get_action_type(action);

            match action_type {
                "click" | "left_click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        computer_tools.left_click(x, y)?;
                        let log = format!("✓ 左键点击 ({}, {})", x, y);
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "mouse_move" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        computer_tools.mouse_move(x, y)?;
                        let log = "✓ 移动鼠标".to_string();
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "middle_click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        computer_tools.middle_click(x, y)?;
                        let log = "✓ 中键点击".to_string();
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "right_click" | "right click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        computer_tools.right_click(x, y)?;
                        let log = format!("✓ 右键点击 ({}, {})", x, y);
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "key" | "hotkey" => {
                    if let Some(keys) = parser.get_keys(action) {
                        computer_tools.press_key(keys.clone())?;
                        let log = format!("✓ 按键 {:?}", keys);
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "type" => {
                    if let Some(text) = parser.get_text(action) {
                        computer_tools.type_text(&text)?;
                        let log = format!("✓ 输入文本：{}", text);
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "drag" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        computer_tools.left_click_drag(x, y)?;
                        let log = "✓ 拖拽".to_string();
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "scroll" => {
                    let pixels = parser.get_pixels(action);
                    computer_tools.scroll(pixels)?;
                    let log = format!("✓ 滚动 {} 像素", pixels);
                    output_log.push_str(&log);
                    output_log.push('\n');
                }
                "computer_double_click" | "double_click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        computer_tools.double_click(x, y)?;
                        let log = "✓ 双击".to_string();
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                }
                "wait" => {
                    let wait_time = parser.get_wait_time(action);
                    computer_tools.wait(wait_time);
                    let log = format!("✓ 等待 {} 秒", wait_time);
                    output_log.push_str(&log);
                    output_log.push('\n');
                }
                "answer" => {
                    if let Some(text) = parser.get_text(action) {
                        let log = format!("✓ 任务完成：{}", text);
                        output_log.push_str(&log);
                        output_log.push('\n');
                    }
                    return Ok(output_log);
                }
                "stop" | "terminate" | "done" => {
                    let status = parser.get_status(action).unwrap_or_else(|| "success".to_string());
                    let log = format!("✓ 任务终止：{}", status);
                    output_log.push_str(&log);
                    output_log.push('\n');
                    return Ok(output_log);
                }
                _ => {
                    let log = format!("未知操作类型：{}", action_type);
                    output_log.push_str(&log);
                    output_log.push('\n');
                }
            }
        }

        // 保存历史
        history.push(output_text);
        thread::sleep(Duration::from_secs(1));
    }

    let log = format!("\n{}\n[完成时间] {}\n[完成] 共执行 {} 步", "=".repeat(60), get_current_time(), history.len());
    output_log.push_str(&log);

    Ok(output_log)
}
