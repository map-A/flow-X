#!/usr/bin/env python3
"""
FlowX 功能增强测试 - 元素查找、手势、多设备
"""
import subprocess
import sys

def run_test(name, test_code):
    """运行 Rust 测试"""
    print(f"\n{name}")
    print("-" * 70)

    result = subprocess.run(
        ["cargo", "test", "-p", "flowx-core", "--lib", test_code, "--", "--nocapture"],
        capture_output=True,
        text=True,
        timeout=30
    )

    if result.returncode == 0:
        print("✅ 通过")
        return True
    else:
        print(f"❌ 失败: {result.stderr[:200]}")
        return False

print("="*70)
print("FlowX 功能增强测试")
print("="*70)

tests = [
    ("元素查找 - Text 选择器", "test_element_finder"),
    ("元素查找 - 计算中心点", "test_element_center"),
    ("手势 - 捏合缩小", "test_pinch_in"),
    ("手势 - 拉伸放大", "test_pinch_out"),
    ("手势 - 双击", "test_double_tap"),
    ("手势 - 三指滑动", "test_three_finger_swipe"),
    ("多设备 - 注册设备", "test_register_device"),
    ("多设备 - 注销设备", "test_unregister_device"),
    ("多设备 - 并发执行", "test_parallel_execution"),
    ("多设备 - 设置状态", "test_set_status"),
]

passed = 0
for name, test_code in tests:
    if run_test(name, test_code):
        passed += 1

print("\n" + "="*70)
print(f"结果: {passed}/{len(tests)} 通过")
print("="*70)

if passed == len(tests):
    print("\n🎉 所有功能增强测试通过！")
    sys.exit(0)
else:
    print("\n⚠️  部分测试失败")
    sys.exit(1)
