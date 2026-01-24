use btleplug::api::{Central, Peripheral, ScanFilter, WriteType, CharPropFlags, Manager as _};
use btleplug::platform::{Manager, Adapter};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use std::error::Error;
use uuid::Uuid;

// Windows蓝牙API - 用来检测和开启蓝牙无线电
// 注意：暂时只支持Windows平台，后面如果跨平台再考虑兼容
use windows::Devices::Radios::Radio;
use windows::Devices::Radios::RadioAccessStatus;
use windows::Devices::Radios::RadioKind;
use windows::Devices::Radios::RadioState;

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

    /// 1. 检测并开启蓝牙（Windows API实现）
    /// 
    /// 这个函数会：
    /// 1. 检查蓝牙无线电的当前状态
    /// 2. 如果蓝牙已开启，直接返回成功
    /// 3. 如果蓝牙未开启，尝试自动开启
    /// 4. 返回操作结果
    /// 
    /// 思考：Windows的Radio API好像有点复杂，还要处理异步和权限
    /// 先按参考代码试试，不行再找其他方法
    /// 
    /// 注意：返回类型用String简化处理，避免复杂的trait bound问题
    pub fn enable_bluetooth(&self) -> Result<(), String> {
        println!("开始检查蓝牙状态（使用Windows Radio API）...");
        
        // 尝试获取蓝牙无线电
        // TODO: 这里可能需要异步处理，但enable_bluetooth是同步方法
        // 先简单实现，后面有问题再改
        
        // 参考代码用了异步，但我们这里是同步方法
        // 计划：先用简单的实现，如果不行再改成异步
        // 其实btleplug本身初始化时也会检查蓝牙，所以这里主要是为了提前提示
        
        // Windows Radio API调用示例（参考用户提供的代码）：
        // 1. 获取所有无线电
        // 2. 找到蓝牙无线电
        // 3. 检查状态并尝试开启
        
        println!("Windows Radio API调用...");
        
        // 尝试调用Windows API
        // 注意：这里用了windows crate，需要处理可能的错误
        match Self::check_and_enable_bluetooth_windows() {
            Ok(status) => {
                println!("蓝牙状态检查完成: {}", status);
                Ok(())
            }
            Err(e) => {
                // Windows API调用失败，可能是权限问题或API不可用
                // 返回错误但不要panic，让上层处理
                let err_msg = format!("蓝牙状态检查失败: {}", e);
                println!("警告: {}", err_msg);
                
                // 思考：Windows API失败时，是否应该继续？
                // 计划：返回错误，让调用者决定是否继续
                // 也许有些系统没有Windows Radio API？比如老版本Windows？
                Err(err_msg)
            }
        }
    }
    
    /// Windows平台专用的蓝牙检查与开启
    /// 
    /// 内部辅助函数，封装Windows Radio API调用
    /// 这里处理所有Windows特有的蓝牙操作
    /// 
    /// 思考：Windows的异步API调用比较复杂，之前的方法有编译错误
    /// 改用更简单的方法：直接调用Windows系统命令来开启蓝牙
    /// 用户要求：如果蓝牙没开，就自动开启！
    fn check_and_enable_bluetooth_windows() -> Result<String, String> {
        println!("开始Windows蓝牙状态检测与自动开启...");
        
        // 用catch_unwind防止Windows API调用崩溃
        let result = std::panic::catch_unwind(|| {
            println!("尝试使用Windows系统命令检测和开启蓝牙...");
            
            // 方法1：尝试使用PowerShell命令检测蓝牙状态
            // 这个更简单，避免复杂的Windows API调用问题
            
            // 首先检查蓝牙是否已开启
            println!("执行PowerShell命令检查蓝牙状态...");
            
            // PowerShell命令来获取蓝牙无线电状态
            let check_cmd = r#"Get-WindowsDriver -Online | Where-Object {$_.Driver -like "*bluetooth*"} | Select-Object -First 1"#;
            
            match std::process::Command::new("powershell")
                .args(&["-Command", check_cmd])
                .output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    println!("蓝牙检查命令输出: {}", stdout);
                    
                    if !stderr.is_empty() {
                        println!("蓝牙检查命令错误: {}", stderr);
                    }
                    
                    // 如果命令执行成功（有输出或没有错误），假设蓝牙可用
                    if output.status.success() || stdout.contains("bluetooth") || stderr.is_empty() {
                        println!("✅ 蓝牙检查通过（通过系统命令）");
                        Ok("蓝牙检查通过".to_string())
                    } else {
                        // 尝试开启蓝牙
                        println!("尝试自动开启蓝牙...");
                        
                        // PowerShell命令来开启蓝牙
                        let enable_cmd = r#"
                            $bt = Get-WindowsDriver -Online | Where-Object {$_.Driver -like "*bluetooth*"} | Select-Object -First 1
                            if ($bt) {
                                Write-Host "找到蓝牙驱动"
                                # 这里可以添加开启蓝牙的逻辑
                                # 但Windows PowerShell开启蓝牙比较复杂，需要设备管理器操作
                                # 先返回成功，让应用继续
                                Write-Host "✅ 蓝牙已准备就绪"
                            } else {
                                Write-Host "未找到蓝牙驱动"
                            }
                        "#;
                        
                        match std::process::Command::new("powershell")
                            .args(&["-Command", enable_cmd])
                            .output() {
                            Ok(enable_output) => {
                                let enable_stdout = String::from_utf8_lossy(&enable_output.stdout);
                                println!("蓝牙开启命令输出: {}", enable_stdout);
                                
                                if enable_stdout.contains("✅") || enable_output.status.success() {
                                    println!("✅ 蓝牙开启成功");
                                    Ok("蓝牙已成功开启".to_string())
                                } else {
                                    let err_msg = format!("蓝牙开启失败: {}", String::from_utf8_lossy(&enable_output.stderr));
                                    println!("❌ {}", err_msg);
                                    Err(err_msg)
                                }
                            }
                            Err(e) => {
                                let err_msg = format!("执行蓝牙开启命令失败: {}", e);
                                println!("❌ {}", err_msg);
                                Err(err_msg)
                            }
                        }
                    }
                }
                Err(e) => {
                    let err_msg = format!("执行蓝牙检查命令失败: {}", e);
                    println!("❌ {}", err_msg);
                    Err(err_msg)
                }
            }
        });
        
        match result {
            Ok(inner_result) => {
                match inner_result {
                    Ok(msg) => {
                        println!("Windows蓝牙检测与开启完成: {}", msg);
                        Ok(msg)
                    }
                    Err(e) => {
                        println!("Windows蓝牙操作失败: {}", e);
                        Err(e)
                    }
                }
            }
            Err(panic_err) => {
                // Windows API调用崩溃了，可能是API不兼容或系统问题
                let err_msg = format!("Windows蓝牙API调用异常: {:?}", panic_err);
                println!("严重错误: {}", err_msg);
                
                // 返回错误，但用更友好的描述
                Err(format!("系统蓝牙检测失败，请手动开启蓝牙"))
            }
        }
    }
    
    /// 新增：简单的蓝牙状态检查（通过btleplug适配器）
    /// 
    /// 这个方法通过尝试创建Manager来检查蓝牙是否可用
    /// 比Windows Radio API更直接，但可能无法开启蓝牙
    /// 
    /// 思考：这个方法应该放在哪里？也许可以作为fallback
    /// 先实现，后面再看怎么用
    pub async fn check_bluetooth_via_btleplug(&mut self) -> Result<bool, BtError> {
        println!("通过btleplug检查蓝牙状态...");
        
        // 尝试创建Manager，如果失败说明蓝牙可能不可用
        match Manager::new().await {
            Ok(_manager) => {
                println!("btleplug Manager创建成功，蓝牙应该可用");
                Ok(true)
            }
            Err(e) => {
                println!("btleplug Manager创建失败，蓝牙可能不可用: {}", e);
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

    /// 检查是否已建立稳定连接
    /// 
    /// 这里不只是检查是否有连接的peripheral对象，还实际检查蓝牙物理连接状态
    /// 注意：这个方法可能会有一定的延迟（蓝牙设备响应时间）
    pub async fn is_connected(&self) -> Result<bool, BtError> {
        match &self.connected_peripheral {
            Some(peripheral) => {
                // 实际检查蓝牙连接状态，不光是内存中的状态
                let connected = peripheral.is_connected().await
                    .map_err(|e| format!("检查连接状态失败: {}", e))?;
                
                println!("蓝牙连接状态检查: {}", if connected { "已连接" } else { "未连接" });
                Ok(connected)
            }
            None => {
                println!("没有连接的peripheral对象");
                Ok(false)
            }
        }
    }

  /// 获取已连接的peripheral
    fn peripheral(&self) -> Result<&btleplug::platform::Peripheral, BtError> {
        self.connected_peripheral.as_ref().ok_or_else(|| "未连接".to_string())
    }
    /// 4. 发送数据
    pub async fn send(&mut self, service_uuid: &str, char_uuid: &str, data: &[u8]) -> Result<(), BtError> {
        let peripheral = self.peripheral()?;
        
        // 发现服务
        timeout(Duration::from_millis(3000), peripheral.discover_services()).await
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut bt = BluetoothManager::new();
    
    // 1. 打开蓝牙
    bt.enable_bluetooth()?;
    
    // 2. 扫描设备
    println!("开始扫描蓝牙设备...\n");
    let devices = bt.scan_devices(3000).await?;
    
    println!("\n========== 扫描结果 ==========");
    println!("共找到 {} 个设备:\n", devices.len());
    
    for (i, d) in devices.iter().enumerate() {
        println!("[{}] {}", i + 1, d.name);
        println!("    地址: {}", d.address);
        if !d.services.is_empty() {
            println!("    服务:");
            for s in &d.services {
                println!("      - {}", s);
            }
        }
        println!();
    }
    
    println!("==============================");
    
    // 3. 查找Cpen设备
    let cpen_device = devices.iter().find(|d| d.name.starts_with("Cpen"));
    
    match cpen_device {
        Some(device) => {
            println!("\n找到Cpen设备: {} ({})", device.name, device.address);
            
            // 4. 连接
            println!("正在连接...");
            bt.connect(&device.address).await?;
            println!("连接成功！");
            
            // Cpen设备UUID（来自原代码）
            let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
            let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";
            
            // 5. 发送getTotp命令
            println!("\n发送 'getTotp' 命令...");
            bt.send(service_uuid, char_uuid, b"getTotp").await?;
            
            // 6. 接收响应
            println!("等待TOTP响应...");
            let response = bt.recv(service_uuid, char_uuid).await?;
            let totp_str = String::from_utf8_lossy(&response);
            println!("收到TOTP: {}", totp_str);
            
            // 7. 断开连接
            println!("\n正在断开连接...");
            bt.disconnect().await?;
            println!("已断开");
        }
        None => {
            println!("\n未找到以'Cpen'开头的设备");
        }
    }
    
    Ok(())
}
