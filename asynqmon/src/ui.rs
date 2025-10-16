//! Dioxus UI components for the web interface

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let mut redis_url = use_signal(|| "redis://127.0.0.1:6379".to_string());
    let mut connected = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());
    let queues = use_signal(|| Vec::<String>::new());
    let servers = use_signal(|| Vec::<ServerInfo>::new());

    rsx! {
        style { {GLOBAL_CSS} }
        div {
            Header {}
            div { class: "container",
                if !connected() {
                    ConnectionDialog {
                        redis_url: redis_url,
                        error_message: error_message,
                        on_connect: move |_| {
                            connected.set(true);
                        }
                    }
                } else {
                    Dashboard {
                        queues: queues,
                        servers: servers,
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ServerInfo {
    pub server_id: String,
    pub host: String,
    pub pid: i32,
    pub concurrency: i32,
    pub status: String,
    pub active_worker_count: i32,
    pub strict_priority: bool,
}

#[component]
fn Header() -> Element {
    rsx! {
        header { class: "header",
            div { class: "header-content",
                div { class: "header-title",
                    h1 { "🔧 Asynq Members" }
                    div { class: "header-badge", "Task Control Panel" }
                }
                div { style: "font-size: 0.9rem; opacity: 0.9;",
                    "Powered by Rust + Actix-web + Dioxus"
                }
            }
        }
    }
}

#[component]
fn ConnectionDialog(
    redis_url: Signal<String>,
    error_message: Signal<String>,
    on_connect: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "connection-dialog",
            h2 { "🔌 Connect to Redis" }
            if !error_message().is_empty() {
                div { class: "error-message", "{error_message}" }
            }
            div { class: "form-group",
                label { "Redis URL" }
                input {
                    r#type: "text",
                    value: "{redis_url}",
                    placeholder: "redis://127.0.0.1:6379",
                    oninput: move |evt| redis_url.set(evt.value())
                }
            }
            button {
                class: "btn btn-full",
                onclick: move |_| {
                    // TODO: Make API call to connect
                    on_connect.call(());
                },
                "Connect"
            }
        }
    }
}

#[component]
fn Dashboard(
    queues: Signal<Vec<String>>,
    servers: Signal<Vec<ServerInfo>>,
) -> Element {
    rsx! {
        div { id: "dashboard",
            div { class: "toolbar",
                h2 { "📊 Dashboard" }
                button {
                    class: "btn",
                    onclick: move |_| {
                        // TODO: Refresh data
                    },
                    "🔄 Refresh"
                }
            }

            div { class: "section",
                h3 { "Servers" }
                div { class: "grid grid-2",
                    if servers().is_empty() {
                        div { class: "empty-state", "No servers found" }
                    } else {
                        for server in servers() {
                            ServerCard { server: server }
                        }
                    }
                }
            }

            div { class: "section",
                h3 { "Queues" }
                div { class: "grid grid-2",
                    if queues().is_empty() {
                        div { class: "empty-state", "No queues found" }
                    } else {
                        for queue in queues() {
                            QueueCard { queue: queue }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ServerCard(server: ServerInfo) -> Element {
    rsx! {
        div { class: "card",
            div { class: "card-header",
                div { class: "card-title", "🖥️ {server.server_id}" }
                div { class: "card-subtitle", "{server.host}:{server.pid}" }
            }
            div { class: "stats-grid", style: "grid-template-columns: repeat(2, 1fr);",
                div { class: "stat",
                    div { class: "stat-label", "Status" }
                    div { class: "stat-value stat-green", "{server.status}" }
                }
                div { class: "stat",
                    div { class: "stat-label", "Concurrency" }
                    div { class: "stat-value stat-blue", "{server.concurrency}" }
                }
                div { class: "stat",
                    div { class: "stat-label", "Active Workers" }
                    div { class: "stat-value stat-orange", "{server.active_worker_count}" }
                }
                div { class: "stat",
                    div { class: "stat-label", "Strict Priority" }
                    div { class: "stat-value stat-purple",
                        if server.strict_priority { "✓ Yes" } else { "✗ No" }
                    }
                }
            }
        }
    }
}

#[component]
fn QueueCard(queue: String) -> Element {
    rsx! {
        div { class: "card",
            div { class: "card-header",
                div { class: "card-title", "📋 {queue}" }
            }
            div { class: "stats-grid",
                div { class: "stat",
                    div { class: "stat-label", "Pending" }
                    div { class: "stat-value stat-blue", "0" }
                }
                div { class: "stat",
                    div { class: "stat-label", "Active" }
                    div { class: "stat-value stat-green", "0" }
                }
                div { class: "stat",
                    div { class: "stat-label", "Scheduled" }
                    div { class: "stat-value stat-orange", "0" }
                }
            }
            div { class: "card-actions",
                button { class: "btn", "▶️ Resume" }
            }
        }
    }
}

const GLOBAL_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    background-color: #f5f5f5;
    min-height: 100vh;
}

.header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1.5rem 2rem;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.header-content {
    max-width: 1400px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.header-title {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.header h1 {
    font-size: 2rem;
    font-weight: 700;
}

.header-badge {
    background: rgba(255,255,255,0.2);
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.9rem;
}

.container {
    max-width: 1400px;
    margin: 0 auto;
    padding: 2rem;
}

.connection-dialog {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 20px rgba(0,0,0,0.1);
    max-width: 500px;
    margin: 2rem auto;
}

.connection-dialog h2 {
    margin: 0 0 1.5rem 0;
    color: #333;
    font-size: 1.5rem;
}

.form-group {
    margin-bottom: 1.5rem;
}

label {
    display: block;
    margin-bottom: 0.5rem;
    color: #666;
    font-weight: 500;
}

input[type="text"] {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #e0e0e0;
    border-radius: 8px;
    font-size: 1rem;
}

.btn {
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.2s;
}

.btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-full {
    width: 100%;
}

.error-message {
    background: #fee;
    color: #c33;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
}

.toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

.toolbar h2 {
    margin: 0;
    color: #333;
    font-size: 1.5rem;
}

.section {
    margin-bottom: 2rem;
}

.section h3 {
    margin: 0 0 1rem 0;
    color: #333;
    font-size: 1.25rem;
}

.grid {
    display: grid;
    gap: 1rem;
}

.grid-2 {
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
}

.card {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    transition: transform 0.2s, box-shadow 0.2s;
}

.card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 16px rgba(0,0,0,0.15);
}

.card-header {
    margin-bottom: 1rem;
}

.card-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: #333;
    margin-bottom: 0.25rem;
}

.card-subtitle {
    font-size: 0.9rem;
    color: #666;
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    margin-top: 1rem;
}

.stat {
    text-align: center;
}

.stat-label {
    font-size: 0.85rem;
    color: #666;
    margin-bottom: 0.25rem;
}

.stat-value {
    font-size: 1.5rem;
    font-weight: 700;
}

.stat-green { color: #10b981; }
.stat-blue { color: #3b82f6; }
.stat-orange { color: #f59e0b; }
.stat-purple { color: #8b5cf6; }
.stat-red { color: #ef4444; }

.card-actions {
    margin-top: 1rem;
    display: flex;
    gap: 0.5rem;
}

.empty-state {
    text-align: center;
    padding: 2rem;
    color: #999;
    grid-column: 1 / -1;
}
"#;
