#!/bin/bash
set -e

export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/config}"

echo "=============================="
echo "==> [2/5] 修复 CoreDNS 镜像"
echo "=============================="

# 动态获取 CoreDNS Pod 信息
COREDNS_POD=$(kubectl get pod -n kube-system -l k8s-app=kube-dns -o jsonpath='{.items[0].metadata.name}' 2>/dev/null)

if [ -z "$COREDNS_POD" ]; then
    echo "❌ 未找到 CoreDNS Pod"
    exit 1
fi

COREDNS_IMAGE=$(kubectl get pod "$COREDNS_POD" -n kube-system -o jsonpath='{.spec.containers[0].image}')
echo "CoreDNS Pod: $COREDNS_POD"
echo "CoreDNS Image: $COREDNS_IMAGE"

# 检查 CoreDNS 是否已经正常运行
COREDNS_STATUS=$(kubectl get pod "$COREDNS_POD" -n kube-system -o jsonpath='{.status.phase}')
COREDNS_READY=$(kubectl get pod "$COREDNS_POD" -n kube-system -o jsonpath='{.status.containerStatuses[0].ready}')
if [ "$COREDNS_STATUS" = "Running" ] && [ "$COREDNS_READY" = "true" ]; then
    echo "✅ CoreDNS 已正常运行，跳过修复"
    exit 0
fi

# 提取版本号
VERSION=$(echo "$COREDNS_IMAGE" | awk -F: '{print $2}')
if [[ ! "$VERSION" =~ ^v ]]; then
    VERSION="v${VERSION}"
fi
echo "版本号: $VERSION"

# 从阿里云拉取并打 tag
ALI_IMAGE="registry.cn-hangzhou.aliyuncs.com/google_containers/coredns:${VERSION}"
echo ">>> 从阿里云拉取 CoreDNS 镜像..."
sudo ctr -n k8s.io images pull "$ALI_IMAGE"

echo ">>> 打 tag..."
if [[ "$COREDNS_IMAGE" != docker.io/* ]]; then
    FULL_IMAGE="docker.io/${COREDNS_IMAGE}"
else
    FULL_IMAGE="$COREDNS_IMAGE"
fi
sudo ctr -n k8s.io images tag "$ALI_IMAGE" "$FULL_IMAGE" 2>/dev/null || true
sudo ctr -n k8s.io images tag "$ALI_IMAGE" "$COREDNS_IMAGE" 2>/dev/null || true

# 删除旧 Pod 触发重建
echo ">>> 删除旧 Pod 触发重建..."
kubectl delete pod "$COREDNS_POD" -n kube-system

echo "⏳ 等待 CoreDNS 重建..."
if kubectl wait --for=condition=ready pod -l k8s-app=kube-dns -n kube-system --timeout=60s 2>/dev/null; then
    echo ""
    echo "✅ CoreDNS 修复完成"
else
    echo "⚠️  CoreDNS 启动超时，检查状态:"
fi

kubectl get pods -n kube-system -l k8s-app=kube-dns
