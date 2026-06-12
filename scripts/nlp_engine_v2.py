#!/usr/bin/env python3
"""
FlowX 自然语言驱动引擎（简化版）
使用本地 LLM 将自然语言转换为 Python 脚本并执行
"""
import json
import sys
import subprocess
import tempfile
import os

class NLPEngine:
    def __init__(self, llm_url: str = "http://127.0.0.1:8080"):
        self.llm_url = llm_url
        self.api_url = f"{llm_url}/v1/chat/completions"

    def translate_to_python(self, nl_command: str) -> str:
        """将自然语言转换为 Python 脚本"""
        prompt = f"""你是一个 FlowX 自动化脚本生成助手。将用户的自然语言指令转换为 Python 代码。

FlowX API 示例:
```python
import flowx
device = flowx.Device.connect("macos")
device.click(100, 200)
device.swipe(100, 500, 100, 200, 300)
device.input_text("hello")
device.press_key("Enter")
device.open_app("Safari")
```

用户指令: {nl_command}

只输出 Python 代码，不要解释。代码以 import flowx 开头。"""

        payload = {
            "model": "mlx-community/Qwen2.5-3B-Instruct-4bit",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.1,
            "max_tokens": 500
        }

        # 使用 curl 调用 API
        result = subprocess.run(
            [
                "curl", "-s", "-X", "POST",
                self.api_url,
                "-H", "Content-Type: application/json",
                "-d", json.dumps(payload)
            ],
            capture_output=True,
            text=True,
            timeout=30
        )

        if result.returncode != 0:
            raise Exception(f"curl failed: {result.stderr}")

        response = json.loads(result.stdout)
        code = response["choices"][0]["message"]["content"]

        # 提取代码块
        if "```python" in code:
            code = code.split("```python")[1].split("```")[0].strip()
        elif "```" in code:
            code = code.split("```")[1].split("```")[0].strip()

        return code

    def execute_code(self, code: str, dry_run: bool = False) -> dict:
        """执行生成的 Python 代码"""
        if dry_run:
            return {"status": "dry_run", "code": code}

        with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
            f.write(code)
            script_path = f.name

        try:
            result = subprocess.run(
                [sys.executable, script_path],
                capture_output=True,
                text=True,
                timeout=30
            )

            return {
                "status": "success" if result.returncode == 0 else "error",
                "code": code,
                "stdout": result.stdout,
                "stderr": result.stderr,
                "returncode": result.returncode
            }
        except subprocess.TimeoutExpired:
            return {
                "status": "timeout",
                "code": code,
                "error": "Execution timeout (30s)"
            }
        finally:
            os.unlink(script_path)

    def run(self, nl_command: str, dry_run: bool = False) -> dict:
        """完整流程: 翻译 + 执行"""
        print(f"[NLP] Input: {nl_command}")

        code = self.translate_to_python(nl_command)
        print(f"[NLP] Generated code:\n{code}\n")

        if dry_run:
            print("[NLP] Dry run mode, skipping execution")
            return {"status": "dry_run", "code": code}

        print("[NLP] Executing...")
        result = self.execute_code(code, dry_run)

        if result["status"] == "success":
            print(f"[NLP] Success!")
        else:
            print(f"[NLP] Error: {result.get('stderr', result.get('error'))}")

        return result


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 nlp_engine_v2.py <natural language command>")
        print('Example: python3 nlp_engine_v2.py "打开 Safari 并搜索 anthropic"')
        sys.exit(1)

    nl_command = " ".join(sys.argv[1:])
    dry_run = "--dry-run" in sys.argv

    engine = NLPEngine()
    result = engine.run(nl_command, dry_run)

    print("\n" + "="*70)
    print("Result:", json.dumps(result, indent=2, ensure_ascii=False))
    sys.exit(0 if result["status"] in ("success", "dry_run") else 1)


if __name__ == "__main__":
    main()
