# CAMFC 客户端 - Cloud AI Multi Function Clicker

> 一个桌面应用，用于连接蓝牙 Cpen 设备并获取 TOTP（一次性密码）和设备 ID

## 项目简介

这个项目是我们几个同学一起搞的设计主要想法是做一个能跟特定蓝牙设备（Cpen）通信的桌面客户端，能拿到设备的 TOTP 和唯一标识。

**用于取代被希沃断头台吓得瑟瑟发抖的u盘们**

## 主要功能

### 1. 蓝牙连接管理
- 自动扫描附近的蓝牙设备
- 智能识别 Cpen 设备（根据设备名前缀匹配）
- 只保持一个有效连接（不能同时连多个，会冲突）

### 2. TOTP 获取
- 向连接的 Cpen 设备发送 `getTotp` 命令
- 自动设置设备时间（需要先 `setTime`）
- 30 秒缓存机制，避免频繁请求

### 3. 设备信息获取
- 获取设备唯一 ID（UUID）
- 实时显示连接状态
- 手动断开/重新连接

### 4. 主题切换
- 支持亮色/暗色主题
- 自动跟随系统主题（如果用户没手动设置过）
- 用户偏好保存到 localStorage

## 技术栈

### 前端
- **Vue 3** - 主要框架
- **TypeScript** - 类型安全（虽然有些地方还是 any，慢慢改吧）
- **Pinia** - 状态管理（主要用在蓝牙状态）
- **Vue Router** - 路由
- **Vite** - 构建工具（打包速度确实快）

### 后端（Rust 层）
- **Tauri** - 桌面应用框架
- **tokio** - 异步运行时（蓝牙操作都是异步的）
- **btleplug** - 蓝牙通信库（Windows/macOS/Linux 都支持）

### 样式
- **CSS 变量** - 主题切换全靠这个
- **Normalize.css** - 重置浏览器默认样式
- **Remix Icon** - 图标库

## 安装和运行

### 环境要求
1. **Node.js** 18+（我们用的 20，低版本没试过）
2. **Rust** 1.70+（装 Tauri 需要的）
3. **系统依赖**：
   - Windows：没啥特别的，应该都能跑
   - macOS：需要 Xcode 命令行工具
   - Linux：需要 libwebkit2gtk 之类的，具体看 Tauri 文档

### 开发模式运行
```bash
# 安装依赖（第一次运行需要）
npm install

# 启动开发服务器
npm run dev

# 同时启动 Tauri 窗口
npm run tauri dev
```

### 生产构建
```bash
# 构建前端
npm run build

# 构建桌面应用（打包成安装包）
npm run tauri build
```

## 项目结构说明

```
h:/dev/CAMFC-client/
├── src/                    # 前端源代码
│   ├── components/        # Vue 组件
│   │   ├── data/         # 数据相关（蓝牙、文件系统）
│   │   ├── file/         # 文件管理组件
│   │   └── layout/       # 布局组件（头部、侧边栏等）
│   ├── composables/      # Vue 组合式函数
│   ├── stores/           # Pinia 状态存储
│   ├── router/           # 路由配置
│   └── views/            # 页面视图
├── src-tauri/            # Rust 后端代码
│   ├── src/
│   │   ├── bluetooth.rs          # 蓝牙底层操作
│   │   ├── cpen_device_manager.rs # 业务逻辑管理器
│   │   └── lib.rs                # Tauri 命令入口
│   └── tauri.conf.json           # Tauri 配置文件
├── public/               # 静态资源
└── 各种配置文件          # package.json, vite.config.ts 等
```

## 代码特点（或者说，我们遇到的坑）

### 蓝牙通信的坑
最开始用 JavaScript 的 Web Bluetooth API，发现限制太多（特别是 Windows 上）。后来改用 Rust 的 btleplug，但异步编程挺烧脑的，尤其是错误处理。

### 状态管理
蓝牙状态需要在多个组件间共享，开始用 props 传，后来发现太乱。换成 Pinia 好多了，但要注意 store 的初始化时机。

### 主题切换
用 CSS 变量实现主题切换，比写两套 CSS 方便。但要注意过渡动画，不然切换时很生硬。

## 开发注意事项

### Rust 部分
- **设备连接**：Cpen 设备有特定服务 UUID，写死在代码里了（如果设备升级可能要改）
- **命令格式**：发送给设备的命令是特定字节数组，不能随便改
- **错误处理**：蓝牙连接可能随时断开，所有操作都要考虑重试

### Vue 部分
- **响应式**：蓝牙状态变化时要及时更新 UI
- **生命周期**：组件卸载时要清理监听器，不然内存泄漏
- **路由**：有些页面需要蓝牙已连接才能访问，做了路由守卫

## 已知问题和 TODO

### 已知问题
1. 某些 Windows 电脑蓝牙驱动有问题，连接不稳定
2. 设备距离远了会自动断开，重连逻辑还不够完善
3. 暗色主题下某些图标颜色对比度不够（懒得调了）

## 许可证

本项目采用 AGPL v3 许可证。简单说就是：你可以用、可以改，但如果你分发修改后的版本，必须开源。

完整许可证文本见 [LICENCE](LICENCE) 文件。

## 贡献者

- **许嘉乐** ([@ant-cave](https://github.com/ant-cave)) - 项目发起，主要 Rust 开发
- **陈欣航** ([@cxh09](https://github.com/cxh09))
- **温子墨** ([@lusamaqq](https://github.com/lusamaqq))

## 问题反馈

遇到问题可以：
1. 先检查设备蓝牙是否开启
2. 确认设备是 Cpen 且电量充足  
3. 查看开发者工具控制台有没有错误
4. 如果还不行...提 issue 吧，我们尽量复现

---

*写于 2026年，某个赶作业的深夜*
