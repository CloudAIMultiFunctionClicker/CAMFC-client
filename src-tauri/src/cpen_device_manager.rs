

use std::time::{SystemTime, Duration};
use crate::bluetooth::{BluetoothManager, DeviceInfo, ResponseType};
use tokio::time::sleep;
use totp_rs::{TOTP, Secret};

type CpenError = String;

const TOTP_CACHE_DURATION_SECONDS: u64 = 30;
const SCAN_DURATION_MS: u64 = 5000;

pub struct CpenDeviceManager {

    bluetooth_manager: BluetoothManager,

    connected_address: Option<String>,

    current_device: Option<DeviceInfo>,

    totp_cache: Option<(String, SystemTime)>,

    device_id_cache: Option<String>,

    connection_status: String,
}

impl CpenDeviceManager {

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

    fn is_debug_mode() -> bool {
        dotenv::dotenv().ok();
        std::env::var("CAMFC_DEBUG")
            .map(|v| v == "1")
            .unwrap_or(false)
    }

    fn get_debug_config() -> Option<(String, String)> {
        dotenv::dotenv().ok();
        let id = std::env::var("CAMFC_ID").ok()?;
        let key = std::env::var("CAMFC_KEY").ok()?;
        if id.is_empty() || key.is_empty() {
            return None;
        }
        Some((id, key))
    }

    fn generate_totp_locally(secret: &str) -> Result<String, CpenError> {
        tracing::info!("secret: {}", secret);
        let secret_bytes = Secret::Encoded(secret.to_string())
            .to_bytes()
            .map_err(|e| format!("密钥格式错误: {}", e))?;

        let totp = TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes,
            None,
            "CAMFC".to_string(),
        ).map_err(|e| format!("创建TOTP失败: {}", e))?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {}", e))?
            .as_secs();

        Ok(totp.generate(timestamp))
    }

    fn cleanup_connection_state(&mut self) {
        self.connected_address = None;
        self.current_device = None;
        self.totp_cache = None;
        self.device_id_cache = None;
        self.connection_status = "disconnected".to_string();
        tracing::info!("[CPEN] 连接状态已彻底清理");
    }

    pub async fn ensure_connected(&mut self) -> Result<(), CpenError> {

        if Self::is_debug_mode() {
            tracing::info!("[CPEN] DEBUG模式：跳过蓝牙连接，直接认为已连接");
            self.connected_address = Some("debug_mode_device".to_string());
            self.current_device = Some(DeviceInfo {
                name: "Cpen-Debug".to_string(),
                address: "debug_mode_device".to_string(),
                services: vec![],
            });
            self.connection_status = "connected".to_string();
            return Ok(());
        }

        tracing::info!("[CPEN] 开始Cpen设备连接流程...");

        tracing::info!("[CPEN] === 蓝牙状态检查开始 ===");

        match self.bluetooth_manager.enable_bluetooth() {
            Ok(_) => {
                tracing::info!("[CPEN] 蓝牙状态检查通过（Windows API）");
            }
            Err(e) => {
                tracing::info!("[CPEN] Windows蓝牙API检查失败，尝试用btleplug检测: {}", e);

                match self.bluetooth_manager.check_bluetooth_via_btleplug().await {
                    Ok(_) => {
                        tracing::info!("[CPEN] 蓝牙状态检查通过（btleplug fallback）");
                    }
                    Err(btleplug_err) => {
                        let err_msg = format!("蓝牙检测失败，请确保蓝牙已开启并可用。Windows API错误: {}, btleplug错误: {}", e, btleplug_err);
                        tracing::info!("[CPEN] {}", err_msg);
                        return Err(err_msg);
                    }
                }
            }
        }

        tracing::info!("[CPEN] === 蓝牙状态检查完成 ===");

        if self.connected_address.is_some() {

            match self.bluetooth_manager.is_connected().await {
                Ok(true) => {
                    self.connection_status = "connected".to_string();
                    tracing::info!("[CPEN] 已经连接到设备，连接状态正常，直接复用连接");
                    return Ok(());
                }
                Ok(false) => {
                    tracing::info!("[CPEN] 之前记录的连接已断开，清理状态后重新连接");

                    self.cleanup_connection_state();
                }
                Err(e) => {
                    tracing::info!("[CPEN] 检查连接状态失败: {}，清理状态后重新连接", e);

                    self.cleanup_connection_state();
                }
            }
        }

        self.connection_status = "connecting".to_string();
        tracing::info!("[CPEN] 开始扫描并连接Cpen设备...");

        tracing::info!("[CPEN] 开始扫描蓝牙设备（蓝牙状态已确认）...");
        let devices = self.bluetooth_manager.scan_devices(SCAN_DURATION_MS).await
            .map_err(|e| format!("扫描设备失败: {}", e))?;

        tracing::info!("[CPEN] 扫描完成，发现 {} 个设备", devices.len());

        let cpen_devices = Self::filter_cpen_devices(&devices);

        if cpen_devices.is_empty() {
            self.connection_status = "disconnected".to_string();
            return Err("没有找到Cpen设备（设备名需以'Cpen'开头）".to_string());
        }

        tracing::info!("[CPEN] 找到 {} 个Cpen设备，连接第一个", cpen_devices.len());

        let target_device = &cpen_devices[0];

        if cpen_devices.len() > 1 {
            tracing::info!("[CPEN] 注意：有 {} 个Cpen设备，但只连接第一个: {}",
                     cpen_devices.len(), target_device.name);
            for (i, dev) in cpen_devices.iter().enumerate().skip(1) {
                tracing::info!("[CPEN]   其他设备[{}]: {} - {}", i, dev.name, dev.address);
            }
        }

        self.bluetooth_manager.connect(&target_device.address).await
            .map_err(|e| format!("连接设备失败: {}", e))?;

        self.connected_address = Some(target_device.address.clone());
        self.current_device = Some(target_device.clone());
        self.connection_status = "connected".to_string();

        tracing::info!("[CPEN] 成功连接到 Cpen 设备：{} ({})",
                 target_device.name, target_device.address);

        sleep(Duration::from_millis(500)).await;

        tracing::info!("[CPEN] 等待设备服务准备就绪...");
        match self.bluetooth_manager.ensure_services_ready().await {
            Ok(_) => tracing::info!("[CPEN] 设备服务已就绪"),
            Err(e) => tracing::info!("[CPEN] 等待设备服务就绪失败：{}，继续尝试", e),
        }

        tracing::info!("[CPEN] 设备连接成功，TOTP 刷新策略已启用（提前 5 秒刷新）");

        Ok(())
    }

    fn filter_cpen_devices(devices: &[DeviceInfo]) -> Vec<DeviceInfo> {
        let mut cpen_devices = Vec::new();

        for device in devices {

            if device.name.chars().count() >= 4 {

                let prefix: String = device.name.chars().take(4).collect();
                if prefix.to_lowercase() == "cpen" {
                    cpen_devices.push(device.clone());
                    tracing::info!("识别为Cpen设备: {} - {}", device.name, device.address);
                }
            }
        }

        cpen_devices
    }

    pub async fn scan_cpen_devices(&mut self) -> Result<Vec<DeviceInfo>, CpenError> {

        if Self::is_debug_mode() {
            tracing::info!("🔧 DEBUG模式：返回模拟的Cpen设备");
            let debug_device = DeviceInfo {
                name: "Cpen-Debug".to_string(),
                address: "debug_mode_device".to_string(),
                services: vec![],
            };
            tracing::info!("✅ DEBUG模式找到 1 个Cpen设备: {}", debug_device.name);
            return Ok(vec![debug_device]);
        }

        tracing::info!("开始扫描Cpen设备列表...");

        match self.bluetooth_manager.enable_bluetooth() {
            Ok(_) => {
                tracing::info!("✅ 蓝牙状态检查通过（Windows API）");
            }
            Err(e) => {
                tracing::info!("⚠️ Windows蓝牙API检查失败，尝试用btleplug检测: {}", e);
                match self.bluetooth_manager.check_bluetooth_via_btleplug().await {
                    Ok(_) => {
                        tracing::info!("✅ 蓝牙状态检查通过（btleplug fallback）");
                    }
                    Err(btleplug_err) => {
                        let err_msg = format!("蓝牙检测失败: {}, {}", e, btleplug_err);
                        tracing::info!("❌ {}", err_msg);
                        return Err(err_msg);
                    }
                }
            }
        }

        tracing::info!("开始扫描蓝牙设备...");
        let devices = self.bluetooth_manager.scan_devices(SCAN_DURATION_MS).await
            .map_err(|e| format!("扫描设备失败: {}", e))?;

        tracing::info!("扫描完成，发现 {} 个设备", devices.len());

        let cpen_devices = Self::filter_cpen_devices(&devices);

        tracing::info!("找到 {} 个Cpen设备", cpen_devices.len());

        for (i, dev) in cpen_devices.iter().enumerate() {
            tracing::info!("  Cpen设备[{}]: {} - {}", i, dev.name, dev.address);
        }

        Ok(cpen_devices)
    }

    pub async fn scan_all_bluetooth_devices(&mut self) -> Result<Vec<DeviceInfo>, CpenError> {
        tracing::info!("开始扫描所有蓝牙设备...");

        match self.bluetooth_manager.enable_bluetooth() {
            Ok(_) => {
                tracing::info!("✅ 蓝牙状态检查通过（Windows API）");
            }
            Err(e) => {
                tracing::info!("⚠️ Windows蓝牙API检查失败，尝试用btleplug检测: {}", e);
                match self.bluetooth_manager.check_bluetooth_via_btleplug().await {
                    Ok(_) => {
                        tracing::info!("✅ 蓝牙状态检查通过（btleplug fallback）");
                    }
                    Err(btleplug_err) => {
                        let err_msg = format!("蓝牙检测失败: {}, {}", e, btleplug_err);
                        tracing::info!("❌ {}", err_msg);
                        return Err(err_msg);
                    }
                }
            }
        }

        tracing::info!("开始扫描蓝牙设备...");
        let devices = self.bluetooth_manager.scan_devices(SCAN_DURATION_MS).await
            .map_err(|e| format!("扫描设备失败: {}", e))?;

        tracing::info!("扫描完成，发现 {} 个设备", devices.len());

        for (i, dev) in devices.iter().enumerate() {
            tracing::info!("  蓝牙设备[{}]: {} - {}", i, dev.name, dev.address);
        }

        Ok(devices)
    }

    pub async fn connect_to_device(&mut self, address: &str) -> Result<DeviceInfo, CpenError> {

        if Self::is_debug_mode() {
            tracing::info!("🔧 DEBUG模式：跳过真实连接，直接设置连接状态");
            self.connected_address = Some(address.to_string());
            self.current_device = Some(DeviceInfo {
                name: "Cpen-Debug".to_string(),
                address: address.to_string(),
                services: vec![],
            });
            self.connection_status = "connected".to_string();
            tracing::info!("✅ DEBUG模式连接成功: {}", address);
            return Ok(self.current_device.clone().unwrap());
        }

        tracing::info!("开始连接到指定Cpen设备: {}", address);

        if self.connected_address.is_some() {
            tracing::info!("断开当前连接...");
            let _ = self.bluetooth_manager.disconnect().await;
            self.connected_address = None;
            self.current_device = None;
        }

        self.connection_status = "connecting".to_string();

        self.bluetooth_manager.connect(address).await
            .map_err(|e| format!("连接设备失败: {}", e))?;

        let device_info = DeviceInfo {
            name: format!("Cpen-{}", &address[address.len().saturating_sub(8)..]),
            address: address.to_string(),
            services: vec![],
        };

        self.connected_address = Some(address.to_string());
        self.current_device = Some(device_info.clone());
        self.connection_status = "connected".to_string();

        tracing::info!("成功连接到Cpen设备: {} ({})", device_info.name, address);

        sleep(Duration::from_millis(500)).await;

        Ok(device_info)
    }

    fn get_cached_totp(&mut self) -> Option<String> {
        match &self.totp_cache {
            Some((totp, cache_time)) => {
                let elapsed = SystemTime::now()
                    .duration_since(*cache_time)
                    .unwrap_or(Duration::from_secs(0));

                if elapsed.as_secs() < TOTP_CACHE_DURATION_SECONDS {

                    if Self::is_valid_totp(totp) {
                        tracing::info!("使用缓存的TOTP（{}秒前获取的）", elapsed.as_secs());
                        Some(totp.clone())
                    } else {
                        tracing::info!("缓存的TOTP无效（{}），需要刷新", totp);
                        None
                    }
                } else {
                    tracing::info!("TOTP缓存已过期（{}秒）", elapsed.as_secs());
                    None
                }
            }
            None => {
                tracing::info!("没有TOTP缓存");
                None
            }
        }
    }

    fn is_valid_totp(totp: &str) -> bool {
        totp.len() == 6 && totp.chars().all(|c| c.is_ascii_digit())
    }

    fn should_refresh_totp(&self) -> bool {
        match &self.totp_cache {
            Some((_, cache_time)) => {
                let elapsed = SystemTime::now()
                    .duration_since(*cache_time)
                    .unwrap_or(Duration::from_secs(0));

                elapsed.as_secs() >= 25
            }
            None => {

                true
            }
        }
    }

    fn update_totp_cache(&mut self, totp: String) {
        self.totp_cache = Some((totp.clone(), SystemTime::now()));
        tracing::info!("TOTP已缓存，30秒内有效");
    }

    pub async fn get_totp(&mut self) -> Result<String, CpenError> {
        tracing::info!("[CPEN] ===== TOTP获取开始 =====");

        if Self::is_debug_mode() {
            tracing::info!("[CPEN] DEBUG模式：从环境变量获取TOTP");
            if let Some((_, key)) = Self::get_debug_config() {
                match Self::generate_totp_locally(&key) {
                    Ok(totp) => {
                        tracing::info!("请求TOTP! 使用的totp：{}", totp);
                        tracing::info!("[CPEN] DEBUG模式TOTP生成成功: {}", totp);
                        tracing::info!("[CPEN] ===== TOTP获取结束（DEBUG模式） =====");
                        return Ok(totp);
                    }
                    Err(e) => {
                        tracing::info!("[CPEN] DEBUG模式TOTP生成失败: {}", e);
                        return Err(e);
                    }
                }
            } else {
                return Err("DEBUG模式需要设置CAMFC_KEY环境变量".to_string());
            }
        }

        let need_refresh = self.should_refresh_totp();

        if !need_refresh {
            if let Some(cached_totp) = self.get_cached_totp() {
                tracing::info!("[CPEN] 使用缓存的TOTP");
                tracing::info!("请求TOTP! 使用的totp：{}", cached_totp);
                tracing::info!("[CPEN] ===== TOTP获取结束（缓存） =====");
                return Ok(cached_totp);
            }
        }

        if need_refresh {
            tracing::info!("[CPEN] TOTP刷新触发：缓存即将过期");
        } else {
            tracing::info!("[CPEN] TOTP刷新触发：没有缓存");
        }

        const MAX_RETRIES: u32 = 2;
        for attempt in 1..=MAX_RETRIES {
            tracing::info!("[CPEN] TOTP获取尝试 {}/{}", attempt, MAX_RETRIES);

            match self.get_totp_once().await {
                Ok(totp) => {
                    tracing::info!("请求TOTP! 使用的totp：{}", totp);
                    tracing::info!("[CPEN] ===== TOTP获取成功 =====");
                    return Ok(totp);
                }
                Err(e) if attempt < MAX_RETRIES => {
                    tracing::info!("[CPEN] TOTP获取失败: {}，清理状态后重试", e);

                    self.cleanup_connection_state();
                    sleep(Duration::from_millis(500)).await;
                }
                Err(e) => {
                    tracing::info!("[CPEN] TOTP获取重试次数用尽: {}", e);
                    return Err(e);
                }
            }
        }

        Err("获取TOTP重试次数用尽".to_string())
    }

    async fn get_totp_once(&mut self) -> Result<String, CpenError> {

        let was_already_connected = self.connected_address.is_some();

        if was_already_connected {
            tracing::info!("[CPEN] 复用现有蓝牙连接");
            match self.bluetooth_manager.is_connected().await {
                Ok(true) => {
                    tracing::info!("[CPEN] 现有连接状态正常");
                    self.connection_status = "connected".to_string();
                }
                _ => {
                    tracing::info!("[CPEN] 现有连接已断开，重新连接");
                    self.ensure_connected().await?;
                }
            }
        } else {
            tracing::info!("[CPEN] 没有现有连接，开始连接设备");
            self.ensure_connected().await?;
        }

        let timestamp = chrono::Utc::now().timestamp().to_string();
        let set_time_command = format!("setTime:{}", timestamp);

        tracing::info!("[CPEN] 发送setTime命令: {}", set_time_command);

        let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
        let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";

        self.bluetooth_manager.send(
            service_uuid,
            char_uuid,
            set_time_command.as_bytes()
        ).await
        .map_err(|e| format!("发送 setTime 命令失败：{}", e))?;

        let _set_time_response = self.bluetooth_manager.recv(service_uuid, char_uuid, ResponseType::SetTime).await
            .map_err(|e| format!("接收 setTime 响应失败: {}", e))?;
        tracing::info!("[CPEN] setTime 响应已处理");

        tracing::info!("[CPEN] 发送getTotp命令");

        sleep(Duration::from_millis(200)).await;

        self.bluetooth_manager.send(
            service_uuid,
            char_uuid,
            b"getTotp"
        ).await
        .map_err(|e| format!("发送getTotp命令失败: {}", e))?;

        let response = self.bluetooth_manager.recv(service_uuid, char_uuid, ResponseType::GetTotp).await
            .map_err(|e| format!("接收TOTP失败: {}", e))?;

        let totp = String::from_utf8(response)
            .map_err(|e| format!("TOTP响应不是有效UTF-8: {}", e))?;

        self.update_totp_cache(totp.clone());

        tracing::info!("[CPEN] TOTP获取成功: {}", totp);

        Ok(totp)
    }

    pub async fn get_device_id(&mut self) -> Result<String, CpenError> {
        tracing::info!("开始获取设备ID...");

        if Self::is_debug_mode() {
            tracing::info!("🔧 DEBUG模式：从环境变量获取设备ID");
            if let Some((id, _)) = Self::get_debug_config() {
                tracing::info!("✅ DEBUG模式设备ID: {}", id);
                tracing::info!("请求设备ID! 使用的ID: {}", id);
                return Ok(id);
            } else {
                return Err("DEBUG模式需要设置CAMFC_ID环境变量".to_string());
            }
        }

        if let Some(cached_id) = &self.device_id_cache {
            tracing::info!("使用缓存的设备ID: {}", cached_id);
            tracing::info!("请求设备ID! 使用的ID: {}", cached_id);
            return Ok(cached_id.clone());
        }

        self.ensure_connected().await?;

        let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
        let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";

        tracing::info!("发送getId命令...");
        self.bluetooth_manager.send(
            service_uuid,
            char_uuid,
            b"getId"
        ).await
        .map_err(|e| format!("发送getId命令失败: {}", e))?;

        let response = self.bluetooth_manager.recv(service_uuid, char_uuid, ResponseType::GetId).await
            .map_err(|e| format!("接收设备ID失败: {}", e))?;

        let device_id = String::from_utf8(response)
            .map_err(|e| format!("设备ID响应不是有效UTF-8: {}", e))?;

        self.device_id_cache = Some(device_id.clone());

        tracing::info!("成功获取设备ID: {}", device_id);

        Ok(device_id)
    }

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

    pub async fn disconnect(&mut self) -> Result<(), CpenError> {
        tracing::info!("[CPEN] 断开Cpen设备连接...");

        if self.connected_address.is_some() {
            match self.bluetooth_manager.disconnect().await {
                Ok(_) => tracing::info!("[CPEN] 蓝牙连接已断开"),
                Err(e) => tracing::info!("[CPEN] 断开蓝牙连接时出错: {}（继续清理状态）", e),
            }
        }

        self.cleanup_connection_state();

        tracing::info!("[CPEN] Cpen设备管理器状态已重置");

        Ok(())
    }

    pub async fn is_connected(&mut self) -> Result<bool, CpenError> {

        if Self::is_debug_mode() {
            return Ok(true);
        }

        if self.connected_address.is_none() {
            return Ok(false);
        }

        match self.bluetooth_manager.is_connected().await {
            Ok(true) => Ok(true),
            Ok(false) => {
                self.connection_status = "disconnected".to_string();
                self.connected_address = None;
                self.current_device = None;
                Ok(false)
            }
            Err(e) => Err(format!("检查连接状态失败: {}", e))
        }
    }

    pub async fn get_local_bluetooth_version(&mut self) -> Result<String, CpenError> {
        self.bluetooth_manager.get_local_bluetooth_info().await
    }

    pub async fn get_cpen_bluetooth_version(&mut self) -> Result<String, CpenError> {
        self.bluetooth_manager.get_cpen_bluetooth_version().await
    }

    pub async fn send_keep_alive(&mut self) -> Result<(), CpenError> {

        let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
        let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";

        self.bluetooth_manager.send_keep_alive(service_uuid, char_uuid).await
    }

    pub fn get_current_device_info(&self) -> Option<String> {
        self.current_device.as_ref().map(|dev| {
            format!("{} - {}", dev.name, dev.address)
        })
    }

    pub async fn get_user_uuid(&mut self) -> Result<String, CpenError> {

        if Self::is_debug_mode() {
            tracing::info!("🔧 DEBUG模式：从环境变量获取用户UUID");
            dotenv::dotenv().ok();
            if let Ok(uuid) = std::env::var("CAMFC_UUID") {
                if !uuid.is_empty() {
                    tracing::info!("✅ DEBUG模式用户UUID: {}", uuid);
                    return Ok(uuid);
                }
            }

            if let Some((id, _)) = Self::get_debug_config() {
                tracing::info!("🔧 DEBUG模式：使用CAMFC_ID作为用户UUID: {}", id);
                return Ok(id);
            }
        }

        let device_id = self.get_device_id().await?;

        if let Some((user_uuid, _)) = device_id.split_once(':') {
            tracing::info!("从设备ID解析出用户UUID: {}", user_uuid);
            Ok(user_uuid.to_string())
        } else {

            tracing::info!("设备ID直接作为用户UUID: {}", device_id);
            Ok(device_id)
        }
    }
}
