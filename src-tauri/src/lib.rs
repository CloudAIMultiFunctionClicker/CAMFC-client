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

// 核心模块
mod core;
// 基础设施模块
mod infrastructure;
// 领域模块
mod domain;
// 应用模块
mod application;

// 托盘相关导入
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::Manager;
use tauri::WindowEvent;

// 使用新的应用服务
use application::services::AppService;
pub use application::commands::{get_totp, get_device_id, scan_cpen_devices, connect_cpen_device, get_connection_status, is_connected, disconnect, cleanup, login, logout, get_user_info, change_password, get_backend_config};
pub use application::commands::get_app_service;

// 导入同步原语
// 原来用tokio::sync::Mutex，继续用这个，适合异步环境
use tokio::sync::Mutex;
use std::sync::OnceLock;
use std::collections::HashMap;
use std::sync::Arc;

// 保留测试用的greet命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 退出应用程序
///
/// 前端调用这个命令来完全退出应用
/// 会先断开蓝牙连接，再关闭应用
#[tauri::command]
async fn exit_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("前端请求退出应用...");
    
    // 退出前先断开蓝牙连接
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;
    
    if let Err(e) = service.disconnect_device().await {
        eprintln!("断开蓝牙连接失败: {}", e);
    } else {
        println!("蓝牙连接已断开");
    }
    
    app_handle.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 创建托盘右键菜单
            // 提供"显示主窗口"和"退出"两个选项
            let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let menu = Menu::with_items(app, &[&show_item, &separator, &quit_item])?;

            // 创建托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("CAMFC Cloud")
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            // 显示主窗口
                            if let Some(window) = app.get_webview_window("main") {
                                if let Err(e) = window.show() {
                                    eprintln!("显示主窗口失败: {}", e);
                                }
                                if let Err(e) = window.set_focus() {
                                    eprintln!("设置主窗口焦点失败: {}", e);
                                }
                            }
                        }
                        "quit" => {
                            // 退出应用前先断开蓝牙连接
                            println!("退出应用，先断开蓝牙连接...");
                            let rt = tokio::runtime::Runtime::new().expect("创建运行时失败");
                            rt.block_on(async {
                                if let Ok(service) = get_app_service().await {
                                    if let Err(e) = service.lock().await.disconnect_device().await {
                                        eprintln!("断开蓝牙连接失败: {}", e);
                                    } else {
                                        println!("蓝牙连接已断开");
                                    }
                                }
                            });
                            // 退出应用
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
                            // 左键点击托盘图标，显示主窗口
                            if let Some(window) = tray.app_handle().get_webview_window("main") {
                                if let Err(e) = window.show() {
                                    eprintln!("显示主窗口失败: {}", e);
                                }
                                if let Err(e) = window.set_focus() {
                                    eprintln!("设置主窗口焦点失败: {}", e);
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // 获取主窗口并设置关闭事件处理
            // 点击关闭按钮时隐藏窗口而不是退出应用
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        // 阻止默认的关闭行为
                        api.prevent_close();
                        // 隐藏窗口
                        if let Err(e) = window_clone.hide() {
                            eprintln!("隐藏窗口失败: {}", e);
                        }
                    }
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,  // 保留测试用的greet命令
            exit_app,  // 退出应用
            get_backend_config,  // 获取后端配置
            get_totp,           // 主要功能：获取TOTP
            scan_cpen_devices,  // 扫描Cpen设备列表
            connect_cpen_device, // 连接指定的Cpen设备
            get_device_id,      // 获取设备ID
            get_connection_status, // 获取连接状态
            is_connected,       // 检查是否已建立稳定连接
            disconnect,         // 断开连接
            cleanup,            // 清理资源
            login,              // 登录
            logout,             // 登出
            get_user_info,      // 获取用户信息
            change_password,    // 修改密码
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
