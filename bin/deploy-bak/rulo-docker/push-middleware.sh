#!/usr/bin/env bash
# ============================================
# 中间件镜像拉取 & 推送到阿里云 ACR
# 用途: 在无法直连 DockerHub 的 K3s 节点上，
#       先在本地拉取官方镜像，推送到私有仓库供集群拉取。
#
# 用法:
#   ./push-middleware.sh pull   # 本地拉取所有中间件镜像
#   ./push-middleware.sh push   # tag + 推送到阿里云 ACR
#   ./push-middleware.sh all    # pull + push
# ============================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/registry.env"

# ---- 中间件镜像清单（与 bin/rulo-k3s/k3s-deploy.yaml 保持一致） ----
declare -A IMAGES=(
    ["postgres"]="postgres:17-alpine"
    ["redis"]="redis:7-alpine"
    ["rustfs"]="rustfs/rustfs:latest"
)

login_registry() {
    echo ">>> 登录镜像仓库: ${REGISTRY}"
    echo "${ALIYUN_PASSWORD}" | docker login --username="${ALIYUN_USERNAME}" --password-stdin "${REGISTRY}"
}

do_pull() {
    echo "========== 拉取中间件镜像 =========="
    for name in "${!IMAGES[@]}"; do
        local src="${IMAGES[${name}]}"
        echo ">>> 拉取 ${src} ..."
        docker pull "${src}"
    done
    echo ""
    echo "✓ 所有中间件镜像拉取完成"
}

do_push() {
    login_registry
    echo ""
    echo "========== 推送中间件镜像到 ACR =========="
    for name in "${!IMAGES[@]}"; do
        local src="${IMAGES[${name}]}"
        local dst="${REGISTRY}/${NAMESPACE}/${name}:latest"

        echo ">>> tag  ${src} -> ${dst}"
        docker tag "${src}" "${dst}"

        echo ">>> push ${dst}"
        docker push "${dst}"
        echo ""
    done
    echo "✓ 所有中间件镜像推送完成"
    echo ""
    echo "镜像列表:"
    for name in "${!IMAGES[@]}"; do
        echo "  ${REGISTRY}/${NAMESPACE}/${name}:latest"
    done
}

usage() {
    echo "用法: $0 {pull|push|all}"
    echo ""
    echo "  pull    拉取 DockerHub 官方镜像到本地"
    echo "  push    tag 并推送到阿里云 ACR（自动登录）"
    echo "  all     pull + push 一步完成"
    exit 1
}

if [[ $# -lt 1 ]]; then
    usage
fi

case "$1" in
    pull) do_pull ;;
    push) do_push ;;
    all)  do_pull; echo ""; do_push ;;
    *)    usage ;;
esac
