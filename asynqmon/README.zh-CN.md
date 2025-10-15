# Asynq Members - 任务控制面板

Asynq Members 是一个基于 Web 的任务控制面板，用于监控和管理 Asynq 队列和任务。它提供了一个现代化、用户友好的界面，灵感来自 [@hibiken/asynqmon](https://github.com/hibiken/asynqmon)。

## 功能特性

- 🖥️ **Web 界面** - 从任何现代 Web 浏览器访问控制面板
- 📊 **实时监控** - 实时查看队列统计、服务器信息和任务状态
- ⏸️ **队列管理** - 一键暂停和恢复队列
- 📋 **任务视图** - 浏览所有队列中的待处理和活跃任务
- 🔄 **自动刷新** - 每 5 秒自动更新数据
- 🎨 **现代化 UI** - 清晰、响应式设计，渐变色彩和流畅过渡

## 快速开始

### 前置条件

- Redis 服务器运行中（默认：`redis://127.0.0.1:6379`）
- 安装 Rust 工具链

### 构建

```bash
cargo build --bin members --release
```

### 运行

```bash
cargo run --bin members
```

或直接运行 release 二进制文件：

```bash
./target/release/members
```

Web 界面将在以下地址可用：**http://127.0.0.1:8080**

## 使用方法

### 连接到 Redis

首次打开 Web 界面时，会看到连接对话框。输入您的 Redis URL（默认为 `redis://127.0.0.1:6379`）并点击**连接**。

### 仪表板概览

连接后，仪表板将显示：

1. **服务器区域** - 显示所有运行的 Asynq 服务器：
   - 服务器 ID 和主机信息
   - 状态和并发设置
   - 活跃工作者数量
   - 严格优先级模式

2. **队列区域** - 显示所有队列：
   - 队列名称和状态（活跃/暂停）
   - 待处理、活跃和计划任务数量
   - 重试、归档和完成任务数量
   - 暂停/恢复控制

3. **任务区域** - 显示每个队列的任务：
   - 等待处理的待处理任务
   - 正在处理的活跃任务
   - 任务 ID、类型、队列和重试信息

### 队列管理

- **暂停队列**：点击任何队列卡片上的"⏸️ 暂停"按钮来暂停任务处理
- **恢复队列**：点击已暂停队列上的"▶️ 恢复"按钮来恢复处理
- **刷新数据**：点击工具栏中的"🔄 刷新"按钮手动更新数据

### 自动刷新

仪表板每 5 秒自动刷新数据，让您随时了解最新状态。

## API 端点

后端提供以下 REST API 端点：

### 连接
- `POST /api/connect` - 连接到 Redis
  ```json
  {
    "redis_url": "redis://127.0.0.1:6379"
  }
  ```

### 队列
- `GET /api/queues` - 获取所有队列名称
- `GET /api/queue/{name}` - 获取详细队列信息

### 服务器
- `GET /api/servers` - 获取所有服务器信息

### 任务
- `GET /api/tasks/{queue}/{state}` - 按队列和状态获取任务
  - 状态：`pending`、`active`、`scheduled`、`retry`、`archived`、`completed`

### 队列控制
- `POST /api/pause/{queue}` - 暂停队列
- `POST /api/unpause/{queue}` - 恢复队列

## 配置

应用程序默认在 `127.0.0.1:8080` 上运行。要自定义，请修改 `main.rs` 文件：

```rust
let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
```

## 架构

members 二进制文件包含：

1. **API 层** (`api.rs`) - 使用 Axum 框架的 REST API 处理器
2. **Inspector 服务** (`inspector_service.rs`) - Asynq Inspector API 的包装器
3. **Web UI** (`static/index.html`) - 使用原生 HTML/CSS/JavaScript 的单页应用

## 与 asynqmon 的对比

此实现提供与基于 Go 的 asynqmon 类似的功能，但使用 Rust 编写，并与 Rust asynq 库紧密集成：

| 功能 | asynqmon (Go) | members (Rust) |
|------|---------------|----------------|
| 队列监控 | ✅ | ✅ |
| 任务查看 | ✅ | ✅ |
| 队列控制 | ✅ | ✅ |
| Web UI | ✅ | ✅ |
| 桌面应用 | ❌ | 🚧 (计划中) |
| 语言 | Go | Rust |
| 集成 | Go asynq | Rust asynq |

## 未来增强

- [ ] 使用 Dioxus desktop 的桌面应用支持
- [ ] 任务详情视图，带有 payload 检查
- [ ] 任务删除和重试控制
- [ ] 历史统计和图表
- [ ] 多 Redis 实例支持
- [ ] 认证和授权
- [ ] WebSocket 支持实时更新

## 开发

### 项目结构

```
members/
├── src/
│   ├── main.rs              # 入口点和服务器设置
│   ├── api.rs               # REST API 处理器
│   └── inspector_service.rs # Inspector 服务包装器
└── static/
    └── index.html           # Web UI (HTML/CSS/JavaScript)
```

### 添加新功能

1. 在 `api.rs` 中添加 API 端点
2. 如需要，在 `inspector_service.rs` 中添加 Inspector 方法
3. 在 `static/index.html` 中更新 UI
4. 在 `main.rs` 中注册新路由

## 许可证

此项目采用与 asynq 库相同的许可条款（MIT OR GPL-3.0）。

## 致谢

- 灵感来自 [@hibiken/asynqmon](https://github.com/hibiken/asynqmon)
- 使用 [Axum](https://github.com/tokio-rs/axum) Web 框架构建
- 使用 [Asynq](https://github.com/cn-kali-team/asynq) Rust 库
