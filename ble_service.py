import bluetooth
import sys
import time
import select  
from micropython import const
import key
import random
import utotp
import machine
import light
import _thread

# ===================== 硬件引脚配置 =====================
PIN_MODE_SWITCH = 2  # GPIO3 检测按键按下/松开
PIN_LED = 48         # RGB LED 引脚

# ========== 全局常量定义 ==========
BUTTON_CMD_PRESS = "button_press"
BUTTON_CMD_RELEASE = "button_release"

# ========== 读取/生成 TOTP 密钥 ==========
try:
    with open('totpsecret.txt', 'r') as f:
        totp = f.read().strip()
        print(f"TOTP 密钥：{totp}")
except OSError:
    print("文件不存在或无法读取，将生成新密钥")
    totp = key.generate()
    print("生成的 TOTP 密钥:", totp)
    with open('totpsecret.txt', 'w') as f:
        f.write(totp)

# ========== 全局变量 ==========
is_connected = False
current_conn = None
notify_handle = None

# GPIO 状态相关
gpio_last_state = None
gpio_pending_cmd = None
last_gpio_change_time = 0
GPIO_DEBOUNCE_MS = 150
gpio_initialized = False

# BLE 相关
last_handle_data = ""
last_handle_time = 0
DEBOUNCE_TIME = 50

# TOTP 任务队列 (线程安全通信)
totp_task_pending = False
totp_result = None

# 设备标识
device_id = random.randint(1000, 9999)
deviceId = '3ed661ad-77a8-42b9-9100-3737e9317f36'

ble = bluetooth.BLE()

# ===================== 封装的核心函数 =====================
def ble_send(data_str):
    """发送数据并打印日志（仅在主线程调用）"""
    global current_conn, ble, notify_handle
    
    if not isinstance(data_str, str) or not data_str.strip():
        return False
    if current_conn is None:
        return False
        
    try:
        max_len = ble.config("mtu") - 3
    except Exception:
        max_len = 20
        
    send_data = data_str[:max_len]
    
    try:
        ble.gatts_notify(current_conn, notify_handle, send_data.encode("utf-8"))
        return True
    except Exception as e:
        print(f"BLE 发送异常：{e}")
        return False

def blehandle(received_data):
    """处理接收到的 BLE 命令（轻量级，耗时任务交给线程）"""
    print(f"[收到 BLE 指令]: {received_data}")
    
    if received_data == 'getTotp':
        # ✅ 触发 TOTP 计算线程任务
        global totp_task_pending
        totp_task_pending = True
        print("[TOTP] 已触发计算任务")
        
    elif received_data == "getId":
        ble_send(deviceId)
        
    elif received_data.startswith("setTime:"):
        try:
            timestamp_str = received_data.split(":", 1)[1]
            timestamp = int(timestamp_str)
            tm = time.gmtime(timestamp)
            rtc_tuple = (tm[0], tm[1], tm[2], tm[6]+1, tm[3], tm[4], tm[5], 0)
            machine.RTC().datetime(rtc_tuple)
            ble_send(f"OK:setTime:{timestamp}")
            print(f"[时间] 已同步：{timestamp}")
        except Exception as e:
            ble_send(f"ERROR:setTime:{e}")
            print(f"[时间] 同步失败：{e}")

# ========== GPIO 中断回调函数 ==========
def gpio_irq_handler(pin):
    """GPIO 电平变化中断回调（检测按下和松开）"""
    global gpio_last_state, gpio_pending_cmd, last_gpio_change_time, gpio_initialized
    
    current_time = time.ticks_ms()
    
    # 去抖动检查
    if time.ticks_diff(current_time, last_gpio_change_time) < GPIO_DEBOUNCE_MS:
        return
    
    last_gpio_change_time = current_time
    current_level = pin.value()
    
    # ✅ 修复：首次中断时同步状态，不触发事件
    if not gpio_initialized:
        gpio_last_state = current_level
        gpio_initialized = True
        print(f"[GPIO] 初始状态同步：{'按下 (0)' if current_level == 0 else '松开 (1)'}")
        return
    
    # 判断是按下还是松开
    if gpio_last_state == 1 and current_level == 0:
        action = "按下"
        gpio_pending_cmd = BUTTON_CMD_PRESS
    elif gpio_last_state == 0 and current_level == 1:
        action = "松开"
        gpio_pending_cmd = BUTTON_CMD_RELEASE
    else:
        return
    
    gpio_last_state = current_level
    
    # 获取当前时间
    current_time_str = time.localtime()
    time_format = f"{current_time_str[0]}-{current_time_str[1]}-{current_time_str[2]} {current_time_str[3]}:{current_time_str[4]}:{current_time_str[5]}"
    
    # 打印详细信息
    print("\n" + "="*50)
    print(f"[按键事件] {action}")
    print(f"时间：{time_format}")
    print(f"引脚：GPIO{PIN_MODE_SWITCH}")
    print(f"电平：{'低电平 (0)' if current_level == 0 else '高电平 (1)'}")
    print(f"命令：{gpio_pending_cmd}")
    print("="*50 + "\n")

# ========== 线程 1: 呼吸灯 + GPIO 监控 ==========
def breathing_light_thread():
    """负责 LED 控制和 GPIO 中断注册"""
    global is_connected, gpio_last_state, gpio_initialized
    
    # 配置 GPIO
    pin_switch = machine.Pin(PIN_MODE_SWITCH, machine.Pin.IN, machine.Pin.PULL_UP)
    
    # ✅ 修复：先读取初始状态，再注册中断
    gpio_last_state = pin_switch.value()
    gpio_initialized = False  # 重置初始化标志，让中断 handler 同步一次
    
    print(f"[GPIO] 初始电平读取：{'按下 (0)' if gpio_last_state == 0 else '松开 (1)'}")
    
    # 注册中断
    pin_switch.irq(trigger=machine.Pin.IRQ_RISING | machine.Pin.IRQ_FALLING, handler=gpio_irq_handler)
    
    print("[线程 1] 呼吸灯 + GPIO 线程已启动")
    
    while True:
        # ✅ 修改：只根据连接状态控制 LED 颜色
        if not is_connected:
            # 未连接：红色呼吸灯
            light.breathing_light(light.COLORS["RED"], speed=5)
        else:
            # 已连接：绿色常亮
            light.set_rgb(light.COLORS["GREEN"])
            time.sleep_ms(100)

# ========== 线程 2: TOTP 计算任务 ==========
def totp_compute_thread():
    """负责耗时的 TOTP 计算，避免阻塞主线程"""
    global totp_task_pending, totp_result, is_connected
    
    print("[线程 2] TOTP 计算线程已启动")
    
    while True:
        if totp_task_pending:
            totp_task_pending = False
            
            try:
                # 执行耗时的 TOTP 计算
                current_timestamp = int(time.time())
                totp_code = utotp.generate_totp(totp, custume_time=current_timestamp)
                totp_result = totp_code
                print(f"[TOTP] 计算完成：{totp_code}")
            except Exception as e:
                totp_result = f"ERROR:{e}"
                print(f"[TOTP] 计算失败：{e}")
        
        time.sleep_ms(50)

# ===================== BLE 初始化与配置 =====================
SERVICE_UUID = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4"
CHARACTERISTIC_UUID = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4"

_IRQ_CENTRAL_CONNECT = const(1)
_IRQ_CENTRAL_DISCONNECT = const(2)
_IRQ_GATTS_WRITE = const(3)
_IRQ_GATTS_READ_REQUEST = const(4)

_FLAG_READ = const(0x0002)
_FLAG_WRITE = const(0x0008)
_FLAG_WRITE_NO_RESPONSE = const(0x0004)
_FLAG_NOTIFY = const(0x0010)

ble.active(True)
ble.config(addr_mode=0x01, gap_name="esp32", mtu=256)

services = [
    (bluetooth.UUID(SERVICE_UUID), (
        (bluetooth.UUID(CHARACTERISTIC_UUID), 
         _FLAG_READ | _FLAG_WRITE | _FLAG_WRITE_NO_RESPONSE | _FLAG_NOTIFY),
    )),
]
handles = ble.gatts_register_services(services)
notify_handle = handles[0][0]

DEVICE_NAME = "Cpen" + str(device_id)
adv_data = b''.join([
    b'\x02', b'\x01', b'\x06',
    bytes([1 + len(DEVICE_NAME)]), b'\x09', DEVICE_NAME.encode(),
    b'\x11', b'\x07', b'\xc4\x42\x26\xcc\x37\x7c\xd5\xbc\xd5\xa7\x99\x1b\xc6\xe4\x16\xd8'
])

# 事件回调
def on_ble_event(event, data):
    global current_conn, last_handle_data, last_handle_time, is_connected
    
    current_time = time.ticks_ms()
    
    if event == _IRQ_CENTRAL_CONNECT:
        conn_handle, _, _ = data
        current_conn = conn_handle
        is_connected = True
        print(f"[BLE] ✅ 连接建立 (Handle: {conn_handle})")
        ble.gap_advertise(None)
        
    elif event == _IRQ_CENTRAL_DISCONNECT:
        conn_handle, _, _ = data
        current_conn = None
        is_connected = False
        print(f"[BLE] ❌ 断开连接 (Handle: {conn_handle})")
        ble.gap_advertise(30000, adv_data=adv_data)
        
    elif event == _IRQ_GATTS_READ_REQUEST:
        return 0
        
    elif event == _IRQ_GATTS_WRITE:
        conn_handle, attr_handle = data
        try:
            received_data = ble.gatts_read(attr_handle).decode("utf-8").strip()
        except UnicodeDecodeError:
            ble.gatts_write(attr_handle, b"")
            return
        
        if (received_data == last_handle_data and 
            time.ticks_diff(current_time, last_handle_time) < DEBOUNCE_TIME):
            ble.gatts_write(attr_handle, b"")
            return
        
        last_handle_data = received_data
        last_handle_time = current_time
        
        if received_data:
            blehandle(received_data)
        
        ble.gatts_write(attr_handle, b"")

ble.irq(on_ble_event)

# ========== 初始化 LED 模块 ==========
try:
    light.init_rgb(pin_num=PIN_LED)
    print("LED 模块初始化成功")
except Exception as e:
    print(f"LED 模块初始化警告：{e}")

# ========== 启动后台线程 ==========
_thread.start_new_thread(breathing_light_thread, ())
_thread.start_new_thread(totp_compute_thread, ())

# ========== 开始广播 ==========
ble.gap_advertise(30000, adv_data=adv_data)

print("-" * 40)
print("系统启动完成")
print(f"设备 ID: {device_id}")
print(f"设备名称：{DEVICE_NAME}")
print(f"GPIO 按键：GPIO{PIN_MODE_SWITCH}")
print(f"RGB LED: GPIO{PIN_LED}")
print(f"LED 状态规则：")
print(f"  - 未连接：红色呼吸灯 🔴")
print(f"  - 已连接：绿色常亮 🟢")
print(f"线程配置：")
print(f"  - 主线程：BLE 通信 + 串口输入")
print(f"  - 线程 1: 呼吸灯 + GPIO 中断")
print(f"  - 线程 2: TOTP 计算任务")
print("-" * 40)

# ========== 主循环 ==========
input_buffer = ""
poll_obj = select.poll()
poll_obj.register(sys.stdin, select.POLLIN)

while True:
    # 处理串口输入
    events = poll_obj.poll(0)
    if events:
        char = sys.stdin.read(1)
        if char == '\n':
            if input_buffer.strip():
                ble_send(input_buffer)
            input_buffer = ""
        else:
            input_buffer += char
    
    # 处理 GPIO 状态变化产生的 BLE 发送请求
    if gpio_pending_cmd is not None:
        if is_connected:
            if ble_send(gpio_pending_cmd):
                print(f"> 蓝牙状态包已推送：[{gpio_pending_cmd}]")
            else:
                print(f"! 蓝牙发送失败：{gpio_pending_cmd}")
        else:
            print(f"! 设备未连接，跳过蓝牙发送：{gpio_pending_cmd}")
        gpio_pending_cmd = None
    
    # 处理 TOTP 计算结果并发送
    if totp_result is not None:
        if is_connected:
            if ble_send(totp_result):
                print(f"> TOTP 验证码已推送：[{totp_result}]")
            else:
                print(f"! TOTP 发送失败：{totp_result}")
        else:
            print(f"! 设备未连接，TOTP 结果丢弃：{totp_result}")
        totp_result = None
    
    time.sleep_ms(10)
