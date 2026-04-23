#!/bin/bash
set -e

echo "=============================="
echo "==> [1/5] 处理 pause 基础镜像"
echo "=============================="

PAUSE_DST="docker.io/rancher/mirrored-pause:3.6"

# 已存在就跳过
if sudo ctr -n k8s.io images check | grep -q "$PAUSE_DST"; then
    echo "✅ pause 镜像已存在，跳过"
    exit 0
fi

PAUSE_SRC="registry.cn-hangzhou.aliyuncs.com/google_containers/pause:3.6"

echo ">>> 从阿里云拉取 pause 镜像..."
sudo ctr -n k8s.io images pull "$PAUSE_SRC"

echo ">>> 打 tag..."
sudo ctr -n k8s.io images tag "$PAUSE_SRC" "rancher/mirrored-pause:3.6" 2>/dev/null || true
sudo ctr -n k8s.io images tag "$PAUSE_SRC" "$PAUSE_DST" 2>/dev/null || true

echo "✅ pause 镜像处理完成"
