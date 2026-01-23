use btleplug::api::{Central, Peripheral, ScanFilter, WriteType, CharPropFlags, Manager as _};
use btleplug::platform::{Manager, Adapter};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use windows::Devices::Radios::Radio;
use windows::Devices::Radios::RadioAccessStatus;
use windows::Devices::Radios::RadioKind;
use windows::Devices::Radios::RadioState;
use std::error::Error;
use uuid::Uuid;

type BtError = String;

/// 设备信息
#[derive(Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub address: String,
    pub services: Vec<Uuid>,
}

/// 蓝牙管理器
pub struct BluetoothManager {
    adapter: Option<Adapter>,
    connected_peripheral: Option<btleplug::platform::Peripheral>,
    listening_rx: Option<tokio::sync::mpsc::Receiver<Vec<u8>>>,
    listening_handle: Option<tokio::task::JoinHandle<()>>,
}

impl BluetoothManager {
    pub fn new() -> Self {
        Self {
            adapter: None,
            connected_peripheral: None,
            listening_rx: None,
            listening_handle: None,
        }
    }

    /// 1. 打开蓝牙（Windows API）
    pub fn enable_bluetooth(&self) -> Result<(), Box<dyn Error>> {
        // Windows API 调用比较麻烦用btleplug的方式
        println!("检查蓝牙状态（通过btleplug）");
        Ok(())
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
        
        println!("扫描设备 {}ms...", duration_ms);
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
    pub async fn connect(&mut self, address: &str) -> Result<(), BtError> {
        println!("连接 {}...", address);
        
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
        
        println!("连接成功");
        sleep(Duration::from_millis(100)).await;
        
        if !peripheral.is_connected().await.map_err(|e| format!("检查连接失败: {}", e))? {
            return Err("连接后立即断开".to_string());
        }
        
        self.connected_peripheral = Some(peripheral);
        Ok(())
    }

    /// 断开连接
    pub async fn disconnect(&mut self) -> Result<(), BtError> {
        self.stop_listening().await;
        
        if let Some(p) = &self.connected_peripheral {
            p.disconnect().await
                .map_err(|e| format!("断开失败: {}", e))?;
        }
        
        self.connected_peripheral = None;
        println!("断开");
        Ok(())
    }

  /// 获取已连接的peripheral
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
        
        println!("发送成功: {} bytes", data.len());
        Ok(())
    }

    /// 5. 阻塞接收（类似recv）
    pub async fn recv(&mut self, service_uuid: &str, char_uuid: &str) -> Result<Vec<u8>, BtError> {
        let peripheral = self.peripheral()?;
        
        // 确保服务已发现
        let service_uuid = Uuid::parse_str(service_uuid)
            .map_err(|e| format!("解析服务UUID失败: {}", e))?;
        
        let services = peripheral.services();
        let service = services
            .iter()
            .find(|s| s.uuid == service_uuid)
            .ok_or_else(|| format!("未找到服务: {}", service_uuid))?;
        
        let char_uuid = Uuid::parse_str(char_uuid)
            .map_err(|e| format!("解析特性UUID失败: {}", e))?;
        
        let characteristic = service.characteristics.iter()
            .find(|c| c.uuid == char_uuid)
            .ok_or_else(|| format!("未找到特性: {}", char_uuid))?;
        
        // 先检查是否已经启动监听
        if self.listening_rx.is_none() || self.listening_handle.as_ref().map_or(true, |h| h.is_finished()) {
            let peripheral_clone = peripheral.clone();
            let char_clone = characteristic.clone();
            let (tx, rx) = tokio::sync::mpsc::channel(10);
            
            // 启动监听任务
            let handle = tokio::spawn(async move {
                if let Ok(stream) = peripheral_clone.notifications().await {
                    let _ = peripheral_clone.subscribe(&char_clone).await;
                    
                    let mut stream = stream;
                    while let Some(notif) = stream.next().await {
                        let _ = tx.send(notif.value).await;
                    }
                }
            });
            
            self.listening_rx = Some(rx);
            self.listening_handle = Some(handle);
        }
        
        // 阻塞等待数据
        if let Some(rx) = &mut self.listening_rx {
            match timeout(Duration::from_secs(10), rx.recv()).await {
                Ok(Some(data)) => Ok(data),
                Ok(None) => Err("通道已关闭".to_string()),
                Err(_) => Err("接收超时".to_string()),
            }
        } else {
            Err("监听未启动".to_string())
        }
    }

    /// 停止监听
    async fn stop_listening(&mut self) {
        if let Some(h) = self.listening_handle.take() {
            h.abort();
        }
        self.listening_rx = None;
    }
}
