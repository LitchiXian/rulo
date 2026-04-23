# Rulo K3s 服务器端部署说明

> 参考 `k3s_deploy_script` 的模式：初始化跑一次，日常更新一条命令。

## 文件清单

| 文件 | 说明 |
|------|------|
| `01_pause_image.sh` | 处理 pause 基础镜像（国内拉不到 Docker Hub，从阿里云拉） |
| `02_coredns_fix.sh` | 修复 CoreDNS 镜像（同上，DNS 不工作 Pod 全卡住） |
| `03_kubeconfig_setup.sh` | 配置 kubectl 免 sudo + 写入 .bashrc 持久化 |
| `04_registry_secret.sh` | 创建 namespace + aliyun-auth Secret（幂等） |
| `05_deploy.sh` | **主脚本** — 部署管理（apply/upgrade/delete/restart/status/logs） |
| `06_nginx_setup.sh` | 部署 Nginx 配置（备份 + 测试 + 重载） |
| `init_rulo_env.sh` | 一键初始化（串联 01→02→03→04→05） |
| `k3s-deploy.yaml` | K3s 部署清单（部署前改密码） |
| `nginx.conf` | Nginx 配置（按需改 IP/域名） |

## 首次部署

### 1. 准备

```bash
# 本地先推送镜像（构建脚本位于 rulo-api/项目根等位置，请按实际路径调整）
./build.sh && ./push.sh     # rulo-api 镜像

# 整个目录传到服务器
scp -r bin/deploy/rulo-k3s/ <服务器>:~/rulo-k3s/
scp rulo-api/config/default.toml <服务器>:/opt/rulo/api/config/
```

### 2. 改密码（先改再 init）

修改服务器上 `rulo-k3s/k3s-deploy.yaml` 里的 `rulo-secrets`。

### 3. 一键初始化

```bash
cd ~/rulo-k3s
chmod +x *.sh
./init_rulo_env.sh
```

自动完成 5 步：
1. pause 基础镜像（已有则跳过）
2. CoreDNS 修复（已正常则跳过）
3. kubectl 免 sudo + .bashrc 持久化
4. 创建 namespace + aliyun-auth Secret
5. 部署全部 K3s 资源

### 4. Nginx（可选）

```bash
./06_nginx_setup.sh
```

## 日常更新

```bash
# === 本地 ===
./build.sh && ./push.sh

# === 服务器 ===
./05_deploy.sh upgrade
```

不需要碰 secret，不需要 delete。

## 05_deploy.sh 命令一览

| 命令 | 说明 |
|------|------|
| `apply` | 部署全部资源 |
| `upgrade` | apply + 滚动重启（**日常用这个**） |
| `restart` | 仅重启 rulo-api |
| `delete` | 删除业务资源（namespace 和 secret 保留，重新 apply 即可） |
| `status` | 查看状态 |
| `logs [-f]` | 查看日志 |

## 端口说明

| 服务 | 容器端口 | NodePort |
|------|----------|----------|
| rulo-api | 13000 | 30130 |
| PostgreSQL | 5432 | 30432 |
| RustFS API | 9000 | 30090 |
| RustFS Console | 9001 | 30091 |
| Redis | 6379 | — (ClusterIP) |
