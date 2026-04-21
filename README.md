<h1 align="center">
	<img src="rulo-admin/public/rulo.ico" width="48" height="48" alt="Rulo logo" style="vertical-align: middle; margin-right: 10px;" />
	<span style="vertical-align: middle;">Rulo</span>
</h1>
<p align="center">A Rust-based management system inspired by the RuoYi style of admin platforms</p>
<div align="center">English | <a href="./README.zh-CN.md">简体中文</a></div>

<br />

## Overview

Rulo is built around two active parts:

- `rulo-api`: Rust backend based on Axum, SQLx, Redis, JWT, and tracing
- `rulo-admin`: Vue 3 + Vite admin panel for system management and monitoring

## Current Features

### Backend

- PostgreSQL connection pooling with SQLx
- Redis connection pooling with deadpool-redis
- JWT authentication middleware
- Password hashing with Argon2
- Unified error handling and JSON response format
- Logging with tracing to terminal and rolling log files
- Config loading via TOML
- System modules:
	- auth
	- user
	- role
	- menu
	- permission
	- monitor

### Admin Frontend

- Login and auth flow
- Dashboard / home page
- User, role, menu, and permission management pages
- Server monitoring page backed by real backend system data
- Changelog page
- Personal center page
- Five layout modes
- Light / dark / system theme modes
- Runtime API config and development proxy support

## Repository Structure

```text
.
├── rulo-api/           # Rust backend workspace
│   ├── app/            # Axum application crate
│   └── common/         # Shared types, errors, utils, state
├── rulo-admin/         # Admin frontend (Vue 3 + Vite)
├── bin/                # Build & deployment scripts (Docker / K3s)
├── doc/                # Project notes and roadmap
└── sql/                # SQL scripts / historical SQL files
```

## Tech Stack

### Backend

- Rust
- Axum
- SQLx
- PostgreSQL
- Redis
- JSON Web Token
- tracing
- config

### Frontend

- Vue 3
- TypeScript
- Vite
- Vue Router
- Pinia
- Element Plus
- Axios

## Quick Start

## Prerequisites

- Rust toolchain
- Node.js 18+
- PostgreSQL
- Redis

## 1. Start the backend

Backend config is loaded from `rulo-api/config/default.toml`.
The default server port is `13000`.

Set your database connection string first:

```bash
export DATABASE_URL=postgres://username:password@127.0.0.1:5432/your_db
```

Then start the backend:

```bash
cd rulo-api
cargo run -p app
```

Notes:

- Ensure PostgreSQL is available
- Ensure Redis is available
- If your schema is not initialized yet, run your SQL or migrations first

## 2. Start the admin frontend

The admin frontend runs on port `9999`.
Its runtime backend target is configured in `rulo-admin/public/config.json`.

```bash
cd rulo-admin
npm install
npm run dev
```

## Development Notes

- The frontends use `/api` proxying in development
- The admin frontend currently uses static routes; dynamic menu/route generation is a planned next step
- The backend is being evolved toward a more complete RBAC-driven management platform

## Roadmap

See the project roadmap and next priorities here:

- [doc/todo.md](doc/todo.md)

Current major next steps include:

- dynamic menus and dynamic routes
- RBAC closure between users, roles, menus, and permissions
- pagination and generic list querying
- profile/avatar backend support
- health checks and containerized deployment

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.
