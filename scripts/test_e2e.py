#!/usr/bin/env python3
"""
FlowX 端到端真实测试
"""
import subprocess
import sys
import time

print("=" * 70)
print("FlowX 端到端真实测试")
print("=" * 70)
print()

# 检查模拟器
print("📱 检查 Android 模拟器...")
result = subprocess.run(["adb", "devices"], capture_output=True, text=True)
devices = [line for line in result.stdout.split('\n') if '\tdevice' in line]

if not devices:
    print("❌ 未检测到 Android 设备")
    print("   请先启动模拟器")
    sys.exit(1)

device_id = devices[0].split('\t')[0]
print(f"✅ 检测到设备: {device_id}")
print()

# 测试 FlowX Python API
print("🧪 测试 FlowX Python API")
print("-" * 70)

try:
    # 需要先激活虚拟环境
    import os
    os.chdir("/Users/mm/Documents/compile_accelarate")

    # 测试导入
    sys.path.insert(0, "/Users/mm/Documents/compile_accelarate/.venv/lib/python3.11/site-packages")
    from flowx import Device

    print("✅ FlowX 模块导入成功")

    # 连接设备
    print(f"\n1. 连接设备: {device_id}")
    device = Device.connect(device_id)
    print("✅ 设备连接成功")

    # 测试点击
    print("\n2. 测试点击操作 (500, 1000)")
    device.click(500, 1000)
    time.sleep(0.5)
    print("✅ 点击执行成功")

    # 测试滑动
    print("\n3. 测试滑动操作 (540, 1500 -> 540, 500)")
    device.swipe(540, 1500, 540, 500, 500)
    time.sleep(0.5)
    print("✅ 滑动执行成功")

    # 测试文本输入
    print("\n4. 测试文本输入 'FlowX'")
    device.input_text("FlowX")
    time.sleep(0.5)
    print("✅ 文本输入成功")

    # 测试截图
    print("\n5. 测试截图功能")
    image = device.screenshot()
    print(f"✅ 截图成功 (尺寸: {image})")

    # 测试屏幕尺寸
    print("\n6. 获取屏幕尺寸")
    width, height = device.get_screen_size()
    print(f"✅ 屏幕尺寸: {width}x{height}")

    print()
    print("=" * 70)
    print("✅ 所有 FlowX API 测试通过！")
    print("=" * 70)

except ImportError as e:
    print(f"❌ 无法导入 FlowX: {e}")
    print("\n请先编译 Python 绑定:")
    print("  cd crates/flowx-python")
    print("  source ../../.venv/bin/activate")
    print("  maturin develop")
    sys.exit(1)

except Exception as e:
    print(f"❌ 测试失败: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
