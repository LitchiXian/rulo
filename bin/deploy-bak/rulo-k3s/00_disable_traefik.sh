#!/bin/bash
set -e

echo "=============================="
echo "==> [0/5] 禁用 K3s Traefik"
echo "=============================="

K3S_SERVICE=/etc/systemd/system/k3s.service

# 已经禁用过，跳过
if grep -q '\-\-disable traefik' "$K3S_SERVICE" 2>/dev/null; then
    echo "✅ Traefik 已禁用，跳过"
    exit 0
fi

echo ">>> 在 k3s.service 中添加 --disable traefik ..."
sudo sed -i '/^ExecStart=.*k3s server/ s/$/ --disable traefik/' "$K3S_SERVICE"

echo ">>> 重载 systemd 并重启 k3s（约 10 秒）..."
sudo systemctl daemon-reload
sudo systemctl restart k3s

echo ">>> 等待 k3s 恢复就绪..."
for i in $(seq 1 30); do
    if kubectl get nodes &>/dev/null; then
        break
    fi
    sleep 2
done

echo ">>> 清理残留 Traefik 资源..."
kubectl delete ns traefik --ignore-not-found=true 2>/dev/null || true
kubectl delete helmchart traefik traefik-crd -n kube-system --ignore-not-found=true 2>/dev/null || true

echo "✅ Traefik 已禁用，80/443 端口已释放"
