#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=============================="
echo "🚀 开始初始化 Rulo K3s 环境"
echo "=============================="
echo ""

# 0. 禁用 Traefik，释放 80/443 给宿主机 Nginx
bash "$SCRIPT_DIR/00_disable_traefik.sh"
echo ""

# 1. pause 基础镜像（不需要 kubectl，只用 ctr）
bash "$SCRIPT_DIR/01_pause_image.sh"
echo ""

# 2. kubectl 免 sudo（source 让 export KUBECONFIG 留在当前 shell）
source "$SCRIPT_DIR/03_kubeconfig_setup.sh"
echo ""

# 3. CoreDNS 修复（需要 kubectl，放在 kubeconfig 之后）
bash "$SCRIPT_DIR/02_coredns_fix.sh"
echo ""

# 4. 创建 namespace + aliyun-auth Secret
bash "$SCRIPT_DIR/04_registry_secret.sh"
echo ""

# 5. 部署全部 K3s 资源
echo "==> [5/5] 部署 K3s 资源"
bash "$SCRIPT_DIR/05_deploy.sh" apply
echo ""

echo "=============================="
echo "🎉 Rulo 环境初始化完成"
echo "=============================="
echo ""

echo "日常更新只需: ./05_deploy.sh upgrade"
