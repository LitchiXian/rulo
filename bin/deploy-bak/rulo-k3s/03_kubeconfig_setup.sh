#!/bin/bash
set -e

echo "=============================="
echo "==> [3/5] 配置 kubectl 免 sudo"
echo "=============================="

if kubectl get nodes &>/dev/null; then
    echo "✅ kubectl 已可正常使用（无需 sudo）"
    return 0 2>/dev/null || exit 0
fi

echo ">>> 复制 K3s kubeconfig 到用户目录..."
mkdir -p "$HOME/.kube"
sudo cp /etc/rancher/k3s/k3s.yaml "$HOME/.kube/config"
sudo chown "$(id -u):$(id -g)" "$HOME/.kube/config"
export KUBECONFIG="$HOME/.kube/config"

# 写入 .bashrc，重启后也生效
if ! grep -q 'KUBECONFIG' "$HOME/.bashrc" 2>/dev/null; then
    echo '' >> "$HOME/.bashrc"
    echo '# K3s kubeconfig' >> "$HOME/.bashrc"
    echo 'export KUBECONFIG="$HOME/.kube/config"' >> "$HOME/.bashrc"
    echo ">>> 已写入 ~/.bashrc"
fi

echo ">>> 验证 kubectl..."
if kubectl get nodes; then
    echo ""
    echo "✅ kubectl 免 sudo 配置完成"
else
    echo ""
    echo "❌ kubectl 验证失败，尝试手动检查:"
    echo "   cat ~/.kube/config | head -5"
    echo "   kubectl get nodes --v=5"
    return 1 2>/dev/null || exit 1
fi
