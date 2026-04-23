#!/usr/bin/env bash
# ============================================
# rulo-api 镜像推送脚本（本地 → 阿里云 ACR）
# 用法:
#   ./push.sh              # 推送 latest
#   TAG=v1.0.0 ./push.sh   # 自定义 tag
# ============================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/registry.env"

TAG="${TAG:-latest}"
IMAGE="${REGISTRY}/${NAMESPACE}/rulo-api:${TAG}"

echo ">>> 登录镜像仓库: ${REGISTRY}"
echo "${ALIYUN_PASSWORD}" | docker login --username="${ALIYUN_USERNAME}" --password-stdin "${REGISTRY}"
echo ""

echo "========== 推送 rulo-api =========="
docker push "${IMAGE}"
echo ""
echo "✓ 推送完成: ${IMAGE}"
