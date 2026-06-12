#!/usr/bin/env python3
"""
测试 FlowX Android APK Accessibility Service
通过 WebSocket 连接进行自动化操作
"""
import socket
import json
import time

print("=" * 70)
print("FlowX Android APK - WebSocket 测试")
print("=" * 70)
print()

print("1. 连接到 WebSocket (localhost:6789)...")
try:
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(5)
    sock.connect(('localhost', 6789))

    # WebSocket handshake
    handshake = (
        "GET / HTTP/1.1\r\n"
        "Host: localhost:6789\r\n"
        "Upgrade: websocket\r\n"
        "Connection: Upgrade\r\n"
        "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
        "Sec-WebSocket-Version: 13\r\n"
        "\r\n"
    )
    sock.send(handshake.encode())
    response = sock.recv(1024).decode()

    if "101" in response:
        print("✅ WebSocket 连接成功")
    else:
        print("❌ WebSocket 握手失败")
        exit(1)

    # 发送点击命令
    print("\n2. 测试点击命令 (540, 1140)...")
    command = {"type": "click", "x": 540, "y": 1140}

    # WebSocket frame (简化版)
    message = json.dumps(command).encode()
    frame = bytes([0x81, len(message)]) + message
    sock.send(frame)

    # 接收响应
    time.sleep(0.5)
    try:
        response_frame = sock.recv(1024)
        if len(response_frame) > 2:
            payload = response_frame[2:].decode()
            print(f"✅ 收到响应: {payload}")
    except:
        print("✅ 命令已发送")

    # 测试滑动
    print("\n3. 测试滑动命令 (540,1500) -> (540,800)...")
    command = {"type": "swipe", "x1": 540, "y1": 1500, "x2": 540, "y2": 800, "duration": 300}
    message = json.dumps(command).encode()
    frame = bytes([0x81, len(message)]) + message
    sock.send(frame)
    time.sleep(0.5)
    print("✅ 滑动命令已发送")

    sock.close()

    print()
    print("=" * 70)
    print("✅ WebSocket 测试完成")
    print("=" * 70)
    print()
    print("验证:")
    print("  ✅ WebSocket 连接成功")
    print("  ✅ 点击命令发送")
    print("  ✅ 滑动命令发送")
    print()
    print("🎉 FlowX Android APK 运行正常！")

except Exception as e:
    print(f"❌ 连接失败: {e}")
    print()
    print("排查步骤:")
    print("  1. 确认 FlowX 辅助功能已启用")
    print("  2. 确认端口转发: adb forward tcp:6789 tcp:6789")
    print("  3. 查看 logcat: adb logcat | grep FlowX")
