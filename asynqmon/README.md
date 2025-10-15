# Asynq Members - Task Control Panel

Asynq Members is a web-based task control panel for monitoring and managing Asynq queues and tasks. It provides a modern, user-friendly interface inspired by [@hibiken/asynqmon](https://github.com/hibiken/asynqmon).

## Features

- 🖥️ **Web-based Interface** - Access the control panel from any modern web browser
- 📊 **Real-time Monitoring** - View queue statistics, server information, and task status in real-time
- ⏸️ **Queue Management** - Pause and resume queues with a single click
- 📋 **Task Views** - Browse pending and active tasks across all queues
- 🔄 **Auto-refresh** - Automatic data updates every 5 seconds
- 🎨 **Modern UI** - Clean, responsive design with gradient colors and smooth transitions

## Quick Start

### Prerequisites

- Redis server running (default: `redis://127.0.0.1:6379`)
- Rust toolchain installed

### Building

```bash
cargo build --bin asynqmon --release
```

### Running

```bash
cargo run --bin asynqmon
```

Or run the release binary directly:

```bash
./target/release/asynqmon
```

The web interface will be available at: **http://127.0.0.1:8080**

## Usage

### Connecting to Redis

When you first open the web interface, you'll see a connection dialog. Enter your Redis URL (default is `redis://127.0.0.1:6379`) and click **Connect**.

### Dashboard Overview

Once connected, the dashboard displays:

1. **Servers Section** - Shows all running Asynq servers with:
   - Server ID and host information
   - Status and concurrency settings
   - Active worker count
   - Strict priority mode

2. **Queues Section** - Displays all queues with:
   - Queue name and status (Active/Paused)
   - Pending, Active, and Scheduled task counts
   - Retry, Archived, and Completed task counts
   - Pause/Resume controls

3. **Tasks Section** - Shows tasks for each queue:
   - Pending tasks waiting to be processed
   - Active tasks currently being processed
   - Task ID, type, queue, and retry information

### Queue Management

- **Pause Queue**: Click the "⏸️ Pause" button on any queue card to pause task processing
- **Resume Queue**: Click the "▶️ Resume" button on a paused queue to resume processing
- **Refresh Data**: Click the "🔄 Refresh" button in the toolbar to manually update data

### Auto-refresh

The dashboard automatically refreshes data every 5 seconds to keep you updated with the latest status.

## API Endpoints

The backend provides the following REST API endpoints:

### Connection
- `POST /api/connect` - Connect to Redis
  ```json
  {
    "redis_url": "redis://127.0.0.1:6379"
  }
  ```

### Queues
- `GET /api/queues` - Get all queue names
- `GET /api/queue/:name` - Get detailed queue information

### Servers
- `GET /api/servers` - Get all server information

### Tasks
- `GET /api/tasks/:queue/:state` - Get tasks by queue and state
  - States: `pending`, `active`, `scheduled`, `retry`, `archived`, `completed`

### Queue Control
- `POST /api/pause/:queue` - Pause a queue
- `POST /api/unpause/:queue` - Resume a queue

## Configuration

The application runs on `127.0.0.1:8080` by default. To customize, modify the `main.rs` file:

```rust
let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
```

## Architecture

The asynqmon binary consists of:

1. **API Layer** (`api.rs`) - REST API handlers using Actix-web framework
2. **Inspector Service** (`inspector_service.rs`) - Wrapper around Asynq Inspector API
3. **Web UI** (`static/index.html`) - Single-page application with vanilla HTML/CSS/JavaScript
4. **Dioxus UI** (`ui.rs`) - Modular Dioxus components for building reactive UIs (ready for WebAssembly deployment)

## Comparison with asynqmon

This implementation provides similar functionality to the Go-based asynqmon but is written in Rust and tightly integrated with the Rust asynq library:

| Feature | asynqmon (Go) | members (Rust) |
|---------|---------------|----------------|
| Queue monitoring | ✅ | ✅ |
| Task viewing | ✅ | ✅ |
| Queue control | ✅ | ✅ |
| Web UI | ✅ | ✅ |
| Desktop app | ❌ | 🚧 (Planned) |
| Language | Go | Rust |
| Integration | Go asynq | Rust asynq |

## Future Enhancements

- [ ] Desktop application support using Dioxus desktop
- [ ] Task detail view with payload inspection
- [ ] Task deletion and retry controls
- [ ] Historical statistics and charts
- [ ] Multiple Redis instance support
- [ ] Authentication and authorization
- [ ] WebSocket support for real-time updates

## Development

### Project Structure

```
asynqmon/
├── src/
│   ├── main.rs              # Entry point and server setup
│   ├── api.rs               # REST API handlers
│   ├── inspector_service.rs # Inspector service wrapper
│   └── ui.rs                # Dioxus UI components (for WebAssembly)
└── static/
    ├── index.html           # Web UI (HTML/CSS/JavaScript)
    └── dioxus.html          # Entry point for Dioxus WebAssembly app
```

### Adding New Features

1. Add API endpoints in `api.rs`
2. Add Inspector methods in `inspector_service.rs` if needed
3. Update the UI in `static/index.html` for vanilla JS version
4. Update Dioxus components in `ui.rs` for reactive version
5. Register new routes in `main.rs`

### Dioxus UI Components

The project includes Dioxus UI components in `ui.rs` that demonstrate how to build reactive interfaces with Dioxus. These components include:

- `App` - Main application component
- `Header` - Application header with branding
- `ConnectionDialog` - Redis connection interface
- `Dashboard` - Main dashboard view
- `ServerCard` - Server information card
- `QueueCard` - Queue statistics card

The Dioxus components can be compiled to WebAssembly for deployment in modern browsers. To build the WebAssembly version:

```bash
# Install dioxus-cli
cargo install dioxus-cli

# Build the WebAssembly app
dx build --release
```

## License

This project is licensed under the same terms as the asynq library (MIT OR GPL-3.0).

## Acknowledgments

- Inspired by [@hibiken/asynqmon](https://github.com/hibiken/asynqmon)
- Built with [Actix-web](https://actix.rs/) web framework
- UI built with [Dioxus](https://dioxuslabs.com/) reactive framework
- Uses the [Asynq](https://github.com/cn-kali-team/asynq) Rust library
