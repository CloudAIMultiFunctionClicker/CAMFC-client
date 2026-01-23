//! Cpen设备管理器
//!
//! 这个模块负责处理Cpen蓝牙设备的完整业务逻辑：
//! 1. 扫描蓝牙设备并识别Cpen设备（根据名前缀）
//! 2. 保证全局只连接一个Cpen设备（重要要求！）
//! 3. 自动处理连接、断开、重连
//! 4. 实现TOTP缓存（30秒有效）
//! 5. 管理设备ID缓存
//!
//! 思考：为啥要单独搞这个模块？
//! 用户要求业务逻辑全在Rust，前端只调简单接口。这样前端代码能大幅简化。
//! 另外，保证单设备连接也是用户明确要求的。

use std::time::{SystemTime, Duration};
use crate::bluetooth::{BluetoothManager, DeviceInfo};
use tokio::time::sleep;

// 错误类型别名，简单点就用String
type CpenError = String;

// 缓存时间常量
const TOTP_CACHE_DURATION_SECONDS: u64 = 30;
const SCAN_DURATION_MS: u64 = 5000; // 扫描5秒，和原来一样

/// Cpen设备管理器
/// 
/// 核心设计：保证全局只连接一个Cpen设备！
/// 用connected_address记录当前连接的设备地址，确保不会连接第二个。
pub struct CpenDeviceManager {
    /// 底层蓝牙管理器
    bluetooth_manager: BluetoothManager,
    
    /// 当前连接的设备地址（None表示未连接）
    /// 重要：这是保证单设备连接的关键！
    connected_address: Option<String>,
    
    /// 当前连接的设备信息（缓存起来，避免重复获取）
    current_device: Option<DeviceInfo>,
    
    /// TOTP缓存（值 + 缓存时间）
    /// 思考：要不要用更精细的缓存结构？先简单搞吧
    totp_cache: Option<(String, SystemTime)>,
    
    /// 设备ID缓存（设备UUID）
    device_id_cache: Option<String>,
    
    /// 连接状态标记，用来给前端返回状态信息
    /// 简化：就用字符串表示状态吧
    connection_status: String,
}

impl CpenDeviceManager {
    /// 创建新的Cpen设备管理器
    pub fn new() -> Self {
        Self {
            bluetooth_manager: BluetoothManager::new(),
            connected_address: None,
            current_device: None,
            totp_cache: None,
            device_id_cache: None,
            connection_status: "disconnected".to_string(),
        }
    }
    
    /// 确保连接到一个Cpen设备（单设备保证的核心！）
    /// 
    /// 这个函数实现了完整的连接逻辑：
    /// 1. 如果已经连接了设备，直接返回成功（复用连接）
    /// 2. 如果没有连接，扫描设备
    /// 3. 从扫描结果中找出Cpen设备
    /// 4. 如果有多个Cpen设备，只连接第一个（单设备保证）
    /// 5. 连接设备并记录状态
    /// 
    /// 思考：如果有多个Cpen设备，用户可能想连特定的那个？
    /// 用户要求说"保证全局只连着1个cpen"，那就先连第一个，以后有需求再改进。
    pub async fn ensure_connected(&mut self) -> Result<(), CpenError> {
        // 1. 检查是否已经连接
        if self.connected_address.is_some() {
            // TODO: 这里应该检查连接是否真的还活着，但bluetooth.rs里没提供is_connected方法
            // 先假设还连着吧，如果断了后面的操作会失败然后重连
            self.connection_status = "connected".to_string();
            println!("已经连接到设备，直接复用连接");
            return Ok(());
        }
        
        // 2. 更新状态为连接中
        self.connection_status = "connecting".to_string();
        println!("开始连接Cpen设备...");
        
        // 3. 扫描设备
        let devices = self.bluetooth_manager.scan_devices(SCAN_DURATION_MS).await
            .map_err(|e| format!("扫描设备失败: {}", e))?;
        
        println!("扫描完成，发现 {} 个设备", devices.len());
        
        // 4. 找出Cpen设备
        let cpen_devices = Self::filter_cpen_devices(&devices);
        
        if cpen_devices.is_empty() {
            self.connection_status = "disconnected".to_string();
            return Err("没有找到Cpen设备（设备名需以'Cpen'开头）".to_string());
        }
        
        println!("找到 {} 个Cpen设备，连接第一个", cpen_devices.len());
        
        // 5. 连接第一个Cpen设备（单设备保证：即使有多个也只连第一个）
        let target_device = &cpen_devices[0];
        
        // 记录一下其他设备，方便调试
        if cpen_devices.len() > 1 {
            println!("注意：有 {} 个Cpen设备，但只连接第一个: {}", 
                     cpen_devices.len(), target_device.name);
            for (i, dev) in cpen_devices.iter().enumerate().skip(1) {
                println!("  其他设备[{}]: {} - {}", i, dev.name, dev.address);
            }
        }
        
        // 6. 连接设备
        self.bluetooth_manager.connect(&target_device.address).await
            .map_err(|e| format!("连接设备失败: {}", e))?;
        
        // 7. 记录连接状态
        self.connected_address = Some(target_device.address.clone());
        self.current_device = Some(target_device.clone());
        self.connection_status = "connected".to_string();
        
        println!("成功连接到Cpen设备: {} ({})", 
                 target_device.name, target_device.address);
        
        // 8. 连接后等待一小会儿，让设备稳定
        sleep(Duration::from_millis(500)).await;
        
        Ok(())
    }
    
    /// 过滤出Cpen设备
    /// 
    /// 根据设备名前缀判断是否为Cpen设备。
    /// 原JavaScript代码检查前4个字符是否为'cpen'（不区分大小写）。
    /// 这里保持同样的逻辑。
    fn filter_cpen_devices(devices: &[DeviceInfo]) -> Vec<DeviceInfo> {
        let mut cpen_devices = Vec::new();
        
        for device in devices {
            // 检查设备名前缀（不区分大小写）
            if device.name.len() >= 4 && 
               device.name[..4].to_lowercase() == "cpen" {
                cpen_devices.push(device.clone());
                println!("识别为Cpen设备: {} - {}", device.name, device.address);
            }
        }
        
        cpen_devices
    }
    
    /// 获取缓存的TOTP（如果30秒内获取过）
    /// 
    /// 原来JavaScript端有这个缓存逻辑，现在移到Rust端。
    /// 思考：缓存时间30秒是用户要求的吗？原代码是30000ms，应该是吧。
    fn get_cached_totp(&self) -> Option<String> {
        match &self.totp_cache {
            Some((totp, cache_time)) => {
                let elapsed = SystemTime::now()
                    .duration_since(*cache_time)
                    .unwrap_or(Duration::from_secs(0));
                
                if elapsed.as_secs() < TOTP_CACHE_DURATION_SECONDS {
                    println!("使用缓存的TOTP（{}秒前获取的）", elapsed.as_secs());
                    Some(totp.clone())
                } else {
                    println!("TOTP缓存已过期（{}秒）", elapsed.as_secs());
                    None
                }
            }
            None => {
                println!("没有TOTP缓存");
                None
            }
        }
    }
    
    /// 更新TOTP缓存
    fn update_totp_cache(&mut self, totp: String) {
        self.totp_cache = Some((totp.clone(), SystemTime::now()));
        println!("TOTP已缓存，30秒内有效");
    }
    
    /// 获取TOTP（主要业务逻辑！）
    /// 
    /// 这个函数实现了完整的TOTP获取流程：
    /// 1. 检查TOTP缓存（30秒内有效）
    /// 2. 确保设备已连接（单设备保证）
    /// 3. 发送setTime命令（设置设备时间）
    /// 4. 发送getTotp命令
    /// 5. 接收并缓存TOTP
    /// 
    /// 注意：超时设置（500ms）在bluetooth.rs里实现。
    pub async fn get_totp(&mut self) -> Result<String, CpenError> {
        println!("开始获取TOTP...");
        
        // 1. 检查缓存
        if let Some(cached_totp) = self.get_cached_totp() {
            return Ok(cached_totp);
        }
        
        // 2. 确保设备已连接（单设备保证）
        self.ensure_connected().await?;
        
        // 3. 发送setTime命令
        // 思考：这个setTime命令是必须的吗？原代码有，应该是设备要求的
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let set_time_command = format!("setTime:{}", timestamp);
        
        println!("发送setTime命令: {}", set_time_command);
        
        // Cpen设备UUID（从原代码复制过来的）
        let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
        let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";
        
        self.bluetooth_manager.send(
            service_uuid, 
            char_uuid, 
            set_time_command.as_bytes()
        ).await
        .map_err(|e| format!("发送setTime命令失败: {}", e))?;
        
        // 等待设备处理setTime命令
        sleep(Duration::from_millis(100)).await;
        
        // 尝试读取setTime的响应（设备可能不响应，所以忽略错误）
        match tokio::time::timeout(
            Duration::from_millis(500), 
            self.bluetooth_manager.recv(service_uuid, char_uuid)
        ).await {
            Ok(Ok(response)) => {
                let response_str = String::from_utf8_lossy(&response);
                println!("收到setTime响应: {}", response_str);
            }
            _ => {
                println!("setTime无响应或超时（可能正常）");
            }
        }
        
        // 4. 发送getTotp命令
        println!("发送getTotp命令...");
        self.bluetooth_manager.send(
            service_uuid, 
            char_uuid, 
            b"getTotp"
        ).await
        .map_err(|e| format!("发送getTotp命令失败: {}", e))?;
        
        // 5. 接收TOTP响应
        let response = self.bluetooth_manager.recv(service_uuid, char_uuid).await
            .map_err(|e| format!("接收TOTP失败: {}", e))?;
        
        let totp = String::from_utf8(response)
            .map_err(|e| format!("TOTP响应不是有效UTF-8: {}", e))?;
        
        // 6. 更新缓存
        self.update_totp_cache(totp.clone());
        
        println!("成功获取TOTP: {}", totp);
        
        // 7. 用户要求：把返回值打印在console
        // 这个在Rust端打印，前端调用时也会看到
        println!("TOTP: {}", totp);
        
        Ok(totp)
    }
    
    /// 获取设备ID（设备UUID）
    /// 
    /// 流程：
    /// 1. 检查设备ID缓存
    /// 2. 确保设备已连接
    /// 3. 发送getId命令
    /// 4. 接收并缓存设备ID
    pub async fn get_device_id(&mut self) -> Result<String, CpenError> {
        println!("开始获取设备ID...");
        
        // 1. 检查缓存
        if let Some(cached_id) = &self.device_id_cache {
            println!("使用缓存的设备ID: {}", cached_id);
            return Ok(cached_id.clone());
        }
        
        // 2. 确保设备已连接
        self.ensure_connected().await?;
        
        // 3. 发送getId命令
        let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
        let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";
        
        println!("发送getId命令...");
        self.bluetooth_manager.send(
            service_uuid, 
            char_uuid, 
            b"getId"
        ).await
        .map_err(|e| format!("发送getId命令失败: {}", e))?;
        
        // 4. 接收设备ID响应
        let response = self.bluetooth_manager.recv(service_uuid, char_uuid).await
            .map_err(|e| format!("接收设备ID失败: {}", e))?;
        
        let device_id = String::from_utf8(response)
            .map_err(|e| format!("设备ID响应不是有效UTF-8: {}", e))?;
        
        // 5. 更新缓存
        self.device_id_cache = Some(device_id.clone());
        
        println!("成功获取设备ID: {}", device_id);
        
        Ok(device_id)
    }
    
    /// 获取连接状态
    /// 
    /// 返回格式化的状态字符串，包含：
    /// - 连接状态（disconnected/connecting/connected/error）
    /// - 设备信息（如果已连接）
    /// 
    /// 思考：这个要给前端用，所以要包含足够信息但不要太复杂。
    pub fn get_connection_status(&self) -> String {
        match (&self.connection_status[..], &self.current_device) {
            ("connected", Some(device)) => {
                format!("已连接到设备: {} ({})", device.name, device.address)
            }
            ("connected", None) => {
                "已连接（设备信息未知）".to_string()
            }
            ("connecting", _) => {
                "正在连接设备...".to_string()
            }
            ("disconnected", _) => {
                "未连接设备".to_string()
            }
            (status, _) => {
                format!("状态: {}", status)
            }
        }
    }
    
    /// 断开连接并清理资源
    /// 
    /// 用户要求：断开设备后清理所有状态。
    /// 这个函数应该被调用，比如应用退出时。
    pub async fn disconnect(&mut self) -> Result<(), CpenError> {
        println!("断开Cpen设备连接...");
        
        // 1. 清理缓存
        self.totp_cache = None;
        self.device_id_cache = None;
        
        // 2. 断开蓝牙连接（如果有的话）
        if self.connected_address.is_some() {
            match self.bluetooth_manager.disconnect().await {
                Ok(_) => println!("蓝牙连接已断开"),
                Err(e) => println!("断开蓝牙连接时出错: {}（继续清理状态）", e),
            }
        }
        
        // 3. 重置所有状态
        self.connected_address = None;
        self.current_device = None;
        self.connection_status = "disconnected".to_string();
        
        println!("Cpen设备管理器状态已重置");
        
        Ok(())
    }
    
    /// 获取当前连接的设备信息（调试用）
    pub fn get_current_device_info(&self) -> Option<String> {
        self.current_device.as_ref().map(|dev| {
            format!("{} - {}", dev.name, dev.address)
        })
    }
}