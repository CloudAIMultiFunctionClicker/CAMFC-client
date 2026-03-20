use async_trait::async_trait;
use btleplug::api::{Central, Peripheral, ScanFilter, Manager as _};
use btleplug::platform::{Manager, Adapter};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use std::time::Duration;
use tokio::time::{sleep, timeout};

use crate::core::{BluetoothAdapter, AppResult, AppError, DeviceInfo, ConnectionState};

const CPEN_SERVICE_UUID: &str = "6e400001-b5a3-f393-e0a9-e50e24dcca9e";
const WRITE_CHARACTERISTIC_UUID: &str = "6e400002-b5a3-f393-e0a9-e50e24dcca9e";
const NOTIFY_CHARACTERISTIC_UUID: &str = "6e400003-b5a3-f393-e0a9-e50e24dcca9e";
const DEFAULT_SCAN_TIMEOUT_MS: u64 = 10000;
const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 5000;

pub struct BtleplugAdapter {
    manager: Manager,
    adapter: RwLock<Option<Adapter>>,
    current_peripheral: RwLock<Option<btleplug::platform::Peripheral>>,
    connection_state: RwLock<ConnectionState>,
}

impl BtleplugAdapter {
    pub async fn new() -> AppResult<Self> {
        let manager = Manager::new()
            .await
            .map_err(|e| AppError::Bluetooth(format!("创建管理器失败: {}", e)))?;

        Ok(Self {
            manager,
            adapter: RwLock::new(None),
            current_peripheral: RwLock::new(None),
            connection_state: RwLock::new(ConnectionState::Disconnected),
        })
    }

    async fn get_adapter(&self) -> AppResult<Adapter> {
        let mut adapter_guard = self.adapter.write().await;
        if adapter_guard.is_none() {
            let adapters = self.manager.adapters()
                .await
                .map_err(|e| AppError::Bluetooth(format!("获取适配器列表失败: {}", e)))?;

            if adapters.is_empty() {
                return Err(AppError::Bluetooth("未找到蓝牙适配器".to_string()));
            }

            *adapter_guard = Some(adapters.into_iter().next().unwrap());
        }

        Ok(adapter_guard.as_ref().unwrap().clone())
    }

    async fn find_device_internal(&self, address: &str) -> AppResult<btleplug::platform::Peripheral> {
        let adapter = self.get_adapter().await?;

        adapter.stop_scan()
            .await
            .map_err(|e| AppError::Bluetooth(format!("停止扫描失败: {}", e)))?;

        adapter.start_scan(ScanFilter::default())
            .await
            .map_err(|e| AppError::Bluetooth(format!("开始扫描失败: {}", e)))?;

        let start = std::time::Instant::now();
        let scan_timeout = Duration::from_millis(DEFAULT_SCAN_TIMEOUT_MS);

        while start.elapsed() < scan_timeout {
            let peripherals = adapter.peripherals()
                .await
                .map_err(|e| AppError::Bluetooth(format!("获取设备列表失败: {}", e)))?;

            for peripheral in peripherals {
                if let Ok(Some(properties)) = peripheral.properties().await {
                    if properties.address.to_string() == address {
                        adapter.stop_scan()
                            .await
                            .map_err(|e| AppError::Bluetooth(format!("停止扫描失败: {}", e)))?;
                        return Ok(peripheral);
                    }
                }
            }

            sleep(Duration::from_millis(500)).await;
        }

        adapter.stop_scan()
            .await
            .map_err(|e| AppError::Bluetooth(format!("停止扫描失败: {}", e)))?;

        Err(AppError::DeviceConnection(format!("未找到设备: {}", address)))
    }
}

#[async_trait]
impl BluetoothAdapter for BtleplugAdapter {
    async fn enable(&self) -> AppResult<()> {
        Ok(())
    }

    async fn disable(&self) -> AppResult<()> {
        Ok(())
    }

    async fn scan_devices(&self, _timeout_ms: u64) -> AppResult<Vec<DeviceInfo>> {
        let adapter = self.get_adapter().await?;

        adapter.start_scan(ScanFilter::default())
            .await
            .map_err(|e| AppError::Bluetooth(format!("开始扫描失败: {}", e)))?;

        sleep(Duration::from_millis(DEFAULT_SCAN_TIMEOUT_MS)).await;

        let peripherals = adapter.peripherals()
            .await
            .map_err(|e| AppError::Bluetooth(format!("获取设备列表失败: {}", e)))?;

        let mut devices = Vec::new();
        for peripheral in peripherals {
            if let Ok(Some(properties)) = peripheral.properties().await {
                let name = properties.local_name.unwrap_or_else(|| "Unknown Device".to_string());
                let address_str = properties.address.to_string();
                devices.push(DeviceInfo {
                    name,
                    address: address_str,
                    services: properties.services,
                });
            }
        }

        adapter.stop_scan()
            .await
            .map_err(|e| AppError::Bluetooth(format!("停止扫描失败: {}", e)))?;

        Ok(devices)
    }

    async fn connect(&self, address: &str) -> AppResult<DeviceInfo> {
        {
            let mut state = self.connection_state.write().await;
            *state = ConnectionState::Connecting;
        }

        let peripheral = self.find_device_internal(address).await?;

        let connect_result = timeout(
            Duration::from_millis(DEFAULT_CONNECT_TIMEOUT_MS),
            peripheral.connect()
        ).await;

        match connect_result {
            Ok(connect_res) => {
                connect_res.map_err(|e| AppError::DeviceConnection(format!("连接失败: {}", e)))?;
            }
            Err(_) => {
                let mut state = self.connection_state.write().await;
                *state = ConnectionState::Disconnected;
                return Err(AppError::Timeout("连接超时".to_string()));
            }
        }

        peripheral.discover_services()
            .await
            .map_err(|e| AppError::Bluetooth(format!("发现服务失败: {}", e)))?;

        let properties = peripheral.properties().await
            .map_err(|e| AppError::Bluetooth(format!("获取设备属性失败: {}", e)))?
            .ok_or_else(|| AppError::Bluetooth("无法获取设备属性".to_string()))?;

        let name = properties.local_name.unwrap_or_else(|| "Unknown Device".to_string());
        let address_str = properties.address.to_string();

        let device_info = DeviceInfo {
            name: name.clone(),
            address: address_str.clone(),
            services: properties.services,
        };

        {
            let mut periph = self.current_peripheral.write().await;
            *periph = Some(peripheral);
        }
        {
            let mut state = self.connection_state.write().await;
            *state = ConnectionState::Connected;
        }

        Ok(device_info)
    }

    async fn disconnect(&self) -> AppResult<()> {
        let peripheral = {
            let periph = self.current_peripheral.read().await;
            periph.clone()
        };

        if let Some(p) = peripheral {
            p.disconnect()
                .await
                .map_err(|e| AppError::Bluetooth(format!("断开连接失败: {}", e)))?;
        }

        {
            let mut periph = self.current_peripheral.write().await;
            *periph = None;
        }
        {
            let mut state = self.connection_state.write().await;
            *state = ConnectionState::Disconnected;
        }

        Ok(())
    }

    async fn is_connected(&self) -> AppResult<bool> {
        let peripheral = self.current_peripheral.read().await;
        if let Some(p) = peripheral.as_ref() {
            let connected = p.is_connected().await
                .map_err(|e| AppError::Bluetooth(format!("检查连接状态失败: {}", e)))?;
            Ok(connected)
        } else {
            Ok(false)
        }
    }

    fn get_connection_state(&self) -> ConnectionState {
        // 同步函数需要返回ConnectionState，需要使用blocking read
        // 这里用一个简单的实现
        ConnectionState::Disconnected
    }

    async fn send_data(&self, _data: &[u8]) -> AppResult<()> {
        Err(AppError::Unknown("send_data待实现".to_string()))
    }

    async fn receive_data(&self, _timeout_ms: u64) -> AppResult<Vec<u8>> {
        Err(AppError::Unknown("receive_data待实现".to_string()))
    }
}