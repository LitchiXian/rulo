#!/bin/bash
set -e

export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/config}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEPLOY_YAML="${SCRIPT_DIR}/k3s-deploy.yaml"
NAMESPACE="rulo"

if [ ! -f "$DEPLOY_YAML" ]; then
    echo "❌ 未找到部署清单: $DEPLOY_YAML"
    echo "   请将 k3s-deploy.yaml 放到本脚本同目录下"
    exit 1
fi

# ============================================
# rulo K3s 部署管理脚本
# Secret 只在 init 时创建一次，日常操作不需要管
# ============================================

usage() {
    echo "用法: $0 {apply|delete|restart|upgrade|status|logs}"
    echo ""
    echo "  apply    部署/更新全部资源（kubectl apply）"
    echo "  delete   删除全部资源"
    echo "  restart  滚动重启 rulo-api"
    echo "  upgrade  apply + 滚动重启（日常更新用这个）"
    echo "  status   查看状态"
    echo "  logs     查看 rulo-api 日志（可追加 -f）"
    exit 1
}

cmd_apply() {
    echo ">>> 创建 rustfs 宿主机目录（uid=1000 可写）..."
    sudo mkdir -p /opt/rulo/env/rustfs/data /opt/rulo/env/rustfs/logs
    sudo chown -R 1000:1000 /opt/rulo/env/rustfs
    echo ""
    echo ">>> 部署 rulo 资源..."
    kubectl apply -f "$DEPLOY_YAML"
    echo ""
    echo ">>> 等待 rulo-api 就绪..."
    kubectl rollout status deployment/rulo-api -n "$NAMESPACE" --timeout=180s || true
    echo ""
    cmd_status
}

cmd_upgrade() {
    echo ">>> 日常更新 rulo-api..."
    kubectl apply -f "$DEPLOY_YAML"
    echo ""
    echo ">>> 滚动重启（拉取最新镜像）..."
    kubectl rollout restart deployment/rulo-api -n "$NAMESPACE"
    kubectl rollout status deployment/rulo-api -n "$NAMESPACE" --timeout=180s
    echo ""
    cmd_status
}

cmd_delete() {
    echo ">>> 删除 rulo 业务资源..."
    kubectl delete -f "$DEPLOY_YAML" --ignore-not-found
    echo ">>> 删除完成（namespace 和 aliyun-auth Secret 保留）"
    echo "   重新部署只需: ./05_deploy.sh apply"
}

cmd_restart() {
    echo ">>> 滚动重启 rulo-api..."
    kubectl rollout restart deployment/rulo-api -n "$NAMESPACE"
    kubectl rollout status deployment/rulo-api -n "$NAMESPACE" --timeout=180s
    echo ">>> 重启完成"
}

cmd_status() {
    echo "=== Pods ==="
    kubectl get pods -n "$NAMESPACE" -o wide 2>/dev/null || true
    echo ""
    echo "=== Deployments ==="
    kubectl get deployments -n "$NAMESPACE" -o wide 2>/dev/null || true
    echo ""
    echo "=== Services ==="
    kubectl get services -n "$NAMESPACE" -o wide 2>/dev/null || true
}

cmd_logs() {
    kubectl logs -l app=rulo-api -n "$NAMESPACE" --tail=200 "$@"
}

# ---------- 入口 ----------
if [ $# -lt 1 ]; then
    usage
fi

case "$1" in
    apply)   cmd_apply ;;
    upgrade) cmd_upgrade ;;
    delete)  cmd_delete ;;
    restart) cmd_restart ;;
    status)  cmd_status ;;
    logs)    shift; cmd_logs "$@" ;;
    *)       usage ;;
esac
