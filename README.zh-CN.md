<h1 align="center">
  <img src="rulo-admin/public/rulo.ico" width="48" height="48" alt="Rulo logo" style="vertical-align: middle; margin-right: 10px;" />
  <span style="vertical-align: middle;">Rulo</span>
</h1>
<p align="center">一个基于 Rust 异步生态构建的管理系统项目，整体风格参考 RuoYi 一类后台平台</p>
<div align="center"><a href="./README.md">English</a> | 简体中文</div>

<br />

## 项目概览

当前主线由三部分组成：

- `rulo-server`：基于 Axum 的 Rust 后端
- `rulo-ui-admin`：基于 Vue 3 + Vite 的后台管理端
- `rulo-ui-client`：基于 Vue 3 + Vite 的客户端前端

## 当前已具备的能力

### 后端

- PostgreSQL 连接池（SQLx）
- Redis 连接池（deadpool-redis）
- JWT 登录鉴权中间件
- Argon2 密码哈希
- 统一错误处理与统一 JSON 响应体
- tracing 日志（终端 + 文件滚动）
- TOML 配置加载
- 已有系统模块：
  - auth
  - user
  - role
  - menu
  - permission
  - monitor

### 管理端前端

- 登录与鉴权流程
- 首页 Dashboard
- 用户 / 角色 / 菜单 / 权限管理页面
- 已与后端联调的服务监控页面
- 更新日志页面
- 个人中心页面
- 5 种布局模式
- 亮色 / 暗色 / 跟随系统三种主题模式
- 运行时配置与开发代理支持

### 客户端前端

- Vue 3 + Element Plus 应用骨架
- 登录相关路由结构
- Markdown 渲染能力
- 与管理端一致的运行时配置模式

## 仓库结构

```text
.
├── rulo-server/        # Rust 后端 workspace
│   ├── rulo-app/       # Axum 应用入口
│   └── rulo-common/    # 公共类型、错误、工具、共享状态
├── rulo-ui-admin/      # 管理端前端（Vue 3 + Vite）
├── rulo-ui-client/     # 客户端前端（Vue 3 + Vite）
├── doc/                # 项目笔记与路线图
└── sql/                # SQL 脚本 / 历史 SQL 文件
```

## 技术栈

### 后端

- Rust
- Axum
- SQLx
- PostgreSQL
- Redis
- JSON Web Token
- tracing
- config

### 前端

- Vue 3
- TypeScript
- Vite
- Vue Router
- Pinia
- Element Plus
- Axios

## 快速开始

## 环境准备

- Rust 工具链
- Node.js 18+
- PostgreSQL
- Redis

## 1. 启动后端

后端配置文件位于 `rulo-server/config/default.toml`。
默认服务端口为 `13000`。

先设置数据库连接串：

```bash
export DATABASE_URL=postgres://username:password@127.0.0.1:5432/your_db
```

然后启动后端：

```bash
cd rulo-server
cargo run -p app
```

说明：

- PostgreSQL 需要先可用
- Redis 需要先可用
- 如果数据库结构尚未初始化，请先执行对应 SQL 或迁移

## 2. 启动管理端

管理端默认端口是 `9999`。
开发环境代理目标配置在 `rulo-ui-admin/public/config.json` 中。

```bash
cd rulo-ui-admin
npm install
npm run dev
```

## 3. 启动客户端

客户端默认端口是 `8888`。
开发环境代理目标配置在 `rulo-ui-client/public/config.json` 中。

```bash
cd rulo-ui-client
npm install
npm run dev
```

## 开发说明

- 前端开发环境通过 `/api` 代理请求后端
- 当前管理端菜单与路由仍以静态方式注册，后续计划切到动态菜单 / 动态路由
- 后端正在朝完整 RBAC 管理平台方向继续演进

## 路线图

项目当前路线图与下一步优先事项见：

- [doc/todo.md](doc/todo.md)

当前最关键的下一步包括：

- 动态菜单与动态路由
- 用户 / 角色 / 菜单 / 权限的 RBAC 闭环
- 通用分页与列表查询能力
- 个人中心头像等后端字段支持
- 健康检查与容器化部署

## 许可证

本项目遵循 [LICENSE](LICENSE) 文件中的许可条款。