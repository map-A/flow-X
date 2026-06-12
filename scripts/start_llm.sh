#!/usr/bin/env bash
# FlowX 本地 LLM 启动脚本
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# 配置
MLX_MODEL="${MLX_MODEL:-mlx-community/Qwen2.5-3B-Instruct-4bit}"
MLX_PORT="${MLX_PORT:-8080}"
MLX_HOST="${MLX_HOST:-127.0.0.1}"
MLX_LOG="${MLX_LOG:-$ROOT_DIR/mlx.log}"

VENV_DIR="$ROOT_DIR/.venv"
PYTHON_BIN="$VENV_DIR/bin/python"

echo "[info] Starting FlowX local LLM..."
echo "[info] Model: $MLX_MODEL"
echo "[info] Port: $MLX_PORT"

# 检查端口
if lsof -i ":$MLX_PORT" -sTCP:LISTEN -t >/dev/null 2>&1; then
  echo "[ok] Port $MLX_PORT already in use, assuming LLM is running"
  exit 0
fi

# 检查 uv
if ! command -v uv >/dev/null 2>&1; then
  echo "[error] uv not found. Install: curl -LsSf https://astral.sh/uv/install.sh | sh"
  exit 1
fi

# 安装 mlx-lm
if ! "$PYTHON_BIN" -c "import mlx_lm" >/dev/null 2>&1; then
  echo "[info] Installing mlx-lm..."
  uv pip install --python "$PYTHON_BIN" mlx-lm
fi

# 启动 LLM
echo "[info] Starting LLM server (log: $MLX_LOG)..."
nohup "$PYTHON_BIN" -u -m mlx_lm.server \
  --model "$MLX_MODEL" \
  --host "$MLX_HOST" \
  --port "$MLX_PORT" \
  >> "$MLX_LOG" 2>&1 &

# 等待就绪
url="http://$MLX_HOST:$MLX_PORT/v1/models"
echo "[info] Waiting for LLM to be ready..."
for i in $(seq 1 60); do
  if curl -sf --noproxy "*" "$url" --max-time 5 >/dev/null 2>&1; then
    echo "[ok] LLM server ready at http://$MLX_HOST:$MLX_PORT"
    exit 0
  fi
  sleep 3
done

echo "[error] LLM not ready after 3 minutes. Check $MLX_LOG"
exit 1
