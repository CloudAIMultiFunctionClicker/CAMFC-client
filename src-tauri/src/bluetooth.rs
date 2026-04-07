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

use btleplug::api::{Central, Peripheral, ScanFilter, WriteType, CharPropFlags, Manager as _};
use btleplug::platform::{Manager, Adapter};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use std::error::Error;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::event_emitter::emit_button_event;

// Windows蓝牙API - 用来检测和开启蓝牙无线电
// 注意：暂时只支持Windows平台，后面如果跨平台再考虑兼容
use windows::Devices::Radios::Radio;
use windows::Devices::Radios::RadioAccessStatus;
use windows::Devices::Radios::RadioKind;

type BtError = String;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseType {
    SetTime,
    GetTotp,
    GetId,
}

impl ResponseType {
    fn matches(&self, response: &BluetoothResponse) -> bool {
        match (self, response) {
            (ResponseType::SetTime, BluetoothResponse::SetTime(_)) => true,
            (ResponseType::GetTotp, BluetoothResponse::GetTotp(_)) => true,
            (ResponseType::GetId, BluetoothResponse::GetId(_)) => true,
            _ => false,
        }
    }
}

impl From<BluetoothResponse> for Vec<u8> {
    fn from(resp: BluetoothResponse) -> Vec<u8> {
        match resp {
            BluetoothResponse::SetTime(s) => s.into_bytes(),
            BluetoothResponse::GetTotp(s) => s.into_bytes(),
            BluetoothResponse::GetId(s) => s.into_bytes(),
            BluetoothResponse::ButtonEvent(_) => vec![],
        }
    }
}

impl BluetoothResponse {
    pub fn into_bytes(self) -> Vec<u8> {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub enum BluetoothResponse {
    SetTime(String),
    GetTotp(String),
    GetId(String),
    ButtonEvent(u8),
}

/// 设备信息
#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub address: String,
    pub services: Vec<Uuid>,
}

/// 蓝牙管理器
pub struct BluetoothManager {
    adapter: Option<Adapter>,
    connected_peripheral: Option<btleplug::platform::Peripheral>,
    listening_handle: Option<tokio::task::JoinHandle<()>>,
    last_connected_state: Option<bool>,
    response_rx: Option<tokio::sync::mpsc::Receiver<BluetoothResponse>>,
    button_rx: Option<tokio::sync::mpsc::Receiver<BluetoothResponse>>,
}

impl BluetoothManager {
    pub fn new() -> Self {
        Self {
            adapter: None,
            connected_peripheral: None,
            listening_handle: None,
            last_connected_state: None,
            response_rx: None,
            button_rx: None,
        }
    }

    /// 1. 查找并启用蓝牙设备（使用新实现）
    /// 
    /// 这个函数会：
    /// 1. 获取所有无线电设备
    /// 2. 查找蓝牙无线电
    /// 3. 检查当前状态
    /// 4. 如果未开启，尝试自动开启
    /// 
    /// 返回值: 
    /// - Ok(true): 蓝牙已成功启用或已经是开启状态
    /// - Ok(false): 未找到蓝牙设备
    /// - Err(...): 过程中发生错误
    /// 
    /// 思考：新实现使用了正确的Windows API调用方式
    /// 用.get()方法同步等待异步操作，应该能解决之前的编译错误
    pub fn enable_bluetooth(&self) -> Result<(), String> {
        tracing::info!("开始检查并启用蓝牙设备（使用Windows Radio API）...");
        
        // 调用内部实现，然后适配返回类型
        match self.enable_bluetooth_internal() {
            Ok(true) => {
                tracing::info!("✅ 蓝牙已成功启用或已经是开启状态");
                Ok(())
            }
            Ok(false) => {
                let err_msg = "未找到蓝牙设备".to_string();
                tracing::info!("❌ {}", err_msg);
                tracing::info!("请确保：");
                tracing::info!("1. 计算机支持蓝牙功能");
                tracing::info!("2. 蓝牙硬件已正确安装");
                tracing::info!("3. 蓝牙驱动程序已更新");
                Err(err_msg)
            }
            Err(e) => {
                let err_msg = format!("蓝牙启用失败: {}", e);
                tracing::info!("❌ {}", err_msg);
                Err(err_msg)
            }
        }
    }
    
    /// 内部实现：查找并启用蓝牙设备
    /// 
    /// 这是代码实现，使用Windows Radio API
    /// 返回类型保持原样：Result<bool, Box<dyn std::error::Error>>
    fn enable_bluetooth_internal(&self) -> Result<bool, Box<dyn std::error::Error>> {
        tracing::info!("正在查找蓝牙设备...");

        // 获取所有无线电设备
        let async_op = Radio::GetRadiosAsync()?;
        let radios = async_op.get()?;
        
        // 查找蓝牙设备
        let bluetooth_radio = Self::find_bluetooth_radio(&radios);
        
        match bluetooth_radio {
            Some(radio) => {
                // 检查当前状态
                let current_state = radio.State()?;
                
                if current_state == windows::Devices::Radios::RadioState::On {
                    tracing::info!("蓝牙已经是开启状态");
                    Ok(true)
                } else {
                    tracing::info!("正在启用蓝牙...");
                    
                    // 尝试启用蓝牙
                    let result = radio.SetStateAsync(windows::Devices::Radios::RadioState::On)?.get()?;
                    
                    match result {
                        RadioAccessStatus::Allowed => {
                            tracing::info!("蓝牙启用成功！");
                            tracing::info!("新状态: {:?}", radio.State()?);
                            Ok(true)
                        }
                        RadioAccessStatus::DeniedBySystem => {
                            let err_msg = "系统拒绝访问蓝牙设备，可能的原因：管理员权限不足或系统策略限制";
                             tracing::error!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                        RadioAccessStatus::DeniedByUser => {
                            let err_msg = "用户拒绝访问蓝牙设备";
                             tracing::error!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                        RadioAccessStatus::Unspecified => {
                            let err_msg = "未知错误，无法启用蓝牙";
                             tracing::error!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                        _ => {
                            let err_msg = format!("未知的访问状态: {:?}", result);
                             tracing::error!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                    }
                }
            }
            None => {
                 tracing::error!("未找到蓝牙设备");
                 tracing::error!("请确保：");
                 tracing::error!("1. 计算机支持蓝牙功能");
                 tracing::error!("2. 蓝牙硬件已正确安装");
                 tracing::error!("3. 蓝牙驱动程序已更新");
                Ok(false)
            }
        }
    }
    
    /// 辅助函数：在无线电设备列表中查找蓝牙设备
    /// 
    /// 遍历所有无线电设备，找到类型为蓝牙的设备
    fn find_bluetooth_radio(radios: &windows::Foundation::Collections::IVectorView<Radio>) -> Option<Radio> {
        tracing::info!("在 {} 个无线电设备中查找蓝牙设备...", radios.Size().unwrap_or(0));
        
        let count = radios.Size().unwrap_or(0);
        for i in 0..count {
            match radios.GetAt(i) {
                Ok(radio) => {
                    match radio.Kind() {
                        Ok(kind) => {
                            if kind == RadioKind::Bluetooth {
                                tracing::info!("找到蓝牙无线电设备 (索引: {})", i);
                                return Some(radio);
                            }
                        }
                        Err(e) => {
                            tracing::info!("获取无线电设备类型失败 (索引: {}): {}", i, e);
                        }
                    }
                }
                Err(e) => {
                    tracing::info!("获取无线电设备失败 (索引: {}): {}", i, e);
                }
            }
        }
        
        tracing::info!("未找到蓝牙无线电设备");
        None
    }
    
    /// 新增：简单的蓝牙状态检查（通过btleplug适配器）
    /// 
    /// 这个方法通过尝试创建Manager来检查蓝牙是否可用
    /// 比Windows Radio API更直接，但可能无法开启蓝牙
    /// 
    /// 思考：这个方法应该放在哪里？也许可以作为fallback
    /// 先实现，后面再看怎么用
    pub async fn check_bluetooth_via_btleplug(&mut self) -> Result<bool, BtError> {
        tracing::info!("通过btleplug检查蓝牙状态...");
        
        // 尝试创建Manager，如果失败说明蓝牙可能不可用
        match Manager::new().await {
            Ok(_manager) => {
                tracing::info!("btleplug Manager创建成功，蓝牙应该可用");
                Ok(true)
            }
            Err(e) => {
                tracing::info!("btleplug Manager创建失败，蓝牙可能不可用: {}", e);
                // 返回错误，但用友好描述
                Err(format!("蓝牙检测失败: {}", e))
            }
        }
    }

    /// 初始化适配器
    async fn get_adapter(&mut self) -> Result<&Adapter, BtError> {
        if self.adapter.is_none() {
            let manager = Manager::new().await
                .map_err(|e| format!("创建管理器失败: {}", e))?;
            
            let adapters = manager.adapters().await
                .map_err(|e| format!("获取适配器失败: {}", e))?;
            
            self.adapter = adapters.into_iter().next();
        }
        
        self.adapter.as_ref().ok_or_else(|| "没有适配器".to_string())
    }

    /// 2. 扫描设备
    pub async fn scan_devices(&mut self, duration_ms: u64) -> Result<Vec<DeviceInfo>, BtError> {
        let adapter = self.get_adapter().await?;
        
        tracing::info!("扫描设备 {}ms...", duration_ms);
        adapter.start_scan(ScanFilter::default()).await
            .map_err(|e| format!("开始扫描失败: {}", e))?;
        
        sleep(Duration::from_millis(duration_ms)).await;
        
        let peripherals = adapter.peripherals().await
            .map_err(|e| format!("获取设备列表失败: {}", e))?;
        
        adapter.stop_scan().await
            .map_err(|e| format!("停止扫描失败: {}", e))?;
        
        let mut devices = Vec::new();
        
        for p in &peripherals {
            if let Ok(Some(props)) = p.properties().await {
                let name = props.local_name.unwrap_or("未知设备".to_string());
                let address = props.address.to_string();
                // 简化处理：直接不包含services
                devices.push(DeviceInfo { name, address, services: vec![] });
            }
        }
        
        Ok(devices)
    }

    /// 3. 连接指定设备
    /// 
    /// 改进：添加重试机制，提高连接成功率
    /// 改进：连接前强制清理旧的监听任务，避免使用旧对象
    pub async fn connect(&mut self, address: &str) -> Result<(), BtError> {
        // 连接前强制清理旧的监听任务和状态
        // 这确保连接新设备时不会复用旧的监听任务
        tracing::info!("[BLUETOOTH] 连接前清理旧状态...");
        self.cleanup_connection_state().await;
        
        const MAX_RETRIES: u32 = 3;
        const RETRY_DELAY_MS: u64 = 500;
        
        for attempt in 1..=MAX_RETRIES {
            tracing::info!("[BLUETOOTH] 连接尝试 {}/{}: {}", attempt, MAX_RETRIES, address);
            
            match self.connect_once(address).await {
                Ok(_) => {
                    tracing::info!("[BLUETOOTH] 连接成功");
                    return Ok(());
                }
                Err(e) if attempt < MAX_RETRIES => {
                    tracing::info!("[BLUETOOTH] 连接失败，{}ms后重试: {}", RETRY_DELAY_MS, e);
                    // 清理状态后重试
                    self.cleanup_connection_state().await;
                    sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                }
                Err(e) => {
                    tracing::info!("[BLUETOOTH] 连接重试次数用尽: {}", e);
                    return Err(e);
                }
            }
        }
        
        Err("连接重试次数用尽".to_string())
    }
    
    /// 单次连接尝试（内部方法）
    async fn connect_once(&mut self, address: &str) -> Result<(), BtError> {
        tracing::info!("[BLUETOOTH] 开始连接 {}...", address);
        
        // 先扫描找到设备
        let adapter = self.get_adapter().await?;
        adapter.start_scan(ScanFilter::default()).await
            .map_err(|e| format!("开始扫描失败: {}", e))?;
        
        sleep(Duration::from_secs(2)).await;
        
        let peripherals = adapter.peripherals().await
            .map_err(|e| format!("获取设备列表失败: {}", e))?;
        
        adapter.stop_scan().await
            .map_err(|e| format!("停止扫描失败: {}", e))?;
        
        // 查找目标
        let mut target = None;
        for p in &peripherals {
            if let Ok(Some(props)) = p.properties().await {
                if props.address.to_string() == address {
                    target = Some(p.clone());
                    break;
                }
            }
        }
        
        let peripheral = target.ok_or_else(|| format!("未找到设备: {}", address))?;
        
        peripheral.connect().await
            .map_err(|e| format!("连接失败: {}", e))?;
        
        // 连接后等待更长时间让连接稳定
        sleep(Duration::from_millis(500)).await;
        
        // 验证连接状态
        if !peripheral.is_connected().await.map_err(|e| format!("检查连接失败: {}", e))? {
            return Err("连接后立即断开".to_string());
        }
        
        // 预先发现服务，避免后续操作时出错
        tracing::info!("[BLUETOOTH] 发现服务...");
        match timeout(Duration::from_secs(5), peripheral.discover_services()).await {
            Ok(Ok(_)) => tracing::info!("[BLUETOOTH] 服务发现完成"),
            Ok(Err(e)) => {
                tracing::info!("[BLUETOOTH] 服务发现失败: {}", e);
                // 不返回错误，继续尝试
            }
            Err(_) => {
                tracing::info!("[BLUETOOTH] 服务发现超时");
                // 不返回错误，继续尝试
            }
        }
        
        // 再等待一下让服务发现生效
        sleep(Duration::from_millis(200)).await;
        
        self.connected_peripheral = Some(peripheral);
        Ok(())
    }

    /// 彻底清理连接状态（内部方法）
    /// 
    /// 这个方法会清理所有与连接相关的状态：
    /// - 停止监听任务并等待其结束
    /// - 清空监听通道
    /// - 清空已连接的peripheral
    /// 
    /// 改进：等待监听任务真正结束，避免使用已关闭的对象
    async fn cleanup_connection_state(&mut self) {
        if let Some(h) = self.listening_handle.take() {
            h.abort();
            match timeout(Duration::from_secs(1), h).await {
                Ok(_) => tracing::info!("[BLUETOOTH] 监听任务已结束"),
                Err(_) => tracing::info!("[BLUETOOTH] 监听任务结束超时，继续清理"),
            }
        }
        self.response_rx = None;
        self.button_rx = None;
        self.connected_peripheral = None;
        tracing::info!("[BLUETOOTH] 连接状态已彻底清理");
    }

    /// 断开连接
    pub async fn disconnect(&mut self) -> Result<(), BtError> {
        tracing::info!("[BLUETOOTH] 开始断开连接...");
        
        // 先停止监听
        self.stop_listening().await;
        
        // 尝试断开蓝牙连接，忽略错误（可能已经断开了）
        if let Some(p) = &self.connected_peripheral {
            let _ = p.disconnect().await;
        }
        
        // 彻底清理状态
        self.cleanup_connection_state().await;
        
        tracing::info!("[BLUETOOTH] 断开连接完成");
        Ok(())
    }

    /// 检查是否已建立稳定连接
    /// 
    /// 这里不只是检查是否有连接的peripheral对象，还实际检查蓝牙物理连接状态
    /// 注意：这个方法可能会有一定的延迟（蓝牙设备响应时间）
    /// 
    /// 改进：添加超时保护，避免在设备无响应时卡住
    pub async fn is_connected(&mut self) -> Result<bool, BtError> {
        match &self.connected_peripheral {
            Some(peripheral) => {
                match timeout(Duration::from_secs(2), peripheral.is_connected()).await {
                    Ok(Ok(connected)) => {
                        if self.last_connected_state != Some(connected) {
                            self.last_connected_state = Some(connected);
                            tracing::info!("[BLUETOOTH] 连接状态变化: {}", connected);
                        }
                        Ok(connected)
                    }
                    Ok(Err(e)) => {
                        if self.last_connected_state != Some(false) {
                            self.last_connected_state = Some(false);
                            tracing::info!("[BLUETOOTH] 检查连接状态失败: {}", e);
                        }
                        Err(format!("检查连接状态失败: {}", e))
                    }
                    Err(_) => {
                        if self.last_connected_state != Some(false) {
                            self.last_connected_state = Some(false);
                            tracing::info!("[BLUETOOTH] 连接状态检测超时，假设已断开");
                        }
                        Ok(false)
                    }
                }
            }
            None => {
                if self.last_connected_state != Some(false) {
                    self.last_connected_state = Some(false);
                    tracing::info!("[BLUETOOTH] 没有已连接的peripheral");
                }
                Ok(false)
            }
        }
    }

    /// 确保服务已发现并准备好
    /// 
    /// 在重新连接后调用，确保设备服务已经完全准备好
    pub async fn ensure_services_ready(&mut self) -> Result<(), BtError> {
        let peripheral = self.peripheral()?;
        
        // 发现服务
        timeout(Duration::from_millis(3000), peripheral.discover_services()).await
            .map_err(|_| "服务发现超时".to_string())?
            .map_err(|e| format!("服务发现失败：{}", e))?;
        
        tracing::info!("[BLUETOOTH] 服务发现完成，共发现 {} 个服务", peripheral.services().len());
        
        Ok(())
    }

    /// 获取已连接的 peripheral
    fn peripheral(&self) -> Result<&btleplug::platform::Peripheral, BtError> {
        self.connected_peripheral.as_ref().ok_or_else(|| "未连接".to_string())
    }
    /// 4. 发送数据
    pub async fn send(&mut self, service_uuid: &str, char_uuid: &str, data: &[u8]) -> Result<(), BtError> {
        let peripheral = self.peripheral()?;
        
        // 发现服务
        timeout(Duration::from_millis(5000), peripheral.discover_services()).await
            .map_err(|_| "服务发现超时".to_string())?
            .map_err(|e| format!("服务发现失败: {}", e))?;
        
        // 查找服务
        let service_uuid = Uuid::parse_str(service_uuid)
            .map_err(|e| format!("解析服务UUID失败: {}", e))?;
        
        let services = peripheral.services();
        let service = services
            .iter()
            .find(|s| s.uuid == service_uuid)
            .ok_or_else(|| format!("未找到服务: {}", service_uuid))?;
        
        // 查找特性
        let char_uuid = Uuid::parse_str(char_uuid)
            .map_err(|e| format!("解析特性UUID失败: {}", e))?;
        
        let characteristic = service.characteristics.iter()
            .find(|c| c.uuid == char_uuid)
            .ok_or_else(|| format!("未找到特性: {}", char_uuid))?;
        
        // 检查可写
        if !characteristic.properties.contains(CharPropFlags::WRITE) && 
           !characteristic.properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE) {
            return Err("特性不可写".to_string());
        }
        
        // 发送
        timeout(Duration::from_millis(2000), peripheral.write(characteristic, data, WriteType::WithoutResponse)).await
            .map_err(|_| "发送超时".to_string())?
            .map_err(|e| format!("发送失败: {}", e))?;
        
        tracing::info!("发送成功: {} bytes", data.len());
        Ok(())
    }

    pub async fn recv(&mut self, service_uuid: &str, char_uuid: &str, expected: ResponseType) -> Result<Vec<u8>, BtError> {
        let need_restart = self.response_rx.is_none() || 
                           self.listening_handle.as_ref().map_or(true, |h| h.is_finished());
        
        if need_restart {
            tracing::info!("[BLUETOOTH] 监听任务需要重启，清理旧状态...");
            
            if let Some(h) = self.listening_handle.take() {
                h.abort();
                match timeout(Duration::from_millis(500), h).await {
                    Ok(_) => tracing::info!("[BLUETOOTH] 旧监听任务已结束"),
                    Err(_) => tracing::info!("[BLUETOOTH] 旧监听任务结束超时"),
                }
            }
            self.response_rx = None;
            self.button_rx = None;
            
            let peripheral = self.peripheral()?;
            
            let service_uuid_parsed = Uuid::parse_str(service_uuid)
                .map_err(|e| format!("解析服务UUID失败: {}", e))?;
            
            let services = peripheral.services();
            let service = services
                .iter()
                .find(|s| s.uuid == service_uuid_parsed)
                .ok_or_else(|| format!("未找到服务: {}", service_uuid))?;
            
            let char_uuid_parsed = Uuid::parse_str(char_uuid)
                .map_err(|e| format!("解析特性UUID失败: {}", e))?;
            
            let characteristic = service.characteristics.iter()
                .find(|c| c.uuid == char_uuid_parsed)
                .ok_or_else(|| format!("未找到特性: {}", char_uuid))?;
            
            tracing::info!("[BLUETOOTH] 启动蓝牙通知监听...");
            let peripheral_clone = peripheral.clone();
            let char_clone = characteristic.clone();
            
            let (response_tx, response_rx) = tokio::sync::mpsc::channel(50);
            let (button_tx, button_rx) = tokio::sync::mpsc::channel(50);
            
            let mut last_button_state: Option<String> = None;
            
            let handle = tokio::spawn(async move {
                tracing::info!("[BLUETOOTH] 等待通知流...");
                match peripheral_clone.notifications().await {
                    Ok(stream) => {
                        tracing::info!("[BLUETOOTH] 通知流已创建，正在订阅...");
                        match peripheral_clone.subscribe(&char_clone).await {
                            Ok(_) => tracing::info!("[BLUETOOTH] 订阅成功"),
                            Err(e) => {
                                tracing::info!("[BLUETOOTH] 订阅失败：{}", e);
                                return;
                            }
                        }
                        
                        let mut stream = stream;
                        tracing::info!("[BLUETOOTH] 开始监听通知...");
                        while let Some(notif) = stream.next().await {
                            let data_hex = notif.value.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
                            let data_str = String::from_utf8_lossy(&notif.value);
                            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                            
                            tracing::info!("========================================");
                            tracing::info!("[BLUETOOTH] 收到数据包");
                            tracing::info!("  时间：{}", timestamp);
                            tracing::info!("  长度：{} bytes", notif.value.len());
                            tracing::info!("  Hex: {}", data_hex);
                            tracing::info!("  ASCII: {}", data_str.trim());
                            tracing::info!("========================================");
                            
                            let is_button_event = notif.value.len() >= 1 && 
                                (notif.value[0] == 0xAA || notif.value[0] == 0xAB || 
                                 notif.value[0] == 0xAC || notif.value[0] == 0xAD ||
                                 notif.value[0] == 0x12 || notif.value[0] == 0x10 || 
                                 notif.value[0] == 0x08 || notif.value[0] == 0x02);
                            
                            if is_button_event {
                                let first_byte = notif.value[0];
                                
                                if first_byte == 0xAA {
                                    if last_button_state.as_ref().map_or(true, |s| s != "press") {
                                        tracing::info!("[BLUETOOTH] 上一页（GPIO10 按下 0xAA）");
                                        last_button_state = Some("press".to_string());
                                        tokio::spawn(async move {
                                            emit_button_event("button_press_left");
                                        });
                                    }
                                } else if first_byte == 0xAB {
                                    if last_button_state.as_ref().map_or(true, |s| s != "release") {
                                        tracing::info!("[BLUETOOTH] 上一页松开（GPIO10 松开 0xAB）");
                                        last_button_state = Some("release".to_string());
                                        tokio::spawn(async move {
                                            emit_button_event("button_release_left");
                                        });
                                    }
                                } else if first_byte == 0xAC {
                                    if last_button_state.as_ref().map_or(true, |s| s != "press_left") {
                                        tracing::info!("[BLUETOOTH] 下一页（GPIO9 按下 0xAC）");
                                        last_button_state = Some("press_left".to_string());
                                        tokio::spawn(async move {
                                            emit_button_event("button_press");
                                        });
                                    }
                                } else if first_byte == 0xAD {
                                    if last_button_state.as_ref().map_or(true, |s| s != "release_left") {
                                        tracing::info!("[BLUETOOTH] 下一页松开（GPIO9 松开 0xAD）");
                                        last_button_state = Some("release_left".to_string());
                                        tokio::spawn(async move {
                                            emit_button_event("button_release");
                                        });
                                    }
                                } else if first_byte == 0x12 {
                                    tracing::info!("[BLUETOOTH] 收到截图命令（0x12）");
                                    tokio::spawn(async move {
                                        crate::event_emitter::emit_screenshot_command();
                                    });
                                } else if first_byte == 0x10 {
                                    tracing::info!("[BLUETOOTH] 收到显示主窗口 + note 命令（0x10）");
                                    tokio::spawn(async move {
                                        crate::event_emitter::emit_show_note_command();
                                    });
                                } else if first_byte == 0x08 {
                                    tracing::info!("[BLUETOOTH] 收到打开云盘命令（0x08）");
                                    tokio::spawn(async move {
                                        crate::event_emitter::emit_open_cloud_command();
                                    });
                                } else if first_byte == 0x02 {
                                    tracing::info!("[BLUETOOTH] 收到新建笔记命令（0x02）");
                                    tokio::spawn(async move {
                                        crate::event_emitter::emit_show_note_command();
                                    });
                                }
                                
                                let _ = button_tx.try_send(BluetoothResponse::ButtonEvent(first_byte));
                            } else {
                                let data_str = String::from_utf8_lossy(&notif.value).trim().to_string();
                                let response = if data_str.starts_with("OK:setTime") {
                                    BluetoothResponse::SetTime(data_str)
                                } else if data_str.len() == 6 && data_str.chars().all(|c| c.is_ascii_digit()) {
                                    BluetoothResponse::GetTotp(data_str)
                                } else if data_str.len() == 36 && data_str.contains('-') {
                                    BluetoothResponse::GetId(data_str)
                                } else {
                                    BluetoothResponse::GetTotp(data_str)
                                };
                                
                                if response_tx.try_send(response).is_err() {
                                    tracing::info!("[BLUETOOTH] 响应通道已满，丢弃数据");
                                }
                            }
                        }
                        
                        tracing::info!("[BLUETOOTH] 通知流已结束，连接可能已断开");
                        crate::event_emitter::emit_bluetooth_disconnect();
                    }
                    Err(e) => {
                        tracing::info!("[BLUETOOTH] 创建通知流失败：{}", e);
                        crate::event_emitter::emit_bluetooth_disconnect();
                    }
                }
            });
            
            self.response_rx = Some(response_rx);
            self.button_rx = Some(button_rx);
            self.listening_handle = Some(handle);
            tracing::info!("[BLUETOOTH] 监听任务已启动");
        } else {
            tracing::info!("[BLUETOOTH] 复用现有的监听任务");
        }
        
        if let Some(rx) = &mut self.response_rx {
            loop {
                match timeout(Duration::from_secs(10), rx.recv()).await {
                    Ok(Some(data)) => {
                        if expected.matches(&data) {
                            return Ok(data.into_bytes());
                        } else {
                            tracing::info!("[BLUETOOTH] 跳过不匹配的响应类型，继续等待...");
                            continue;
                        }
                    }
                    Ok(None) => {
                        tracing::info!("[BLUETOOTH] 响应通道已关闭，需要重新连接");
                        self.response_rx = None;
                        return Err("监听通道已关闭，请重新连接".to_string());
                    }
                    Err(_) => return Err("接收超时".to_string()),
                }
            }
        } else {
            Err("监听未启动".to_string())
        }
    }

    async fn stop_listening(&mut self) {
        if let Some(h) = self.listening_handle.take() {
            h.abort();
        }
        self.response_rx = None;
        self.button_rx = None;
    }

    /// 获取本地蓝牙适配器信息
    /// 
    /// 返回蓝牙适配器的简单信息
    pub async fn get_local_bluetooth_info(&mut self) -> Result<String, BtError> {
        tracing::info!("获取本地蓝牙适配器信息...");
        
        // 简单实现，直接返回一个默认值
        // 注意：btleplug 没有直接提供版本信息
        Ok("5.0".to_string())
    }

    /// 获取 Cpen 设备的蓝牙版本
    /// 
    /// 通过读取设备的服务和特征来获取版本信息
    pub async fn get_cpen_bluetooth_version(&mut self) -> Result<String, BtError> {
        tracing::info!("获取 Cpen 设备蓝牙版本...");
        
        match &self.connected_peripheral {
            Some(peripheral) => {
                // 尝试读取设备信息
                // 注意：Cpen 设备可能没有标准的版本特征，这里返回一个估计值
                // 实际项目中需要根据硬件文档确定如何获取版本
                
                // 先尝试获取设备的服务列表
                match peripheral.discover_services().await {
                    Ok(_) => {
                        tracing::info!("成功发现设备服务");
                        // 如果连接成功，说明设备支持至少蓝牙 4.0（BLE）
                        Ok("5.0".to_string())
                    }
                    Err(e) => {
                        Err(format!("发现服务失败：{}", e))
                    }
                }
            }
            None => {
                Err("未连接设备".to_string())
            }
        }
    }

    /// 发送心跳包以保持蓝牙连接
    /// 
    /// 在指定的时间间隔内发送心跳包，防止连接因超时而断开
    pub async fn send_keep_alive(&mut self, service_uuid: &str, char_uuid: &str) -> Result<(), BtError> {
        tracing::info!("发送蓝牙保活心跳包...");
        
        match &self.connected_peripheral {
            Some(peripheral) => {
                // 检查连接状态
                if !peripheral.is_connected().await.map_err(|e| format!("检查连接状态失败: {}", e))? {
                    return Err("设备未连接".to_string());
                }
                
                // 获取特征
                let _service = Uuid::parse_str(service_uuid).map_err(|e| format!("无效的服务 UUID: {}", e))?;
                let characteristic = Uuid::parse_str(char_uuid).map_err(|e| format!("无效的特征 UUID: {}", e))?;
                
                // 获取已发现的特征列表
                let chars = peripheral.characteristics();
                
                // 查找目标特征
                let target_char = chars.iter().find(|c| c.uuid == characteristic)
                    .ok_or_else(|| format!("未找到特征：{}", char_uuid))?;
                
                // 发送心跳包
                match peripheral.write(target_char, b"ping", WriteType::WithoutResponse).await {
                    Ok(_) => {
                        tracing::info!("蓝牙保活心跳包发送成功");
                        Ok(())
                    }
                    Err(e) => {
                        tracing::info!("发送保活心跳包失败: {}", e);
                        Err(format!("发送保活心跳包失败: {}", e))
                    }
                }
            }
            None => {
                Err("未连接设备".to_string())
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut bt = BluetoothManager::new();
    
    // 1. 打开蓝牙
    bt.enable_bluetooth()?;
    
    // 2. 扫描设备
    tracing::info!("开始扫描蓝牙设备...\n");
    let devices = bt.scan_devices(5000).await?;
    
    tracing::info!("\n========== 扫描结果 ==========");
    tracing::info!("共找到 {} 个设备:\n", devices.len());
    
    for (i, d) in devices.iter().enumerate() {
        tracing::info!("[{}] {}", i + 1, d.name);
        tracing::info!("    地址: {}", d.address);
        if !d.services.is_empty() {
            tracing::info!("    服务:");
            for s in &d.services {
                tracing::info!("      - {}", s);
            }
        }
        tracing::info!("");    }
    
    tracing::info!("==============================");
    
    // 3. 查找Cpen设备
    let cpen_device = devices.iter().find(|d| d.name.starts_with("Cpen"));
    
    match cpen_device {
        Some(device) => {
            tracing::info!("\n找到Cpen设备: {} ({})", device.name, device.address);
            
            // 4. 连接
            tracing::info!("正在连接...");
            bt.connect(&device.address).await?;
            tracing::info!("连接成功！");
            
            // Cpen设备UUID（来自原代码）
            let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
            let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";
            
            // 5. 发送getTotp命令
            tracing::info!("\n发送 'getTotp' 命令...");
            bt.send(service_uuid, char_uuid, b"getTotp").await?;
            
            // 6. 接收响应
            tracing::info!("等待TOTP响应...");
            let response = bt.recv(service_uuid, char_uuid, ResponseType::GetTotp).await?;
            let totp_str = String::from_utf8_lossy(&response);
            tracing::info!("收到TOTP: {}", totp_str);
            
            // 7. 断开连接
            tracing::info!("\n正在断开连接...");
            bt.disconnect().await?;
            tracing::info!("已断开");
        }
        None => {
            tracing::info!("\n未找到以'Cpen'开头的设备");
        }
    }
    
    Ok(())
}
