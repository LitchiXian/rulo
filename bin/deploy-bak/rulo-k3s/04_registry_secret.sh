#!/bin/bash
set -e

export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/config}"
NAMESPACE="rulo"

echo "=============================="
echo "==> [4/5] 创建 namespace + 镜像仓库 Secret"
echo "Namespace: $NAMESPACE"
echo "=============================="

# 确保 namespace 存在（幂等）
kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -

# 使用 dry-run + apply 实现幂等：存在则更新，不存在则创建
# 不再需要先 delete 再 create，也不会因为已存在而报错
kubectl create secret docker-registry aliyun-auth \
  --docker-server=CHANGE_ME_REGISTRY_HOST \
  --docker-username=CHANGE_ME_REGISTRY_USERNAME \
  --docker-password='CHANGE_ME_REGISTRY_PASSWORD' \
  --docker-email=none@example.com \
  -n "$NAMESPACE" \
  --dry-run=client -o yaml | kubectl apply -f -

echo ""
echo "✅ Secret aliyun-auth 创建/更新完成（namespace: $NAMESPACE）"
