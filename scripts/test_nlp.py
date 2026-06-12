#!/usr/bin/env python3
"""
测试 FlowX 自然语言驱动功能
"""
import sys
import os
sys.path.insert(0, os.path.dirname(__file__))

from nlp_engine import NLPEngine

print("=" * 70)
print("FlowX 自然语言驱动测试")
print("=" * 70)
print()

# 测试用例
test_cases = [
    {
        "name": "测试 1: 打开应用",
        "command": "打开 Safari 浏览器",
        "platform": "macos"
    },
    {
        "name": "测试 2: 点击操作",
        "command": "点击屏幕坐标 (100, 200)",
        "platform": "macos"
    },
    {
        "name": "测试 3: 文本输入",
        "command": "输入文字 hello world",
        "platform": "macos"
    },
    {
        "name": "测试 4: Android 滑动",
        "command": "从坐标 (540, 1500) 滑动到 (540, 800)",
        "platform": "android"
    }
]

engine = NLPEngine()
results = []

for i, test in enumerate(test_cases, 1):
    print(f"\n{test['name']}")
    print(f"指令: {test['command']}")
    print("-" * 70)

    try:
        # Dry run mode - 只生成代码不执行
        result = engine.run(test['command'], dry_run=True)

        if result['status'] == 'dry_run':
            print("✅ 代码生成成功")
            results.append({'test': test['name'], 'status': 'pass'})
        else:
            print(f"❌ 生成失败: {result}")
            results.append({'test': test['name'], 'status': 'fail'})

    except Exception as e:
        print(f"❌ 异常: {e}")
        results.append({'test': test['name'], 'status': 'error', 'error': str(e)})

print()
print("=" * 70)
print("测试总结")
print("=" * 70)

passed = sum(1 for r in results if r['status'] == 'pass')
total = len(results)

for r in results:
    status = "✅" if r['status'] == 'pass' else "❌"
    print(f"{status} {r['test']}")

print()
print(f"结果: {passed}/{total} 通过")
print()

if passed == total:
    print("🎉 所有自然语言测试通过！")
else:
    print("⚠️  部分测试失败")
    sys.exit(1)
