use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Manager, Adapter};
use std::time::Duration;
use tokio::time::sleep;
use windows::Devices::Radios::{Radio, RadioAccessStatus, RadioKind};
use std::error::Error;

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
/// 思考：要不要用Arc<Mutex>？可能多个地方要用到同一个管理器
/// 先简单实现，后面再看
pub struct BluetoothManager {
    adapter: Option<Adapter>,
    devices: Vec<BluetoothDevice>,
    // 这里有个问题：btleplug的Peripheral不能跨await保存？
    // 先不存，每次需要时重新获取
}

impl BluetoothManager {
    /// 创建新的蓝牙管理器
    pub fn new() -> Self {
        Self {
            adapter: None,
            devices: Vec::new(),
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
                        
                        // 检查连接状态
                        if peripheral.is_connected().await
                            .map_err(|e| format!("检查连接状态失败: {}", e))?
                        {
                            println!("设备已连接，可以开始通信");
                            Ok(())
                        } else {
                            Err("连接状态异常".to_string())
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
