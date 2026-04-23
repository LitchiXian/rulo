#!/usr/bin/env bash
# ============================================
# rulo-api 镜像构建脚本（本地）
# 用法:
#   ./build.sh                  # cargo build --release + docker build (tag=latest)
#   TAG=v1.0.0 ./build.sh       # 自定义 tag
#   ./build.sh --skip-cargo     # 跳过 cargo 编译，直接打镜像
# ============================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/registry.env"

TAG="${TAG:-latest}"
IMAGE="${REGISTRY}/${NAMESPACE}/rulo-api:${TAG}"

cd "${PROJECT_ROOT}"

# ---- 本地 cargo 编译 ----
if [[ "${1:-}" != "--skip-cargo" ]]; then
    echo "========== cargo build --release =========="
    (cd rulo-api && cargo build --release)
    echo ""
fi

# ---- 检查二进制 ----
BINARY="rulo-api/target/release/app"
if [[ ! -f "${BINARY}" ]]; then
    echo "✗ 未找到编译产物: ${BINARY}"
    echo "  请先执行 cd rulo-api && cargo build --release"
    exit 1
fi

echo "========== 构建 rulo-api 镜像 =========="
echo "镜像: ${IMAGE}"
echo ""

docker build \
    -f "${SCRIPT_DIR}/Dockerfile.api" \
    -t "${IMAGE}" \
    .

echo ""
echo "✓ 构建完成: ${IMAGE}"
docker images | grep "rulo-api" || true
