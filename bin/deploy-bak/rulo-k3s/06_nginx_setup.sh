#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NGINX_CONF="${SCRIPT_DIR}/nginx.conf"
TARGET_DIR="/etc/nginx"
TARGET_FILE="$TARGET_DIR/nginx.conf"
TIMESTAMP=$(date +"%Y%m%d%H%M")

echo "=============================="
echo "==> 部署 Nginx 配置"
echo "=============================="

if [ ! -f "$NGINX_CONF" ]; then
    echo "❌ 未找到 nginx.conf: $NGINX_CONF"
    exit 1
fi

# 备份旧配置
if [ -f "$TARGET_FILE" ]; then
    BACKUP="$TARGET_DIR/nginx_${TIMESTAMP}.conf.bak"
    echo ">>> 备份旧配置 → $BACKUP"
    sudo cp "$TARGET_FILE" "$BACKUP"
fi

# 复制新配置（用 cp 而非 mv，保留源文件）
echo ">>> 复制新配置 → $TARGET_FILE"
sudo cp "$NGINX_CONF" "$TARGET_FILE"

# 测试配置
echo ">>> 测试 Nginx 配置..."
if sudo nginx -t; then
    echo ">>> 重载 Nginx..."
    sudo nginx -s reload
    echo ""
    echo "✅ Nginx 配置部署并重载完成"
else
    echo ""
    echo "❌ Nginx 配置有语法错误，已复制但未重载"
    echo "   请修复后手动执行: sudo nginx -s reload"
    exit 1
fi
