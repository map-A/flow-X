#!/usr/bin/env python3
"""FlowX AI 模块使用示例"""

import flowx
import os

def main():
    # 方式1: 从环境变量初始化
    # export FLOWX_API_KEY="your-glm-api-key"
    agent = flowx.PyAIAgent.from_env()

    # 方式2: 直接传入 API key
    # agent = flowx.PyAIAgent("your-glm-api-key")

    # 创建设备
    device = flowx.PyDevice.connect("macos")

    # 获取截图
    screenshot = device.screenshot()

    # 示例1: 执行自然语言指令
    print("=== 示例1: 执行指令 ===")
    result = agent.execute("打开 Safari 浏览器", screenshot)

    if result.success:
        print(f"✅ 执行成功，步骤数：{result.steps}")
    else:
        print(f"❌ 执行失败：{result.error}")

    # 示例2: 视觉问答
    print("\n=== 示例2: 视觉问答 ===")
    answer = agent.ask("当前屏幕显示的是什么应用？", screenshot)
    print(f"答案：{answer}")

    # 示例3: 配置最大步骤数
    print("\n=== 示例3: 自定义配置 ===")
    agent.set_max_steps(30)

    result = agent.execute("点击搜索框，输入 hello world", screenshot)
    print(f"执行结果：成功={result.success}, 步骤={result.steps}")

if __name__ == "__main__":
    main()
