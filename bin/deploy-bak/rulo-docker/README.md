# bin/rulo-docker — 本地构建 & 推送阿里云 ACR

本目录脚本在 **本地开发机** 上执行：
- 编译 `rulo-api` 二进制并打成 docker 镜像，推送到阿里云私有镜像仓库
- 拉取中间件官方镜像（postgres / redis / rustfs），转推到同一仓库供 K3s 集群拉取

> K3s 节点上的部署脚本在 `bin/rulo-k3s/`，本目录与之解耦，不要混用。

## 目录结构

| 文件 | 作用 |
|---|---|
| `registry.env` | 阿里云 ACR 仓库地址、命名空间、账号密码（被各脚本 source） |
| `Dockerfile.api` | rulo-api 运行镜像（产物模式，体积最小） |
| `build.sh` | `cargo build --release` + `docker build` |
| `push.sh` | docker login + 推送 rulo-api 到 ACR |
| `push-middleware.sh` | 拉取/转推 postgres、redis、rustfs |

## 快速使用

```bash
cd bin/rulo-docker

# 1. 一次性：把中间件镜像推到 ACR（K3s 节点拉不到 DockerHub 时需要）
./push-middleware.sh all

# 2. 业务发版：编译 + 构建 + 推送 rulo-api
./build.sh
./push.sh

# 自定义 tag
TAG=v1.0.0 ./build.sh
TAG=v1.0.0 ./push.sh

# 只重新打包不重新编译（已有 release 产物时）
./build.sh --skip-cargo
```

## 镜像清单

| 名称 | 推送到 ACR 的地址 |
|---|---|
| rulo-api | `CHANGE_ME_REGISTRY_HOST/rulo/rulo-api:<tag>` |
| postgres | `CHANGE_ME_REGISTRY_HOST/rulo/postgres:latest` |
| redis    | `CHANGE_ME_REGISTRY_HOST/rulo/redis:latest` |
| rustfs   | `CHANGE_ME_REGISTRY_HOST/rulo/rustfs:latest` |

## 修改密码 / 仓库

只改 `registry.env` 一处即可，所有脚本都从这里读取。
