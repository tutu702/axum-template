# Axum-Template

一个基于 Rust [Axum](https://github.com/tokio-rs/axum) 框架构建的 Web 服务模板,集成了 JWT 认证、SQLx 多数据库驱动、统一错误处理与结构化日志,可作为新项目的起点快速扩展。

## ✨ 特性

- 🚀 **Axum 0.8** — 基于 Tokio 的高性能异步 Web 框架
- 🔐 **JWT 认证** — `Authorization: Bearer <token>`,支持自定义过期时间
- 🗄️ **多数据库驱动** — 通过 `sqlx::any` 同时支持 SQLite 与 PostgreSQL,运行时按 URL 自动选择
- ⚙️ **分层配置** — 默认值 + `config.yaml` + `APP_*` 环境变量 + `.env` 文件,优先级从低到高
- 🛡️ **统一错误响应** — `AppError` 自动映射为标准化的 JSON 响应
- 📜 **结构化日志** — `tracing` + `tracing-subscriber`,支持按 `RUST_LOG` 过滤

## 🚀 快速开始

### 环境要求

- Rust 工具链见 [rust-toolchain.toml](./rust-toolchain.toml)
- (可选)PostgreSQL 或 SQLite 用于数据库功能

### 启动服务

```bash
# 克隆并进入项目
git clone https://github.com/tutu702/axum-template
cd axum-template

# 直接运行(将使用内置默认值)
cargo run
```

## ⚙️ 配置

### 优先级(从低到高)

1. **代码内默认值** —— 见 [`config.rs`](src/config.rs) 中各类型的 `impl Default`
2. **`config.yaml`** —— 项目根目录,可选
3. **`APP_*` 环境变量** —— 使用 `APP_` 前缀与 `_` 分隔符
4. **`.env` 文件** —— 通过 `dotenvy` 加载到进程环境(可选)

### 默认值一览

| 字段                      | 默认值                   |
| ------------------------- | ------------------------ |
| `server.host`             | `0.0.0.0`                |
| `server.port`             | `3000`                   |
| `auth.username`           | `admin`                  |
| `auth.password`           | `123456`                 |
| `auth.secret`             | `axum-template`          |
| `auth.expireMinutes`      | `30`                     |
| `database.driver`         | `sqlite3`                |
| `database.url`            | `sqlite://./data/app.db` |
| `database.maxConnections` | `10`                     |
| `database.minConnections` | `1`                      |
| `database.acquireTimeout` | `30`                     |

### `config.yaml` 示例

```yaml
server:
  host: 127.0.0.1
  port: 8080

auth:
  username: admin
  password: s3cr3t
  secret: please-change-me-in-production
  expireMinutes: 60

database:
  driver: pgsql # 或 sqlite3
  url: postgres://user:pass@localhost:5432/axum_template
  maxConnections: 20
  minConnections: 2
  acquireTimeout: 10
```

### 环境变量示例

环境变量使用 `APP_` 前缀,嵌套字段用 `_` 连接,驼峰字段保留原名:

```bash
export APP_SERVER__PORT=8080
export APP_AUTH__USERNAME=alice
export APP_AUTH__PASSWORD=hunter2
export APP_AUTH__SECRET=super-secret-key
export APP_AUTH__EXPIREMINUTES=120
export APP_DATABASE__URL="postgres://user:pass@localhost:5432/mydb"
export APP_DATABASE__DRIVER=pgsql
```

`.env` 文件同样生效(在项目根目录创建即可):

```dotenv
APP_SERVER__PORT=8080
APP_AUTH__SECRET=super-secret-key
```

> ⚠️ **生产环境务必修改默认 `secret` 与默认凭证,切勿提交到版本库。**

## 📄 许可证

本项目基于 [MIT License](./LICENSE) 开源。
