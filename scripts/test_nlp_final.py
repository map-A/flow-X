#!/usr/bin/env python3
import subprocess
import sys

tests = [
    "打开 Safari 浏览器",
    "点击屏幕坐标 (100, 200)",
    "输入文字 hello world",
    "从坐标 (540, 1500) 滑动到 (540, 800)"
]

print("="*70)
print("FlowX 自然语言驱动测试")
print("="*70)

passed = 0
for i, cmd in enumerate(tests, 1):
    print(f"\n测试 {i}: {cmd}")
    print("-"*70)
    
    result = subprocess.run(
        ["python3", "scripts/nlp_engine_v2.py", "--dry-run", cmd],
        capture_output=True,
        text=True,
        timeout=30
    )
    
    if result.returncode == 0 and "import flowx" in result.stdout:
        print("✅ 通过")
        passed += 1
    else:
        print(f"❌ 失败: {result.stderr[:100]}")

print("\n" + "="*70)
print(f"结果: {passed}/{len(tests)} 通过")
print("="*70)

sys.exit(0 if passed == len(tests) else 1)
