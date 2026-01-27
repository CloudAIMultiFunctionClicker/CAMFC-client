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
//! 计划业务逻辑全在Rust，前端只调简单接口。这样前端代码能大幅简化。
//! 另外，保证单设备连接也是用户明确要求的。

use std::time::{SystemTime, Duration};
use crate::bluetooth::{BluetoothManager, DeviceInfo};
use tokio::time::sleep;

// 错误类型别名，简单点就用String
type CpenError = String;

// 缓存时间常量
const TOTP_CACHE_DURATION_SECONDS: u64 = 30;
const SCAN_DURATION_MS: u64 = 5000; // 扫描3秒

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
    /// 1. 在一切最开始，检查蓝牙是否开启，如果没开就尝试开启
    /// 2. 如果已经连接了设备，直接返回成功（复用连接）
    /// 3. 如果没有连接，扫描设备
    /// 4. 从扫描结果中找出Cpen设备
    /// 5. 如果有多个Cpen设备，只连接第一个（单设备保证）
    /// 6. 连接设备并记录状态
    /// 
    /// 思考：用户要求"在一切的一切最开始 应当监测蓝牙是否开启 如果不是 就开"
    /// 所以这里要在扫描设备之前先检查蓝牙状态
    pub async fn ensure_connected(&mut self) -> Result<(), CpenError> {
        println!("开始Cpen设备连接流程...");
        
        // ==== 新增：在一切最开始检查蓝牙状态 ====
        // 用户要求：在尝试扫描设备之前检查并且开启
        println!("=== 蓝牙状态检查开始 ===");
        
        // 方法1：先尝试用Windows API检查蓝牙状态
        // 注意：enable_bluetooth是同步方法，但我们在异步上下文中
        // 可以用tokio::task::spawn_blocking或者直接调用
        match self.bluetooth_manager.enable_bluetooth() {
            Ok(_) => {
                println!("✅ 蓝牙状态检查通过（Windows API）");
            }
            Err(e) => {
                // Windows API检查失败，可能是API不可用或权限问题
                // 尝试用btleplug的fallback方法
                println!("⚠️ Windows蓝牙API检查失败，尝试用btleplug检测: {}", e);
                
                match self.bluetooth_manager.check_bluetooth_via_btleplug().await {
                    Ok(_) => {
                        println!("✅ 蓝牙状态检查通过（btleplug fallback）");
                    }
                    Err(btleplug_err) => {
                        // 两个方法都失败了，蓝牙可能真的不可用
                        let err_msg = format!("蓝牙检测失败，请确保蓝牙已开启并可用。Windows API错误: {}, btleplug错误: {}", e, btleplug_err);
                        println!("❌ {}", err_msg);
                        return Err(err_msg);
                    }
                }
            }
        }
        
        println!("=== 蓝牙状态检查完成 ===");
        // ==== 蓝牙状态检查结束 ====
        
        // 1. 检查是否已经连接
        if self.connected_address.is_some() {
            // 新增：使用bluetooth.rs提供的is_connected方法检查连接是否真的还活着
            match self.bluetooth_manager.is_connected().await {
                Ok(true) => {
                    self.connection_status = "connected".to_string();
                    println!("已经连接到设备，连接状态正常，直接复用连接");
                    return Ok(());
                }
                Ok(false) => {
                    println!("之前记录的连接已断开，需要重新连接");
                    // 连接已断开，清理状态
                    self.connected_address = None;
                    self.current_device = None;
                    self.connection_status = "disconnected".to_string();
                }
                Err(e) => {
                    println!("检查连接状态失败，假设需要重新连接: {}", e);
                    // 检查失败，保守起见重新连接
                    self.connected_address = None;
                    self.current_device = None;
                    self.connection_status = "disconnected".to_string();
                }
            }
        }
        
        // 2. 更新状态为连接中
        self.connection_status = "connecting".to_string();
        println!("开始连接Cpen设备...");
        
        // 3. 扫描设备（现在蓝牙已经确认开启）
        println!("开始扫描蓝牙设备（蓝牙状态已确认）...");
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
        
        // 9. 注意：现在使用"提前5秒刷新"策略，不需要单独的后台任务
        // 每次调用get_totp时，如果缓存快过期了（还剩5秒）就会自动刷新
        println!("设备连接成功，TOTP刷新策略已启用（提前5秒刷新）");
        
        Ok(())
    }
    
    /// 过滤出Cpen设备
    /// 
    /// 根据设备名前缀判断是否为Cpen设备。
    /// 原JavaScript代码检查前4个字符是否为'cpen'（不区分大小写）。
    /// 这里需要正确处理UTF-8字符串，使用字符迭代而不是字节切片。
    fn filter_cpen_devices(devices: &[DeviceInfo]) -> Vec<DeviceInfo> {
        let mut cpen_devices = Vec::new();
        
        for device in devices {
            // 先检查设备名长度是否足够
            if device.name.chars().count() >= 4 {
                // 获取前4个字符并转换为小写进行比较
                let prefix: String = device.name.chars().take(4).collect();
                if prefix.to_lowercase() == "cpen" {
                    cpen_devices.push(device.clone());
                    println!("识别为Cpen设备: {} - {}", device.name, device.address);
                }
            }
        }
        
        cpen_devices
    }
    
    /// 获取缓存的TOTP（如果30秒内获取过）
    /// 
    /// 原来JavaScript端有这个缓存逻辑，现在移到Rust端。
    /// 思考：缓存时间30秒是计划的吗？原代码是50000ms，应该是吧。
    /// 
    /// 修改：现在这个方法只是检查，真正的刷新逻辑在get_totp中实现
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
    
    /// 检查TOTP缓存是否需要刷新（提前5秒刷新）
    /// 
    /// 照逻辑：每30秒重新请求TOTP
    /// 策略：当缓存还有5秒过期时，就认为需要刷新
    /// 这样get_totp调用时缓存总是新鲜的
    fn should_refresh_totp(&self) -> bool {
        match &self.totp_cache {
            Some((_, cache_time)) => {
                let elapsed = SystemTime::now()
                    .duration_since(*cache_time)
                    .unwrap_or(Duration::from_secs(0));
                
                // 如果已经过去25秒（还剩5秒过期），就需要刷新
                elapsed.as_secs() >= 25
            }
            None => {
                // 没有缓存，肯定需要获取
                true
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
    /// 1. 检查TOTP缓存是否需要刷新（提前5秒刷新策略）
    /// 2. 如果需要刷新，重新获取TOTP
    /// 3. 如果不需要刷新，返回缓存的TOTP
    /// 4. 确保设备已连接（单设备保证）
    /// 5. 发送setTime和getTotp命令
    /// 
    /// 照逻辑：每30秒重新请求TOTP
    /// 实现策略：在缓存还有5秒过期时主动刷新，这样get_totp返回的值总是新鲜的
    pub async fn get_totp(&mut self) -> Result<String, CpenError> {
        println!("===== TOTP获取开始 =====");
        
        // 1. 检查是否需要刷新TOTP（提前5秒刷新策略）
        let need_refresh = self.should_refresh_totp();
        
        // 2. 如果有缓存且不需要刷新，直接返回缓存的TOTP
        if !need_refresh {
            if let Some(cached_totp) = self.get_cached_totp() {
                println!("✅ 使用缓存的TOTP（提前5秒刷新策略，当前不需要刷新）");
                println!("📋 当前TOTP值: {}", cached_totp);
                println!("===== TOTP获取结束（缓存） =====");
                return Ok(cached_totp);
            }
        }
        
        // 3. 记录刷新原因
        if need_refresh {
            println!("🔄 TOTP刷新触发：缓存即将过期（提前5秒刷新策略）");
        } else {
            println!("🔄 TOTP刷新触发：没有缓存");
        }
        
        // 4. 检查是否已有连接，如果有则复用，避免重新扫描
        let was_already_connected = self.connected_address.is_some();
        
        if was_already_connected {
            println!("🔗 复用现有蓝牙连接，不重新扫描设备");
            // 只是确保连接还活着，不重新扫描
            match self.bluetooth_manager.is_connected().await {
                Ok(true) => {
                    println!("✅ 现有连接状态正常，直接复用");
                    self.connection_status = "connected".to_string();
                }
                _ => {
                    println!("⚠️  现有连接已断开，需要重新连接");
                    // 调用ensure_connected重新连接
                    self.ensure_connected().await?;
                }
            }
        } else {
            // 5. 确保设备已连接（单设备保证）
            println!("🔗 没有现有连接，开始连接设备...");
            self.ensure_connected().await?;
        }
        
        // 6. 发送setTime命令
        // 思考：这个setTime命令是必须的吗？原代码有，应该是设备要求的
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let set_time_command = format!("setTime:{}", timestamp);
        
        println!("📤 发送setTime命令: {}", set_time_command);
        
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
                println!("📥 收到setTime响应: {}", response_str);
            }
            _ => {
                println!("⏱️  setTime无响应或超时（可能正常）");
            }
        }
        
        // 7. 发送getTotp命令
        println!("📤 发送getTotp命令...");
        self.bluetooth_manager.send(
            service_uuid, 
            char_uuid, 
            b"getTotp"
        ).await
        .map_err(|e| format!("发送getTotp命令失败: {}", e))?;
        
        // 8. 接收TOTP响应
        let response = self.bluetooth_manager.recv(service_uuid, char_uuid).await
            .map_err(|e| format!("接收TOTP失败: {}", e))?;
        
        let totp = String::from_utf8(response)
            .map_err(|e| format!("TOTP响应不是有效UTF-8: {}", e))?;
        
        // 9. 更新缓存
        self.update_totp_cache(totp.clone());
        
        // 10. 明显输出TOTP测试信息（照逻辑）
        println!("==========================================");
        println!("✅ TOTP刷新成功！");
        println!("📋 新TOTP值: {}", totp);
        println!("⏰ 缓存时间: 30秒");
        println!("🔄 下次刷新: 25秒后（提前5秒策略）");
        println!("🔗 连接状态: {}", 
                 if was_already_connected { "复用现有连接" } else { "新建连接" });
        println!("==========================================");
        
        // 11. 计划：把返回值打印在console
        // 这个在Rust端打印，前端调用时也会看到
        println!("TOTP: {}", totp);
        println!("===== TOTP获取结束（刷新） =====");
        
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
    /// 计划：断开设备后清理所有状态。
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
    
    /// 检查是否已建立稳定连接
    /// 
    /// 这个方法会实际检查蓝牙物理连接状态，而不是仅仅检查内存中的记录
    /// 可以用来验证连接是否真的还活着，避免使用过期的连接
    pub async fn is_connected(&mut self) -> Result<bool, CpenError> {
        println!("检查Cpen设备连接状态...");
        
        // 先检查是否有记录的连接地址
        if self.connected_address.is_none() {
            println!("没有记录的连接地址");
            return Ok(false);
        }
        
        // 使用底层蓝牙管理器检查实际连接状态
        match self.bluetooth_manager.is_connected().await {
            Ok(true) => {
                println!("蓝牙物理连接正常");
                Ok(true)
            }
            Ok(false) => {
                println!("蓝牙物理连接已断开");
                // 更新内部状态以保持一致
                self.connection_status = "disconnected".to_string();
                self.connected_address = None;
                self.current_device = None;
                Ok(false)
            }
            Err(e) => {
                println!("检查蓝牙连接状态时出错: {}", e);
                // 检查失败，保守返回false
                Err(format!("检查连接状态失败: {}", e))
            }
        }
    }
    
    /// 获取当前连接的设备信息（调试用）
    pub fn get_current_device_info(&self) -> Option<String> {
        self.current_device.as_ref().map(|dev| {
            format!("{} - {}", dev.name, dev.address)
        })
    }
    
    // 注意：移除了复杂的后台任务实现
    // 改为简单的"提前5秒刷新"策略，这样更简单可靠
    // 照逻辑每30秒重新请求TOTP，我们的策略是在缓存还有5秒过期时就刷新
    // 这样get_totp方法返回的值总是新鲜的（最多25秒内的）
}
