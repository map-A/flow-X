#!/usr/bin/env python3
"""
FlowX 完整测试套件
运行所有测试并生成报告
"""
import subprocess
import sys
from datetime import datetime

def run_test_script(name, script):
    """运行测试脚本"""
    print(f"\n{'='*70}")
    print(f"运行: {name}")
    print('='*70)

    try:
        result = subprocess.run(
            ["python3", script],
            capture_output=True,
            text=True,
            timeout=120
        )

        if result.returncode == 0:
            print(result.stdout)
            return True, result.stdout
        else:
            print(result.stdout)
            print(result.stderr)
            return False, result.stdout
    except Exception as e:
        print(f"❌ 异常: {e}")
        return False, str(e)

print("="*70)
print("FlowX 完整测试套件")
print("="*70)
print(f"时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")

test_suites = [
    ("CLI 命令测试", "scripts/test_cli.py"),
    ("自然语言驱动测试", "scripts/test_nlp_final.py"),
    ("功能增强测试", "scripts/test_enhancements.py"),
]

results = {}
for name, script in test_suites:
    success, output = run_test_script(name, script)
    results[name] = success

print("\n" + "="*70)
print("测试总结")
print("="*70)

total = len(results)
passed = sum(1 for v in results.values() if v)

for name, success in results.items():
    status = "✅" if success else "❌"
    print(f"{status} {name}")

print()
print(f"总计: {passed}/{total} 通过")
print("="*70)

if passed == total:
    print("\n🎉 所有测试通过！FlowX 功能完整验证成功！")
    sys.exit(0)
else:
    print("\n⚠️  部分测试失败")
    sys.exit(1)
