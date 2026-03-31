# CAMFC Client Rust 部分文档

## 项目结构

```
src-tauri/
├── src/
│   ├── utils/              # 工具函数
│   │   ├── bluetooth.rs    # 蓝牙相关工具
│   │   ├── file.rs         # 文件操作工具
│   │   ├── network.rs      # 网络相关工具
│   │   └── mod.rs
│   ├── bluetooth.rs        # 蓝牙通信底层实现
│   ├── cpen_device_manager.rs  # 核心业务逻辑
│   ├── config.rs           # 配置管理
│   ├── download.rs         # 文件下载功能
│   ├── event_emitter.rs    # 事件发射
│   ├── lib.rs              # 主库文件，定义前端交互命令
│   ├── main.rs             # 应用入口
│   ├── screenshot.rs       # 屏幕截图功能
│   ├── storage.rs          # 数据存储功能
│   └── upload.rs           # 文件上传功能
├── Cargo.toml              # Rust依赖管理
└── tauri.conf.json         # Tauri配置
```

## 核心功能模块

### 1. CpenDeviceManager

**核心业务逻辑实现**，负责管理Cpen蓝牙设备的连接和TOTP获取。

**主要功能：**
- 自动扫描并识别Cpen设备（根据设备名前缀）
- 保证全局只连接一个Cpen设备
- TOTP缓存机制（30秒有效，提前5秒刷新）
- 设备ID缓存
- 自动重连和错误处理

**关键方法：**
- `get_totp()`: 获取一次性密码（核心功能）
- `get_device_id()`: 获取设备唯一标识
- `scan_cpen_devices()`: 扫描Cpen设备列表
- `connect_to_device()`: 连接指定设备
- `is_connected()`: 检查连接状态

### 2. 蓝牙通信

**底层蓝牙通信实现**，基于btleplug库。

**主要功能：**
- 设备扫描和发现
- 蓝牙连接管理
- 数据发送和接收
- 服务和特征值管理

### 3. 文件传输

**支持文件的下载和上传**，实现了分片传输和断点续传。

**下载功能：**
- 从云盘下载文件到本地
- 支持断点续传
- 下载进度查询

**上传功能：**
- 从本地选择文件上传到云盘
- 支持批量上传
- 上传进度查询

### 4. 屏幕截图

**实现屏幕截图功能**，支持多显示器。

**主要功能：**
- 截取当前屏幕
- 获取显示器列表
- 返回base64编码的图片数据

## 前端API接口

### 蓝牙相关

| 命令 | 功能 | 参数 | 返回值 |
|------|------|------|--------|
| `get_totp` | 获取TOTP码 | 无 | `Result<String, String>` |
| `get_device_id` | 获取设备ID | 无 | `Result<String, String>` |
| `scan_cpen_devices` | 扫描Cpen设备 | 无 | `Result<Vec<DeviceInfo>, String>` |
| `scan_all_bluetooth_devices` | 扫描所有蓝牙设备 | 无 | `Result<Vec<DeviceInfo>, String>` |
| `connect_cpen_device` | 连接指定设备 | `address: String` | `Result<DeviceInfo, String>` |
| `get_connection_status` | 获取连接状态 | 无 | `Result<String, String>` |
| `is_connected` | 检查是否连接 | 无 | `Result<bool, String>` |
| `disconnect` | 断开连接 | 无 | `Result<(), String>` |
| `get_local_bluetooth_version` | 获取本地蓝牙版本 | 无 | `Result<String, String>` |
| `get_cpen_bluetooth_version` | 获取Cpen设备蓝牙版本 | 无 | `Result<String, String>` |
| `send_keep_alive` | 发送保活心跳包 | 无 | `Result<String, String>` |

### 文件操作

| 命令 | 功能 | 参数 | 返回值 |
|------|------|------|--------|
| `download_file` | 下载文件 | `file_id: String` | `Result<String, String>` |
| `get_download_progress` | 获取下载进度 | `file_id: String` | `Result<serde_json::Value, String>` |
| `upload_file` | 上传文件 | `file_path: String` | `Result<String, String>` |
| `upload_files_from_paths` | 批量上传文件 | `file_paths: Vec<String>, target_path: Option<String>` | `Result<serde_json::Value, String>` |
| `get_upload_progress` | 获取上传进度 | `upload_id: String` | `Result<serde_json::Value, String>` |
| `select_and_upload_file` | 选择并上传文件 | `target_path: Option<String>` | `Result<serde_json::Value, String>` |
| `select_and_upload_multiple_files` | 选择并上传多个文件 | 无 | `Result<serde_json::Value, String>` |
| `select_files` | 选择文件（不上传） | 无 | `Result<serde_json::Value, String>` |

### 其他功能

| 命令 | 功能 | 参数 | 返回值 |
|------|------|------|--------|
| `get_backend_config` | 获取后端配置 | 无 | `Result<serde_json::Value, String>` |
| `capture_screen` | 截取屏幕 | 无 | `Result<serde_json::Value, String>` |
| `get_monitors` | 获取显示器列表 | 无 | `Result<serde_json::Value, String>` |
| `press_win_key` | 模拟按下右箭头键 | 无 | `Result<(), String>` |
| `press_left_key` | 模拟按下左箭头键 | 无 | `Result<(), String>` |
| `load_app_data` | 加载应用数据 | 无 | `Result<serde_json::Value, String>` |
| `save_app_data` | 保存应用数据 | `data: serde_json::Value` | `Result<(), String>` |
| `exit_app` | 退出应用 | 无 | `()` |

## 技术栈

- **Rust 2021**：主要开发语言
- **Tauri**：跨平台桌面应用框架
- **Tokio**：异步运行时
- **Btleplug**：蓝牙通信库
- **Reqwest**：HTTP客户端
- **Serde**：序列化/反序列化
- **Windows API**：系统集成

## 开发与调试

### DEBUG模式

设置环境变量 `CAMFC_DEBUG=1` 启用DEBUG模式：
- `CAMFC_ID`：设备ID（DEBUG模式下使用）
- `CAMFC_KEY`：TOTP密钥（DEBUG模式下使用）

DEBUG模式下会跳过蓝牙连接，直接本地生成TOTP。

### 运行应用

```bash
# 开发模式
cargo tauri dev

# 构建发布版本
cargo tauri build
```

### 测试

```bash
# 运行单元测试
cargo test

# 运行集成测试
cargo test --test utils_integration_test
```

## 错误处理

- 所有API接口都返回 `Result<T, String>` 类型
- 错误信息会被包装成字符串返回给前端
- 内部实现了重试机制，提高稳定性
- 详细的日志输出，便于调试

## 性能优化

1. **TOTP缓存**：30秒内复用TOTP，减少蓝牙通信
2. **设备ID缓存**：缓存设备ID，避免重复获取
3. **连接复用**：保持蓝牙连接，避免频繁重连
4. **异步处理**：文件传输等耗时操作在后台执行
5. **分片传输**：大文件采用分片传输，支持断点续传

## 安全考虑

- 使用设备ID和TOTP进行认证
- 蓝牙通信加密
- 本地数据存储安全
- 防重放攻击（TOTP机制）

## 跨平台支持

- Windows：完全支持
- macOS：基本支持（蓝牙功能可能有限制）
- Linux：基本支持（蓝牙功能可能有限制）

## 未来改进

1. 增强蓝牙连接稳定性
2. 支持更多设备类型
3. 优化文件传输速度
4. 增加更多系统集成功能
5. 改进错误处理和用户反馈
