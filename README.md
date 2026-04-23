<h1 align="center">
	<img src="rulo-admin/public/rulo.ico" width="48" height="48" alt="Rulo logo" style="vertical-align: middle; margin-right: 10px;" />
	<span style="vertical-align: middle;">Rulo</span>
</h1>
<p align="center">A Rust-based admin platform inspired by the RuoYi family of management systems</p>
<div align="center">English | <a href="./README.zh-CN.md">简体中文</a></div>

<br />

## Overview

Rulo currently consists of two parts:

- `rulo-api`: Rust backend based on Axum, with SQLx, Redis, JWT and tracing wired in
- `rulo-admin`: Vue 3 + Vite frontend, focused on system management and monitoring use cases

## Screenshots

<table border="0" cellspacing="0" cellpadding="0">
  <tr>
    <td width="50%"><img src="assets/photo1.png" alt="Dashboard – dark mode" /></td>
    <td width="50%"><img src="assets/photo2.png" alt="Department management – light mode" /></td>
  </tr>
</table>

## Repository Structure

```text
.
├── rulo-api/           # Rust backend workspace
│   ├── app/            # Axum application crate
│   └── common/         # Shared types, errors, utils, state
├── rulo-admin/         # Frontend (Vue 3 + Vite)
├── bin/
│   ├── deploy/         # Active deployment scripts (Docker / K3s)
│   └── deploy-bak/     # Reference deployment scripts (demo / backup)
├── doc/                # Project notes and roadmap
└── sql/                # SQL scripts / historical SQL files
```

## Tech Stack

### Backend

| Area | Technology | Notes |
|------|------------|-------|
| Language / Runtime | Rust + Tokio | Memory safety, zero-cost async, no GC |
| Web Framework | Axum | Ergonomic, modular, built on Tower middleware |
| Database | SQLx + PostgreSQL | Async, compile-time SQL verification |
| Cache | deadpool-redis + Redis | Async connection pool, session and rate-limit store |
| Auth | JWT + Argon2 | Stateless token auth; modern password hashing standard |
| Logging | tracing | Structured, async-aware, file rolling support |
| Config | config-rs | TOML base + environment variable override |
| Macros | syn + quote | Procedural macro for compile-time permission checks |

### Frontend

| Area | Technology | Notes |
|------|------------|-------|
| Framework | Vue 3 + TypeScript | Composition API, full type safety |
| Build Tool | Vite | Fast HMR, optimized production builds |
| UI Library | Element Plus | Rich component set, fits admin panel style |
| Routing | Vue Router | Official router, history mode |
| State | Pinia | Lightweight, type-safe store |
| HTTP | Axios | Interceptor-based request/response handling |

## Quick Start

### Prerequisites

- Rust toolchain
- Node.js 18+
- PostgreSQL
- Redis

### 1. Start the backend

Backend config is loaded from `rulo-api/config/default.toml`; the default server port is `13000`.

Make sure PostgreSQL and Redis are running, then set your database connection string:

```bash
export DATABASE_URL=postgres://username:password@127.0.0.1:5432/your_db
```

Start the backend:

```bash
cd rulo-api
cargo run -p app
```

> Run the SQL files / migrations under `migrations/` first if your database schema is not initialized.

### 2. Start the frontend

The frontend runs on port `9999`. Its development backend target is configured in `rulo-admin/public/config.json`.

```bash
cd rulo-admin
npm install
npm run dev
```

## Deployment

See `bin/deploy-bak/` and the `RULO_README.md` inside for step-by-step instructions.

## Default Accounts

The following accounts are seeded by the init migration (`migrations/20260423000000_init.sql`) and are intended for local trial use only.

| Username   | Password         | Role        |
|------------|------------------|-------------|
| `sadmin`   | `sadmin123.Aa`   | Super Admin |
| `admin`    | `admin123.Aa`    | Admin       |
| `common01` | `common01123.Aa` | Common User |
| `test01`   | `test01123.Aa`   | Test User   |
| `test02`   | `test02123.Aa`   | Test User   |

> **Note:** Change all default passwords before deploying to any public environment.

## Development Notes

- The frontend uses `/api` proxying in development
- Frontend menus and routes are still registered statically; dynamic menu/route generation is a planned next step
- The backend is being evolved toward a more complete RBAC-driven management platform

## Roadmap

See the project roadmap and next priorities in [doc/todo.md](doc/todo.md).

Current focus areas:

- dynamic menus and dynamic routes
- RBAC closure between users, roles, menus, and permissions
- pagination and generic list querying
- profile/avatar backend support
- health checks and containerized deployment

## Community

Questions and feedback are welcome — join the QQ group: **1093744757**

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.
