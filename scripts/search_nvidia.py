#!/usr/bin/env python3
"""
FlowX 自动化脚本 - 使用 FlowX Python API 搜索英伟达
"""
import time
from flowx import Device

print("=" * 60)
print("FlowX 自动化脚本 - 搜索英伟达")
print("=" * 60)
print()

# 连接设备
print("📱 连接设备...")
device = Device.connect("emulator-5554")
print("✅ 已连接\n")

# 1. 点击主屏幕唤醒
print("1. 唤醒设备...")
device.click(540, 1140)
time.sleep(0.5)

# 2. 点击 Chrome 图标位置（底部应用栏中间）
print("2. 点击 Chrome 图标...")
device.click(540, 2100)
time.sleep(3)

# 3. 点击地址栏
print("3. 点击地址栏...")
device.click(540, 200)
time.sleep(1)

# 4. 输入搜索词 "nvidia"（英伟达）
print("4. 输入 'nvidia'...")
device.input_text("nvidia")
time.sleep(1)

# 5. 点击软键盘的搜索按钮（右下角）
print("5. 点击搜索按钮...")
device.click(950, 2100)
time.sleep(3)

# 6. 滑动到页面底部（模拟真人滑动）
print("6. 滑动到页面底部...")
for i in range(5):
    device.swipe(540, 1500, 540, 500, 500)
    time.sleep(0.5)
    print(f"   滑动 {i+1}/5")

print()
print("=" * 60)
print("✅ 脚本执行完成！")
print("=" * 60)
print()
print("📊 使用的 FlowX API:")
print("  - Device.connect()  - 连接设备")
print("  - device.click()    - 点击屏幕")
print("  - device.input_text() - 输入文本")
print("  - device.swipe()    - 滑动屏幕")
