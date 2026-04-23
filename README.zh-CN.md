<h1 align="center">
  <img src="rulo-admin/public/rulo.ico" width="48" height="48" alt="Rulo logo" style="vertical-align: middle; margin-right: 10px;" />
  <span style="vertical-align: middle;">Rulo</span>
</h1>
<p align="center">一个基于 Rust 异步生态构建的后台管理系统，整体风格参考 RuoYi 系列</p>
<div align="center"><a href="./README.md">English</a> | 简体中文</div>

<br />

## 项目概览

Rulo 目前由两部分组成：

- `rulo-api`：基于 Axum 的 Rust 后端，集成 SQLx、Redis、JWT、tracing 等常用套件
- `rulo-admin`：基于 Vue 3 + Vite 的前端，面向后台管理与系统监控场景

## 预览

<table border="0" cellspacing="0" cellpadding="0">
  <tr>
    <td width="50%"><img src="assets/photo1.png" alt="Dashboard 首页（暗色）" /></td>
    <td width="50%"><img src="assets/photo2.png" alt="部门管理（亮色）" /></td>
  </tr>
</table>

## 仓库结构

```text
.
├── rulo-api/           # Rust 后端 workspace
│   ├── app/            # Axum 应用入口
│   └── common/         # 公共类型、错误、工具、共享状态
├── rulo-admin/         # 前端（Vue 3 + Vite）
├── bin/
│   ├── deploy/         # 当前生效的部署脚本（Docker / K3s）
│   └── deploy-bak/     # 参考用部署脚本（示例 / 备份）
├── doc/                # 项目笔记与路线图
└── sql/                # SQL 脚本 / 历史 SQL 文件
```

## 技术栈

### 后端

| 领域 | 技术选型 | 说明 |
|------|--------|------|
| 语言 / 运行时 | Rust + Tokio | 内存安全、零开销异步、无 GC 停顿 |
| Web 框架 | Axum | 模块化、符合人体工程学，基于 Tower 中间件生态 |
| 数据库 | SQLx + PostgreSQL | 异步、编译期 SQL 校验 |
| 缓存 | deadpool-redis + Redis | 异步连接池，用于会话与限流 |
| 认证授权 | JWT + Argon2 | 无状态 Token 认证；现代密码哈希标准 |
| 日志 | tracing | 结构化、异步感知，支持滚动文件 |
| 配置 | config-rs | TOML 基础配置 + 环境变量覆盖 |
| 宏 | syn + quote | 自定义过程宏，编译期权限校验 |

### 前端

| 领域 | 技术选型 | 说明 |
|------|--------|------|
| 框架 | Vue 3 + TypeScript | 组合式 API，全量类型安全 |
| 构建工具 | Vite | 极速 HMR，生产构建优化 |
| UI 组件库 | Element Plus | 组件丰富，适合后台管理风格 |
| 路由 | Vue Router | 官方路由方案，history 模式 |
| 状态管理 | Pinia | 轻量、类型安全的状态库 |
| HTTP | Axios | 拦截器机制，统一请求 / 响应处理 |

## 快速开始

### 环境准备

- Rust 工具链
- Node.js 18+
- PostgreSQL
- Redis

### 1. 启动后端

后端配置文件位于 `rulo-api/config/default.toml`，默认服务端口为 `13000`。

请先确保 PostgreSQL 与 Redis 已可用，然后设置数据库连接串：

```bash
export DATABASE_URL=postgres://username:password@127.0.0.1:5432/your_db
```

启动后端：

```bash
cd rulo-api
cargo run -p app
```

> 首次启动前请先执行 `migrations/` 下的 SQL 或对应迁移，以初始化数据库结构。

### 2. 启动前端

前端默认端口为 `9999`，开发环境下的后端代理目标配置于 `rulo-admin/public/config.json`。

```bash
cd rulo-admin
npm install
npm run dev
```

## 默认账号

以下账号由初始化迁移文件（`migrations/20260423000000_init.sql`）预置，仅供本地体验使用。

| 用户名     | 密码             | 角色       |
|------------|------------------|------------|
| `sadmin`   | `sadmin123.Aa`   | 超级管理员 |
| `admin`    | `admin123.Aa`    | 管理员     |
| `common01` | `common01123.Aa` | 普通用户   |
| `test01`   | `test01123.Aa`   | 测试用户   |
| `test02`   | `test02123.Aa`   | 测试用户   |

> **注意：** 部署到任何公网环境前，请先修改所有默认密码。

## 开发说明

- 前端开发环境通过 `/api` 代理请求后端
- 当前前端菜单与路由仍以静态方式注册，后续计划迁移到动态菜单 / 动态路由
- 后端正朝完整 RBAC 管理平台的方向持续演进

## 部署

参考 `bin/deploy-bak/`，详细步骤见其中的 `RULO_README.md`。

## 路线图

项目路线图与下一步优先事项参见 [doc/todo.md](doc/todo.md)。

当前重点包括：

- 动态菜单与动态路由
- 用户 / 角色 / 菜单 / 权限的 RBAC 闭环
- 通用分页与列表查询能力
- 个人中心头像等后端字段支持
- 健康检查与容器化部署

## 交流

欢迎加 QQ 群一起交流、提问或提意见：**1093744757**

## 许可证

本项目遵循 [LICENSE](LICENSE) 文件中的许可条款。