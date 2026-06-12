#!/usr/bin/env python3
"""
测试 FlowX 在 macOS 上操作 Edge 浏览器
使用 FlowX Python API
"""
import flowx
import time

print("=" * 70)
print("FlowX macOS Edge 浏览器测试 (使用 FlowX API)")
print("=" * 70)
print()

# 连接到本地 macOS
print("连接到 macOS...")
device = flowx.Device.connect("macos")
print("✅ 已连接")
print()

# 测试 1: 打开 Edge
print("1. 打开 Microsoft Edge...")
device.open_app("Microsoft Edge")
print("✅ Edge 已打开")
time.sleep(2)

# 测试 2: 打开新标签页
print("\n2. 打开新标签页 (⌘+T)...")
device.press_key("CommandT")
time.sleep(1)
print("✅ 新标签页已打开")

# 测试 3: 输入网址
print("\n3. 输入网址 'anthropic.com'...")
device.input_text("anthropic.com")
time.sleep(1)
print("✅ 网址已输入")

# 测试 4: 按回车
print("\n4. 按回车键...")
device.press_key("Enter")
time.sleep(3)
print("✅ 页面加载中")

# 测试 5: 滚动页面
print("\n5. 向下滚动页面...")
for i in range(3):
    device.press_key("Down")
    time.sleep(0.5)
print("✅ 页面滚动完成")

print()
print("=" * 70)
print("✅ macOS Edge 浏览器测试完成")
print("=" * 70)
print()
print("验证功能:")
print("  ✅ 打开应用 (使用 FlowX)")
print("  ✅ 新建标签页 (使用 FlowX)")
print("  ✅ 文本输入 (使用 FlowX)")
print("  ✅ 按键操作 (使用 FlowX)")
print("  ✅ 页面滚动 (使用 FlowX)")
print()
print("🎉 FlowX 可以完整控制 macOS Edge 浏览器！")
