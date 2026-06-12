#!/usr/bin/env python3
"""
FlowX CLI 命令测试
"""
import subprocess
import sys

def run_cli(*args):
    """运行 CLI 命令"""
    result = subprocess.run(
        ["cargo", "run", "-p", "flowx-cli", "--"] + list(args),
        capture_output=True,
        text=True,
        timeout=10
    )
    return result

print("="*70)
print("FlowX CLI 命令测试")
print("="*70)

tests = [
    {
        "name": "测试 1: Click 命令",
        "args": ["--device", "test", "click", "100", "200"],
        "expect": "Click: (100, 200)"
    },
    {
        "name": "测试 2: Swipe 命令",
        "args": ["--device", "test", "swipe", "100", "500", "100", "200"],
        "expect": "Swipe: (100,500) -> (100,200)"
    },
    {
        "name": "测试 3: Input 命令",
        "args": ["--device", "test", "input", "hello"],
        "expect": "Input: hello"
    },
    {
        "name": "测试 4: Info 命令",
        "args": ["--device", "test", "info"],
        "expect": "Info"
    }
]

passed = 0
for test in tests:
    print(f"\n{test['name']}")
    print(f"命令: {' '.join(test['args'])}")
    print("-"*70)

    try:
        result = run_cli(*test['args'])

        if result.returncode == 0 and test['expect'] in result.stdout:
            print("✅ 通过")
            passed += 1
        else:
            print(f"❌ 失败")
            print(f"输出: {result.stdout[:100]}")

    except Exception as e:
        print(f"❌ 异常: {e}")

print("\n" + "="*70)
print(f"结果: {passed}/{len(tests)} 通过")
print("="*70)

sys.exit(0 if passed == len(tests) else 1)
