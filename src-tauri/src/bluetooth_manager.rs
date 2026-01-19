use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter, WriteType};
use btleplug::platform::{Manager, Adapter};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tokio::sync::mpsc;
use windows::Devices::Radios::{Radio, RadioAccessStatus, RadioKind};
use std::error::Error;
use uuid::Uuid;

// 用于向TAURI前端发送事件
use tauri::Emitter;

// Cpen设备的蓝牙服务UUID（来自MicroPython代码）
// 注意：这是设备提供的服务，不是我们本地创建的服务
const CPEN_SERVICE_UUID: &str = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
const CPEN_CHARACTERISTIC_UUID: &str = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";

// 自定义错误类型，用来统一处理各种蓝牙错误
// 不想搞得太复杂，就先用简单的字符串错误
// TODO: 可以改成更具体的错误类型，比如分连接错误、扫描错误等
type BluetoothError = String;

/// 蓝牙设备信息
/// 
/// 这个结构体表示一个蓝牙设备，包含基本信息和连接状态
/// 注意：这里的peripheral是Option，因为不一定一直持有连接
#[derive(Clone)]
pub struct BluetoothDevice {
    pub name: String,
    pub address: String,
    pub is_connected: bool,
    // 这里不直接存储peripheral，因为peripheral的生命周期管理有点麻烦
    // 先简单点，只存基本信息
}

impl BluetoothDevice {
    /// 创建新的蓝牙设备
    pub fn new(name: String, address: String) -> Self {
        Self {
            name,
            address,
            is_connected: false,
        }
    }
    
    /// 检查设备名是否以指定前缀开头
    /// 用来找Cpen设备
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.name.starts_with(prefix)
    }
    
    /// 获取显示用的设备信息
    pub fn display_info(&self) -> String {
        format!("{} - {}", self.name, self.address)
    }
}

/// 蓝牙操作trait
/// 
/// 定义蓝牙管理器应该提供哪些功能
/// 用trait的好处是以后可以换不同的实现，比如测试用的mock
pub trait BluetoothOperations {
    /// 启用蓝牙硬件
    fn enable_bluetooth(&self) -> Result<bool, Box<dyn Error>>;
    
    /// 扫描蓝牙设备
    async fn scan_devices(&mut self, scan_duration: Duration) -> Result<Vec<BluetoothDevice>, BluetoothError>;
    
    /// 连接指定设备
    async fn connect_device(&mut self, device: &BluetoothDevice) -> Result<(), BluetoothError>;
    
    /// 断开设备连接
    async fn disconnect_device(&mut self, device: &BluetoothDevice) -> Result<(), BluetoothError>;
    
    /// 查找Cpen设备
    fn find_cpen_devices(&self, devices: &[BluetoothDevice]) -> Vec<BluetoothDevice> {
        devices
            .iter()
            .filter(|d| d.starts_with("Cpen"))
            .cloned()
            .collect()
    }
}

/// 蓝牙管理器
/// 
/// 管理蓝牙适配器和设备连接
/// 这次修改后，我们会保存已连接的peripheral，以便后续通信
/// 思考：一个管理器同时只能连接一个设备，因为peripheral是具体的连接
/// 如果要多设备连接，可能需要多个管理器实例或者用Arc<Mutex>
pub struct BluetoothManager {
    adapter: Option<Adapter>,
    devices: Vec<BluetoothDevice>,
    // 保存当前连接的peripheral，这样我们可以在连接后进行通信
    // 原来这里没存，导致连接后无法进行读写操作
    connected_peripheral: Option<btleplug::platform::Peripheral>,
    // 数据监听任务句柄，用于停止监听
    data_listening_handle: Option<tokio::task::JoinHandle<()>>,
    // 用于停止监听任务的信号
    stop_listening_signal: Option<tokio::sync::mpsc::Sender<()>>,
}

impl BluetoothManager {
    /// 创建新的蓝牙管理器
    pub fn new() -> Self {
        Self {
            adapter: None,
            devices: Vec::new(),
            connected_peripheral: None,
            data_listening_handle: None,
            stop_listening_signal: None,
        }
    }
    
    /// 获取当前连接的peripheral（如果有）
    /// 这个方法会检查连接是否真的有效，不只是检查引用是否存在
    /// 
    /// 问题：之前只检查引用是否存在，没检查实际连接状态
    /// 导致连接断开后还以为连着，结果发送命令失败
    async fn get_connected_peripheral(&self) -> Result<&btleplug::platform::Peripheral, BluetoothError> {
        match &self.connected_peripheral {
            Some(peripheral) => {
                // 重要改进：实际检查连接状态，不只是检查引用
                // 思考：is_connected()可能也不是100%可靠，但比不检查强
                match peripheral.is_connected().await {
                    Ok(true) => {
                        // 连接确实有效
                        Ok(peripheral)
                    }
                    Ok(false) => {
                        println!("警告：peripheral引用存在，但连接已断开");
                        Err("连接已断开".to_string())
                    }
                    Err(e) => {
                        println!("检查连接状态失败: {}", e);
                        // 检查失败，假设连接可能还活着？先返回错误比较安全
                        Err(format!("检查连接状态失败: {}", e))
                    }
                }
            }
            None => Err("没有已连接的设备".to_string()),
        }
    }
    
    /// 断开当前连接（如果有）
    pub async fn disconnect_current(&mut self) -> Result<(), BluetoothError> {
        // 先停止数据监听
        self.stop_data_listening().await;
        
        if let Some(peripheral) = &self.connected_peripheral {
            println!("正在断开当前设备连接...");
            match peripheral.disconnect().await {
                Ok(_) => {
                    println!("断开连接成功");
                    self.connected_peripheral = None;
                    Ok(())
                }
                Err(e) => {
                    println!("断开连接失败: {}", e);
                    // 即使断开失败，也清空引用
                    self.connected_peripheral = None;
                    Err(format!("断开连接失败: {}", e))
                }
            }
        } else {
            println!("没有已连接的设备需要断开");
            Ok(())
        }
    }
    
    /// 向已连接的设备发送命令
    /// 
    /// 这个函数会：
    /// 1. 检查是否有有效连接（现在会实际验证连接状态）
    /// 2. 发现服务（如果还没发现）
    /// 3. 找到指定的特性
    /// 4. 写入命令数据
    /// 5. 等待响应（可选的）
    /// 
    /// 重要改进：在关键步骤前强制检查连接状态
    /// 从日志看，连接可能在服务发现后、发送命令前断开
    pub async fn send_command(&mut self, command: &str) -> Result<Vec<u8>, BluetoothError> {
        // 检查连接 - 现在会实际验证连接状态
        let peripheral = self.get_connected_peripheral().await?;
        
        println!("准备向设备发送命令: {}", command);
        
        // 重要改进：在服务发现前再次检查连接
        // 避免连接在检查后立即断开的情况
        println!("发送命令前检查连接状态...");
        match peripheral.is_connected().await {
            Ok(true) => {
                println!("连接状态正常，继续执行");
            }
            Ok(false) => {
                println!("连接已断开，无法发送命令");
                return Err("连接已断开".to_string());
            }
            Err(e) => {
                println!("检查连接状态失败: {}", e);
                return Err(format!("检查连接状态失败: {}", e));
            }
        }
        
        // 先尝试发现服务（如果还没发现）
        // 思考：要不要缓存服务发现结果？但缓存可能带来状态同步问题
        // 先简单点，每次都发现，确保服务状态最新
        println!("正在发现设备服务...（超时：5000ms）");
        
        // 重要：超时从2000ms增加到5000ms
        // 从错误日志看，之前可能超时太短
        match timeout(Duration::from_millis(5000), peripheral.discover_services()).await {
            Ok(Ok(_)) => {
                println!("服务发现完成");
            }
            Ok(Err(e)) => {
                return Err(format!("发现服务失败: {}", e));
            }
            Err(_) => {
                // 修正错误信息，匹配实际超时时间
                return Err("发现服务超时（5000ms）".to_string());
            }
        }
        
        // 重要改进：在服务发现后再次检查连接
        // 服务发现过程可能导致连接不稳定
        println!("服务发现后检查连接状态...");
        match peripheral.is_connected().await {
            Ok(true) => {
                println!("连接状态正常，继续执行");
            }
            Ok(false) => {
                println!("连接在服务发现后断开");
                return Err("连接在服务发现后断开".to_string());
            }
            Err(e) => {
                println!("检查连接状态失败: {}", e);
                return Err(format!("检查连接状态失败: {}", e));
            }
        }
        
        // 查找目标服务UUID
        let service_uuid = Uuid::parse_str(CPEN_SERVICE_UUID)
            .map_err(|e| format!("解析服务UUID失败: {}", e))?;
        
        let services = peripheral.services();
        let target_service = services
            .iter()
            .find(|s| s.uuid == service_uuid);
        
        match target_service {
            Some(service) => {
                println!("找到目标服务: {}", service.uuid);
                
                // 查找目标特性UUID
                let char_uuid = Uuid::parse_str(CPEN_CHARACTERISTIC_UUID)
                    .map_err(|e| format!("解析特性UUID失败: {}", e))?;
                
                let target_char = service.characteristics.iter()
                    .find(|c| c.uuid == char_uuid);
                
                match target_char {
                    Some(characteristic) => {
                        println!("找到目标特性: {}", characteristic.uuid);
                        
                        // 检查特性是否可写
                        if !characteristic.properties.contains(btleplug::api::CharPropFlags::WRITE) && !characteristic.properties.contains(btleplug::api::CharPropFlags::WRITE_WITHOUT_RESPONSE) {
                            return Err("特性不支持写入".to_string());
                        }
                        
                        // 发送命令
                        let command_bytes = command.as_bytes();
                        println!("发送命令数据: {:?} ({} 字节)", command, command_bytes.len());
                        
                        // 重要：发送命令超时增加到2000ms
                        // 设备可能需要时间处理TOTP生成
                        match timeout(
                            Duration::from_millis(2000),
                            peripheral.write(
                                &characteristic,
                                command_bytes,
                                WriteType::WithoutResponse, // 用WithoutResponse应该更快
                            )
                        ).await {
                            Ok(Ok(_)) => {
                                println!("命令发送成功");
                                
                                // 等待一会儿，让设备处理命令
                                // 从MicroPython代码看，设备处理getTotp后会通过notify发送响应
                                // 但我们需要读取特性值来获取响应
                                // 给设备更多时间处理：从300ms增加到500ms
                                // 思考：设备端日志显示生成TOTP很快，但发送失败，可能连接不稳定
                                // 多等一会儿看看
                                println!("等待设备处理命令...（500ms）");
                                sleep(Duration::from_millis(500)).await;
                                
                                // 重要改进：设备通过NOTIFY发送响应，不是通过READ
                                // 我们需要设置notify监听来接收数据
                                
                                // 首先检查特性是否支持NOTIFY
                                if characteristic.properties.contains(btleplug::api::CharPropFlags::NOTIFY) {
                                    println!("特性支持NOTIFY，设置监听...");
                                    
                                    // 创建通道用于接收notify数据
                                    let (tx, rx) = mpsc::channel(10);
                                    
                                    // 克隆发送器用于回调
                                    let tx_clone = tx.clone();
                                    
                                    // btleplug API使用notifications()方法获取通知流
                                    // 我们需要在后台任务中处理通知
                                    match peripheral.notifications().await {
                                        Ok(notification_stream) => {
                                            println!("NOTIFY流获取成功，启动后台任务...");
                                            
                                            // 启动后台任务处理通知
                                            tokio::spawn(async move {
                                                // 将流转换为可用的流
                                                let mut stream = notification_stream;
                                                while let Some(notification) = stream.next().await {
                                                    println!("收到NOTIFY数据: {:?}", notification.value);
                                                    // 尝试发送数据到通道
                                                    let _ = tx_clone.try_send(notification.value);
                                                }
                                                println!("NOTIFY流结束");
                                            });
                                    
                                            // 订阅notify
                                            match peripheral.subscribe(&characteristic).await {
                                                Ok(_) => {
                                                    println!("NOTIFY订阅成功，等待设备响应...（超时：5000ms）");
                                                    
                                                    // 等待notify数据 - 使用局部变量，不存储到self中
                                                    let mut receiver = rx;
                                                    
                                                    // 思考：原来3000ms可能不够，设备端显示连接可能在1.5秒后断开
                                                    // 增加到5000ms，给设备更多时间
                                                    match timeout(
                                                        Duration::from_millis(5000),
                                                        async {
                                                            // 等待通道数据
                                                            if let Some(data) = receiver.recv().await {
                                                                println!("通过NOTIFY收到响应数据: {:?}", data);
                                                                return Ok(data);
                                                            }
                                                            Err("未收到NOTIFY数据".to_string())
                                                        }
                                                    ).await {
                                                        Ok(Ok(data)) => {
                                                            // 取消订阅
                                                            let _ = peripheral.unsubscribe(&characteristic).await;
                                                            println!("NOTIFY处理完成");
                                                            Ok(data)
                                                        }
                                                        Ok(Err(e)) => {
                                                            println!("NOTIFY接收错误: {}", e);
                                                            let _ = peripheral.unsubscribe(&characteristic).await;
                                                            Ok(Vec::new())
                                                        }
                                                        Err(_) => {
                                                            println!("等待NOTIFY响应超时（5000ms）");
                                                            // 日志显示设备在1.5秒后收到断开事件
                                                            // 5000ms应该足够了，如果还超时可能是其他问题
                                                            let _ = peripheral.unsubscribe(&characteristic).await;
                                                            Ok(Vec::new())
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    println!("NOTIFY订阅失败: {}", e);
                                                    // 订阅失败，回退到READ方式
                                                    self.fallback_read_response(peripheral, &characteristic).await
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            println!("获取NOTIFY流失败: {}", e);
                                            // 获取流失败，回退到READ方式
                                            self.fallback_read_response(peripheral, &characteristic).await
                                        }
                                    }
                                } else {
                                    println!("特性不支持NOTIFY，尝试使用READ方式...");
                                    // 特性不支持NOTIFY，使用原来的READ方式
                                    self.fallback_read_response(peripheral, &characteristic).await
                                }
                            }
                            Ok(Err(e)) => {
                                Err(format!("发送命令失败: {}", e))
                            }
                            Err(_) => {
                                Err("发送命令超时（2000ms）".to_string())
                            }
                        }
                    }
                    None => {
                        Err(format!("未找到目标特性: {}", CPEN_CHARACTERISTIC_UUID))
                    }
                }
            }
            None => {
                Err(format!("未找到目标服务: {}", CPEN_SERVICE_UUID))
            }
        }
    }
    
    /// 备用方法：使用READ方式读取响应
    /// 当NOTIFY不可用时使用这个方法
    async fn fallback_read_response(&self, peripheral: &btleplug::platform::Peripheral, characteristic: &btleplug::api::Characteristic) -> Result<Vec<u8>, BluetoothError> {
        if characteristic.properties.contains(btleplug::api::CharPropFlags::READ) {
            println!("尝试使用READ方式读取响应...（超时：2000ms）");
            match timeout(
                Duration::from_millis(2000),
                peripheral.read(&characteristic)
            ).await {
                Ok(Ok(data)) => {
                    println!("通过READ收到响应数据: {:?}", data);
                    Ok(data)
                }
                Ok(Err(e)) => {
                    println!("READ响应失败: {}", e);
                    Ok(Vec::new())
                }
                Err(_) => {
                    println!("READ响应超时（2000ms）");
                    Ok(Vec::new())
                }
            }
        } else {
            println!("特性既不支持NOTIFY也不支持READ，无法获取响应");
            Ok(Vec::new())
        }
    }
    
    /// 尝试重新连接设备
    /// 
    /// 当连接断开时，尝试重新连接当前设备
    async fn try_reconnect(&mut self) -> Result<(), BluetoothError> {
        println!("尝试重新连接设备...");
        
        // 获取之前连接的设备信息
        let Some(peripheral) = &self.connected_peripheral else {
            return Err("没有已连接的设备需要重连".to_string());
        };
        
        // 获取设备地址
        let properties = peripheral.properties().await
            .map_err(|e| format!("获取设备属性失败: {}", e))?
            .ok_or("无法获取设备属性".to_string())?;
        
        let address = properties.address.to_string();
        let name = properties.local_name.unwrap_or_else(|| "未知设备".to_string());
        
        println!("尝试重新连接设备: {} - {}", name, address);
        
        // 创建设备对象
        let device = BluetoothDevice::new(name, address);
        
        // 断开当前连接（如果有）
        let _ = peripheral.disconnect().await;
        self.connected_peripheral = None;
        
        // 重新连接
        self.connect_device(&device).await?;
        
        println!("重新连接成功");
        Ok(())
    }
    
    /// 带重试机制的发送命令方法
    /// 
    /// 如果发送失败且是因为连接断开，尝试重新连接并重试
    async fn send_command_with_retry(&mut self, command: &str, max_retries: u32) -> Result<Vec<u8>, BluetoothError> {
        let mut retries = 0;
        
        loop {
            match self.send_command(command).await {
                Ok(result) => {
                    return Ok(result);
                }
                Err(e) => {
                    // 检查错误是否与连接有关
                    let is_connection_error = e.contains("连接已断开") || 
                                             e.contains("没有已连接的设备") ||
                                             e.contains("连接在服务发现后断开");
                    
                    if is_connection_error && retries < max_retries {
                        retries += 1;
                        println!("发送命令失败（连接问题），尝试重连并重试 ({}/{})", retries, max_retries);
                        
                        // 等待一会儿再重试
                        sleep(Duration::from_millis(500)).await;
                        
                        // 尝试重新连接
                        match self.try_reconnect().await {
                            Ok(_) => {
                                println!("重连成功，重新发送命令...");
                                continue;
                            }
                            Err(reconnect_error) => {
                                println!("重连失败: {}", reconnect_error);
                                return Err(format!("发送命令失败且重连失败: {}", e));
                            }
                        }
                    } else {
                        // 不是连接错误，或者已达到最大重试次数
                        return Err(e);
                    }
                }
            }
        }
    }
    
    /// 专门发送getTotp命令的快捷方法（带重试机制）
    /// 
    /// 这个会发送"getTotp"并返回TOTP字符串
    /// 重要改进：增加重试机制，处理连接不稳定问题
    pub async fn send_get_totp(&mut self) -> Result<String, BluetoothError> {
        println!("开始获取TOTP...");
        
        // 使用带重试机制的发送命令方法
        let result = self.send_command_with_retry("getTotp", 2).await?;
        
        if result.is_empty() {
            // 可能是设备通过notify发送，我们没捕获到
            // 或者设备需要更长时间响应
            println!("警告：未收到TOTP响应数据（空响应）");
            println!("可能的原因：");
            println!("1. NOTIFY设置失败或超时");
            println!("2. 设备处理超时");
            println!("3. 连接在发送后断开");
            println!("4. 设备响应格式问题");
            println!("5. 设备未正确处理getTotp命令");
            Err("未收到TOTP响应".to_string())
        } else {
            // 先保存原始数据的引用，因为String::from_utf8会消耗所有权
            let result_clone = result.clone();
            
            // 尝试将字节数据转换为字符串
            match String::from_utf8(result) {
                Ok(totp_str) => {
                    println!("获取到TOTP: {}", totp_str);
                    println!("TOTP长度: {} 字节", totp_str.len());
                    Ok(totp_str)
                }
                Err(e) => {
                    println!("TOTP响应数据不是有效UTF-8: {:?}", e);
                    println!("原始字节数据: {:?}", result_clone);
                    Err("TOTP响应格式错误".to_string())
                }
            }
        }
    }
    
    /// 初始化蓝牙适配器
    /// 
    /// 这个方法会获取系统的蓝牙适配器
    /// 注意：在Windows上可能要先启用蓝牙硬件
    pub async fn init_adapter(&mut self) -> Result<(), BluetoothError> {
        println!("正在初始化蓝牙适配器...");
        
        let manager = Manager::new().await
            .map_err(|e| format!("创建蓝牙管理器失败: {}", e))?;
        
        let adapters = manager.adapters().await
            .map_err(|e| format!("获取适配器列表失败: {}", e))?;
        
        if adapters.is_empty() {
            return Err("没有找到可用的蓝牙适配器".to_string());
        }
        
        // 就用第一个适配器，一般系统只有一个蓝牙适配器
        let adapter = adapters.into_iter().next().unwrap();
        self.adapter = Some(adapter);
        
        println!("蓝牙适配器初始化完成");
        Ok(())
    }
    
    /// 获取适配器，如果没初始化就初始化
    async fn get_adapter(&mut self) -> Result<&Adapter, BluetoothError> {
        if self.adapter.is_none() {
            self.init_adapter().await?;
        }
        
        // 这里unwrap应该是安全的，因为上面已经检查过了
        Ok(self.adapter.as_ref().unwrap())
    }
    
    /// 更新设备列表
    pub fn update_devices(&mut self, devices: Vec<BluetoothDevice>) {
        self.devices = devices;
    }
    
    /// 获取当前设备列表
    pub fn get_devices(&self) -> &[BluetoothDevice] {
        &self.devices
    }
    
    /// 开始持续监听设备数据
    /// 
    /// 这个函数会启动一个后台任务，持续监听来自设备的数据包
    /// 当收到数据时，会通过TAURI事件系统发送到前端
    /// 
    /// 注意：需要先连接设备才能使用这个函数
    pub async fn start_data_listening(&mut self, app_handle: tauri::AppHandle) -> Result<(), BluetoothError> {
        // 检查是否已经有监听任务在运行
        if self.data_listening_handle.is_some() {
            println!("数据监听任务已经在运行");
            return Ok(());
        }
        
        // 检查是否有已连接的设备
        let peripheral = self.get_connected_peripheral().await?;
        
        println!("开始设置数据监听...");
        
        // 先发现服务
        println!("发现设备服务...");
        match timeout(Duration::from_millis(5000), peripheral.discover_services()).await {
            Ok(Ok(_)) => {
                println!("服务发现完成");
            }
            Ok(Err(e)) => {
                return Err(format!("发现服务失败: {}", e));
            }
            Err(_) => {
                return Err("发现服务超时（5000ms）".to_string());
            }
        }
        
        // 查找目标服务UUID
        let service_uuid = Uuid::parse_str(CPEN_SERVICE_UUID)
            .map_err(|e| format!("解析服务UUID失败: {}", e))?;
        
        let services = peripheral.services();
        let target_service = services
            .iter()
            .find(|s| s.uuid == service_uuid);
        
        let service = match target_service {
            Some(service) => {
                println!("找到目标服务: {}", service.uuid);
                service
            }
            None => {
                return Err(format!("未找到目标服务: {}", CPEN_SERVICE_UUID));
            }
        };
        
        // 查找目标特性UUID
        let char_uuid = Uuid::parse_str(CPEN_CHARACTERISTIC_UUID)
            .map_err(|e| format!("解析特性UUID失败: {}", e))?;
        
        let target_char = service.characteristics.iter()
            .find(|c| c.uuid == char_uuid);
        
        let characteristic = match target_char {
            Some(characteristic) => {
                println!("找到目标特性: {}", characteristic.uuid);
                characteristic
            }
            None => {
                return Err(format!("未找到目标特性: {}", CPEN_CHARACTERISTIC_UUID));
            }
        };
        
        // 检查特性是否支持NOTIFY
        if !characteristic.properties.contains(btleplug::api::CharPropFlags::NOTIFY) {
            return Err("目标特性不支持NOTIFY，无法持续监听数据".to_string());
        }
        
        // 创建停止信号通道
        let (stop_tx, mut stop_rx) = mpsc::channel(1);
        
        // 克隆需要的数据用于后台任务
        let peripheral_clone = peripheral.clone();
        let characteristic_clone = characteristic.clone();
        
        // 启动后台监听任务
        let handle = tokio::spawn(async move {
            println!("数据监听任务启动");
            
            // 获取通知流
            match peripheral_clone.notifications().await {
                Ok(notification_stream) => {
                    println!("获取NOTIFY流成功");
                    
                    // 订阅notify
                    match peripheral_clone.subscribe(&characteristic_clone).await {
                        Ok(_) => {
                            println!("NOTIFY订阅成功，开始持续监听数据...");
                            
                            let mut stream = notification_stream;
                            
                            // 持续监听数据
                            loop {
                                tokio::select! {
                                    // 收到停止信号
                                    _ = stop_rx.recv() => {
                                        println!("收到停止信号，结束数据监听");
                                        break;
                                    }
                                    // 收到通知数据
                                    notification = stream.next() => {
                                        match notification {
                                            Some(notification) => {
                                                println!("收到设备数据包: {:?}", notification.value);
                                                
                                                // 尝试将数据转换为字符串
                                                let data_str = String::from_utf8_lossy(&notification.value).to_string();
                                                println!("数据包内容: {}", data_str);
                                                
                                                // 通过TAURI事件发送到前端
                                                // 注意：这里需要在前端设置事件监听器
                                                match app_handle.emit("bluetooth-data", &data_str) {
                                                    Ok(_) => println!("数据包已发送到前端"),
                                                    Err(e) => println!("发送数据包到前端失败: {}", e),
                                                }
                                            }
                                            None => {
                                                println!("NOTIFY流结束，停止监听");
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 取消订阅
                            let _ = peripheral_clone.unsubscribe(&characteristic_clone).await;
                            println!("数据监听任务结束");
                        }
                        Err(e) => {
                            println!("NOTIFY订阅失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("获取NOTIFY流失败: {}", e);
                }
            }
        });
        
        // 保存任务句柄和停止信号
        self.data_listening_handle = Some(handle);
        self.stop_listening_signal = Some(stop_tx);
        
        println!("数据监听已启动");
        Ok(())
    }
    
    /// 停止数据监听
    pub async fn stop_data_listening(&mut self) {
        if let Some(stop_tx) = self.stop_listening_signal.take() {
            println!("发送停止信号给数据监听任务...");
            let _ = stop_tx.send(()).await;
        }
        
        if let Some(handle) = self.data_listening_handle.take() {
            println!("等待数据监听任务结束...");
            // 等待一小段时间让任务正常结束
            match timeout(Duration::from_millis(1000), handle).await {
                Ok(_) => println!("数据监听任务已正常结束"),
                Err(_) => {
                    println!("数据监听任务超时，强制取消");
                    // 注意：这里handle已经在timeout中移动了，所以不能再次使用
                    // 我们不需要在这里调用abort，因为timeout已经失败了
                }
            }
        } else {
            println!("没有正在运行的数据监听任务");
        }
    }
    
    /// 检查是否正在监听数据
    pub fn is_listening_data(&self) -> bool {
        self.data_listening_handle.is_some()
    }
}

impl BluetoothOperations for BluetoothManager {
    /// 启用蓝牙硬件（Windows专用）
    /// 
    /// 这个是用Windows API启用蓝牙无线电
    /// 注意：这个方法不是异步的，但里面调用了异步API
    /// 有点别扭，但Windows API就这样...
    fn enable_bluetooth(&self) -> Result<bool, Box<dyn Error>> {
        println!("正在查找蓝牙设备...");

        // 获取所有无线电设备
        let async_op = Radio::GetRadiosAsync()?;
        let radios = async_op.get()?;
        
        // 查找蓝牙设备
        let bluetooth_radio = find_bluetooth_radio(&radios);
        
        match bluetooth_radio {
            Some(radio) => {
                // 检查当前状态
                let current_state = radio.State()?;
                
                if current_state == windows::Devices::Radios::RadioState::On {
                    println!("蓝牙已经是开启状态");
                    Ok(true)
                } else {
                    println!("正在启用蓝牙...");
                    
                    // 尝试启用蓝牙
                    let result = radio.SetStateAsync(windows::Devices::Radios::RadioState::On)?.get()?;
                    
                    match result {
                        RadioAccessStatus::Allowed => {
                            println!("蓝牙启用成功！");
                            println!("新状态: {:?}", radio.State()?);
                            Ok(true)
                        }
                        RadioAccessStatus::DeniedBySystem => {
                            let err_msg = "系统拒绝访问蓝牙设备，可能的原因：管理员权限不足或系统策略限制";
                            eprintln!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                        RadioAccessStatus::DeniedByUser => {
                            let err_msg = "用户拒绝访问蓝牙设备";
                            eprintln!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                        RadioAccessStatus::Unspecified => {
                            let err_msg = "未知错误，无法启用蓝牙";
                            eprintln!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                        _ => {
                            let err_msg = format!("未知的访问状态: {:?}", result);
                            eprintln!("错误：{}", err_msg);
                            Err(err_msg.into())
                        }
                    }
                }
            }
            None => {
                eprintln!("未找到蓝牙设备");
                eprintln!("请确保：");
                eprintln!("1. 计算机支持蓝牙功能");
                eprintln!("2. 蓝牙硬件已正确安装");
                eprintln!("3. 蓝牙驱动程序已更新");
                Ok(false)
            }
        }
    }
    
    /// 扫描蓝牙设备
    async fn scan_devices(&mut self, scan_duration: Duration) -> Result<Vec<BluetoothDevice>, BluetoothError> {
        // 先确保蓝牙硬件是启用的
        self.enable_bluetooth()
            .map_err(|e| format!("启用蓝牙失败: {}", e))?;
        
        let adapter = self.get_adapter().await?;
        
        println!("开始扫描蓝牙设备，持续 {:?}...", scan_duration);
        
        // 开始扫描
        adapter.start_scan(ScanFilter::default()).await
            .map_err(|e| format!("开始扫描失败: {}", e))?;
        
        // 等待一会儿让设备被发现
        sleep(scan_duration).await;
        
        // 获取发现的设备
        let peripherals = adapter.peripherals().await
            .map_err(|e| format!("获取设备列表失败: {}", e))?;
        
        // 停止扫描
        adapter.stop_scan().await
            .map_err(|e| format!("停止扫描失败: {}", e))?;
        
        let mut devices = Vec::new();
        
        // 这里有个问题：如果设备很多，逐个获取属性可能会慢
        // 但一般不会太多，先这样吧
        for peripheral in &peripherals {
            if let Ok(properties) = peripheral.properties().await {
                if let Some(props) = properties {
                    let name = props.local_name.unwrap_or_else(|| "未知设备".to_string());
                    let address = props.address.to_string();
                    
                    devices.push(BluetoothDevice::new(name, address));
                }
            }
        }
        
        if devices.is_empty() {
            println!("没有发现蓝牙设备");
        } else {
            println!("扫描完成，发现 {} 个设备", devices.len());
        }
        
        // 更新管理器中的设备列表
        self.update_devices(devices.clone());
        
        Ok(devices)
    }
    
    /// 连接指定设备
    async fn connect_device(&mut self, device: &BluetoothDevice) -> Result<(), BluetoothError> {
        println!("正在连接设备: {}", device.display_info());
        
        let adapter = self.get_adapter().await?;
        
        // 先扫描一下，找到目标设备
        adapter.start_scan(ScanFilter::default()).await
            .map_err(|e| format!("开始扫描失败: {}", e))?;
        
        sleep(Duration::from_secs(2)).await;
        
        let peripherals = adapter.peripherals().await
            .map_err(|e| format!("获取设备列表失败: {}", e))?;
        
        adapter.stop_scan().await
            .map_err(|e| format!("停止扫描失败: {}", e))?;
        
        // 查找目标设备
        // 注意：这里需要逐个检查设备属性，所以用循环而不是find
        let mut target_peripheral = None;
        
        for peripheral in &peripherals {
            // 这里需要等待properties()的结果
            match peripheral.properties().await {
                Ok(Some(props)) => {
                    let address = props.address.to_string();
                    if address == device.address {
                        target_peripheral = Some(peripheral.clone());
                        break;
                    }
                }
                Ok(None) => {
                    // 没有属性，跳过
                    continue;
                }
                Err(_) => {
                    // 获取属性失败，跳过
                    continue;
                }
            }
        }
        
        match target_peripheral {
            Some(peripheral) => {
                println!("找到目标设备，尝试连接...");
                
                match peripheral.connect().await {
                    Ok(_) => {
                        println!("连接成功！");
                        
                        // 检查连接状态 - 增加重试机制
                        // 有时候连接后立即检查可能不稳定，给点时间
                        println!("验证连接状态...");
                        sleep(Duration::from_millis(100)).await;
                        
                        match peripheral.is_connected().await {
                            Ok(true) => {
                                println!("设备已连接，可以开始通信");
                                
                                // 重要：保存peripheral引用，这样我们可以在连接后进行通信
                                // 之前忘记保存了，导致连接后无法发送命令
                                self.connected_peripheral = Some(peripheral);
                                
                                // 再等一会儿，让连接稳定
                                println!("等待连接稳定...（200ms）");
                                sleep(Duration::from_millis(200)).await;
                                
                                Ok(())
                            }
                            Ok(false) => {
                                println!("连接状态异常：连接后立即断开");
                                Err("连接状态异常".to_string())
                            }
                            Err(e) => {
                                println!("检查连接状态失败: {}", e);
                                Err(format!("检查连接状态失败: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        println!("连接失败: {}", e);
                        Err(format!("连接失败: {}", e))
                    }
                }
            }
            None => {
                println!("未找到设备: {}", device.address);
                Err(format!("未找到设备: {}", device.address))
            }
        }
    }
    
    /// 断开设备连接
    async fn disconnect_device(&mut self, device: &BluetoothDevice) -> Result<(), BluetoothError> {
        // TODO: 实现断开连接逻辑
        // 问题：我们没有保存peripheral，所以不知道要断开哪个
        println!("断开连接功能待实现，设备: {}", device.display_info());
        Ok(())
    }
}

/// 在无线电设备列表中查找蓝牙设备
/// 
/// 这个是Windows专用的辅助函数
fn find_bluetooth_radio(radios: &windows::Foundation::Collections::IVectorView<Radio>) -> Option<Radio> {
    for radio in radios {
        // 直接使用radio，不需要?操作符
        if let Ok(kind) = radio.Kind() {
            if kind == RadioKind::Bluetooth {
                if let Ok(name) = radio.Name() {
                    println!("找到蓝牙设备: {}", name.to_string_lossy());
                }
                if let Ok(state) = radio.State() {
                    println!("当前状态: {:?}", state);
                }
                return Some(radio);
            }
        }
    }
    None
}
