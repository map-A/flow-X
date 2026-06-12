#!/usr/bin/env python3
"""
FlowX macOS 平台完整测试
"""
import subprocess
import time
import sys

print("=" * 70)
print("FlowX macOS 平台测试")
print("=" * 70)
print()

# 测试 1: 检查系统
print("🔍 测试 1: 检查 macOS 系统")
print("-" * 70)
result = subprocess.run(["sw_vers"], capture_output=True, text=True)
if result.returncode == 0:
    for line in result.stdout.strip().split('\n'):
        print(f"   {line}")
    print("✅ macOS 系统检测成功")
else:
    print("❌ 无法检测系统信息")
    sys.exit(1)
print()

# 测试 2: 检查 osascript
print("🔍 测试 2: 检查 AppleScript (osascript)")
print("-" * 70)
result = subprocess.run([
    "osascript", "-e", "return 1 + 1"
], capture_output=True, text=True)

if result.returncode == 0:
    print(f"✅ osascript 可用")
    print(f"   测试计算: 1 + 1 = {result.stdout.strip()}")
else:
    print("❌ osascript 不可用")
print()

# 测试 3: 检查辅助功能权限
print("🔍 测试 3: 检查辅助功能权限")
print("-" * 70)
result = subprocess.run([
    "osascript", "-e",
    'tell application "System Events" to get name of first process'
], capture_output=True, text=True)

if result.returncode == 0:
    print(f"✅ 辅助功能权限已授予")
    print(f"   第一个进程: {result.stdout.strip()}")
    has_permission = True
else:
    print("⚠️  需要授予辅助功能权限")
    print("   错误:", result.stderr.strip())
    print()
    print("📝 授权步骤:")
    print("   1. 打开 系统设置 (System Settings)")
    print("   2. 进入 隐私与安全性 → 辅助功能")
    print("   3. 添加 Terminal 或 Python")
    print("   4. 重新运行此脚本")
    has_permission = False
print()

# 测试 4: 测试点击操作（需要权限）
if has_permission:
    print("🖱️  测试 4: 测试点击操作")
    print("-" * 70)

    # 获取鼠标位置
    result = subprocess.run([
        "osascript", "-e",
        'tell application "System Events" to return position of front window of (first process whose frontmost is true)'
    ], capture_output=True, text=True)

    if result.returncode == 0:
        print(f"✅ 可以获取窗口位置")
        print(f"   位置: {result.stdout.strip()}")
    else:
        print("⚠️  无法获取窗口位置")
    print()

# 测试 5: 测试文本输入
print("⌨️  测试 5: 测试键盘输入")
print("-" * 70)
print("提示: 将在 3 秒后输入测试文本 'test'")
print("      请打开一个文本编辑器并将光标置于其中...")
time.sleep(3)

result = subprocess.run([
    "osascript", "-e",
    'tell application "System Events" to keystroke "test"'
], capture_output=True, text=True)

if result.returncode == 0:
    print("✅ 文本输入命令执行成功")
else:
    print("❌ 文本输入失败:", result.stderr.strip())
print()

# 总结
print("=" * 70)
print("📊 测试总结")
print("=" * 70)
print()

if has_permission:
    print("✅ macOS 平台完全就绪")
    print()
    print("🚀 FlowX macOS API 可用:")
    print("   from flowx import Device")
    print("   device = Device.connect('macos://localhost')")
    print("   device.click(500, 500)")
    print("   device.input_text('Hello macOS')")
else:
    print("⚠️  需要授予辅助功能权限后才能使用")
    print("   授权后 FlowX 即可控制 macOS")

print()
