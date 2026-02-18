#!/usr/bin/env bash
set -euo pipefail

MODEL_ID="${MODEL_ID:-sentence-transformers/all-MiniLM-L6-v2}"
MODEL_NAME="${MODEL_NAME:-all-MiniLM-L6-v2}"
OUT_DIR="${OUT_DIR:-$HOME/.yai/models/embeddings/$MODEL_NAME}"

mkdir -p "$OUT_DIR"

BASE_URL="https://huggingface.co/${MODEL_ID}/resolve/main"
MODEL_URL="${BASE_URL}/onnx/model.onnx"
TOKEN_URL="${BASE_URL}/tokenizer.json"

echo "Fetching embeddings model:"
echo "  model_id:   ${MODEL_ID}"
echo "  model_name: ${MODEL_NAME}"
echo "  out_dir:    ${OUT_DIR}"

curl -L -o "${OUT_DIR}/model.onnx" "${MODEL_URL}"
curl -L -o "${OUT_DIR}/tokenizer.json" "${TOKEN_URL}"

echo "OK: downloaded model.onnx + tokenizer.json"
