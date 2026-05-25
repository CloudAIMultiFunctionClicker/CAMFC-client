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

use crate::{check_agent_stop_flag, set_agent_stop_flag};
use api::ApiClient;
use gui::ComputerTools;
use parser::ToolCallParser;
use screenshot::ScreenshotTool;

fn get_current_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn get_output_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("Desktop").join("gui_automation")
}

async fn check_stop_flag() -> bool {
    check_agent_stop_flag().await
}

pub async fn run_gui_automation(instruction: &str, max_step: usize) -> Result<String> {
    let mut output_log = String::new();

    set_agent_stop_flag(false).await;

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

    let screenshot_tool = ScreenshotTool::new();
    let mut computer_tools = ComputerTools::new();
    let parser = ToolCallParser::new();

    computer_tools.reset();

    let output_dir = get_output_dir();
    std::fs::create_dir_all(&output_dir)?;

    let mut history: Vec<String> = Vec::new();

    let log = format!("[任务] {}\n[开始时间] {}\n{}", instruction, get_current_time(), "=".repeat(60));
    output_log.push_str(&log);
    output_log.push('\n');

    for step_id in 0..max_step {

        if check_stop_flag().await {
            let log = "\n[用户停止] 自动化已被用户停止".to_string();
            output_log.push_str(&log);
            break;
        }

        let log = format!("\n[步骤 {}]\n[上传数据时间] {}", step_id + 1, get_current_time());
        output_log.push_str(&log);
        output_log.push('\n');

        let screenshot_path = output_dir.join(format!("screenshot_{}.png", step_id));
        let screenshot_path_str = screenshot_path.to_str().unwrap();

        if let Err(e) = screenshot_tool.capture(screenshot_path_str) {
            let log = format!("截图失败：{}", e);
            output_log.push_str(&log);
            break;
        }

        let screenshot_base64 = match screenshot_tool.encode_to_base64(screenshot_path_str) {
            Ok(b64) => b64,
            Err(e) => {
                let log = format!("编码失败：{}", e);
                output_log.push_str(&log);
                break;
            }
        };

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

        let mut action_list = parser.extract_tool_calls(&output_text);
        if action_list.is_empty() {
            let log = "未提取到有效操作".to_string();
            output_log.push_str(&log);
            break;
        }

        // 显示提取到的操作详情
        let log = format!("\n[解析结果] 提取到 {} 个操作:\n", action_list.len());
        output_log.push_str(&log);
        for (i, action) in action_list.iter().enumerate() {
            let coords = parser.get_coordinate(action);
            let coord_str = match coords {
                Some((x, y)) => format!("坐标({}, {})", x, y),
                None => "无坐标".to_string()
            };
            let log = format!("  {}. [{}] {}\n", i + 1, action.arguments.action, coord_str);
            output_log.push_str(&log);
        }

        let (screen_width, screen_height) = ComputerTools::get_screen_size();
        let log = format!("[屏幕分辨率] {}x{}", screen_width, screen_height);
        output_log.push_str(&log);
        output_log.push('\n');

        for action in &mut action_list {

            if check_stop_flag().await {
                let log = "\n[用户停止] 自动化已被用户停止".to_string();
                output_log.push_str(&log);
                return Ok(output_log);
            }

            parser.transform_coordinates(action, screen_width as u32, screen_height as u32);

            let action_type = parser.get_action_type(action);

            let log = format!("\n▶ 执行操作 [{}]\n", action_type.to_uppercase());
            output_log.push_str(&log);

            match action_type {
                "click" | "left_click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        let log = format!("  → 移动到坐标 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                        computer_tools.left_click(x, y)?;
                        let log = format!("✓ 完成: 左键点击 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                    }
                }
                "mouse_move" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        let log = format!("  → 移动到坐标 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                        computer_tools.mouse_move(x, y)?;
                        let log = "✓ 完成: 移动鼠标\n".to_string();
                        output_log.push_str(&log);
                    }
                }
                "middle_click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        let log = format!("  → 移动到坐标 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                        computer_tools.middle_click(x, y)?;
                        let log = format!("✓ 完成: 中键点击 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                    }
                }
                "right_click" | "right click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        let log = format!("  → 移动到坐标 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                        computer_tools.right_click(x, y)?;
                        let log = format!("✓ 完成: 右键点击 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                    }
                }
                "key" | "hotkey" => {
                    // 优先使用 keys 字段，如果没有则使用 text 字段
                    let keys = parser.get_keys(action);
                    if keys.is_none() {
                        // 尝试从 text 字段获取单个按键
                        if let Some(text) = parser.get_text(action) {
                            let key_vec = vec![text.clone()];
                            let log = format!("  → 按键 (text): {}\n", text);
                            output_log.push_str(&log);
                            computer_tools.press_key(key_vec.clone())?;
                            let log = format!("✓ 完成: 按键 {}\n", text);
                            output_log.push_str(&log);
                        }
                    } else if let Some(keys) = keys {
                        let log = format!("  → 按键: {:?}\n", keys);
                        output_log.push_str(&log);
                        computer_tools.press_key(keys.clone())?;
                        let log = format!("✓ 完成: 按键 {:?}\n", keys);
                        output_log.push_str(&log);
                    }
                }
                "type" => {
                    if let Some(text) = parser.get_text(action) {
                        let display_text = if text.len() > 30 { format!("{}...", &text[..30]) } else { text.clone() };
                        let log = format!("  → 输入文本: {}\n", display_text);
                        output_log.push_str(&log);
                        computer_tools.type_text(&text)?;
                        let log = format!("✓ 完成: 输入 {} 个字符\n", text.len());
                        output_log.push_str(&log);
                    }
                }
                "drag" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        let log = format!("  → 拖拽到坐标 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                        computer_tools.left_click_drag(x, y)?;
                        let log = "✓ 完成: 拖拽\n".to_string();
                        output_log.push_str(&log);
                    }
                }
                "scroll" => {
                    let pixels = parser.get_pixels(action);
                    let log = format!("  → 滚动 {} 像素\n", pixels);
                    output_log.push_str(&log);
                    computer_tools.scroll(pixels)?;
                    let log = format!("✓ 完成: 滚动 {} 像素\n", pixels);
                    output_log.push_str(&log);
                }
                "computer_double_click" | "double_click" => {
                    if let Some((x, y)) = parser.get_coordinate(action) {
                        let log = format!("  → 移动到坐标 ({}, {})\n", x, y);
                        output_log.push_str(&log);
                        computer_tools.double_click(x, y)?;
                        let log = format!("✓ 完成: 双击 ({}, {})\n", x, y);
                        output_log.push_str(&log);
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

        history.push(output_text);
        thread::sleep(Duration::from_secs(1));
    }

    let log = format!("\n{}\n[完成时间] {}\n[完成] 共执行 {} 步", "=".repeat(60), get_current_time(), history.len());
    output_log.push_str(&log);

    Ok(output_log)
}
