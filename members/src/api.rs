//! API handlers for the web interface

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use asynq::base::keys::TaskState;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Deserialize)]
pub struct ConnectRequest {
    pub redis_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskInfoResponse {
    pub id: String,
    pub task_type: String,
    pub queue: String,
    pub max_retry: i32,
    pub retried: i32,
}

impl From<asynq::task::TaskInfo> for TaskInfoResponse {
    fn from(task: asynq::task::TaskInfo) -> Self {
        Self {
            id: task.id,
            task_type: task.task_type,
            queue: task.queue,
            max_retry: task.max_retry,
            retried: task.retried,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerInfoResponse {
    pub server_id: String,
    pub host: String,
    pub pid: i32,
    pub concurrency: i32,
    pub status: String,
    pub active_worker_count: i32,
    pub strict_priority: bool,
}

impl From<asynq::proto::ServerInfo> for ServerInfoResponse {
    fn from(server: asynq::proto::ServerInfo) -> Self {
        Self {
            server_id: server.server_id,
            host: server.host,
            pid: server.pid,
            concurrency: server.concurrency,
            status: server.status,
            active_worker_count: server.active_worker_count,
            strict_priority: server.strict_priority,
        }
    }
}

/// Connect to Redis
pub async fn connect(
    State(state): State<AppState>,
    Json(req): Json<ConnectRequest>,
) -> impl IntoResponse {
    match state.inspector.initialize(req.redis_url).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success("Connected".to_string())),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<String>::error(e)),
        ),
    }
}

/// Get all queues
pub async fn get_queues(State(state): State<AppState>) -> impl IntoResponse {
    match state.inspector.get_queues().await {
        Ok(queues) => (StatusCode::OK, Json(ApiResponse::success(queues))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<String>>::error(e)),
        ),
    }
}

/// Get queue information
pub async fn get_queue_info(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    use asynq::task::QueueInfo;
    
    match state.inspector.get_queue_info(&name).await {
        Ok(info) => (StatusCode::OK, Json(ApiResponse::success(info))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<QueueInfo>::error(e)),
        ),
    }
}

/// Get server information
pub async fn get_servers(State(state): State<AppState>) -> impl IntoResponse {
    match state.inspector.get_servers().await {
        Ok(servers) => {
            let response_servers: Vec<ServerInfoResponse> =
                servers.into_iter().map(|s| s.into()).collect();
            (StatusCode::OK, Json(ApiResponse::success(response_servers)))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<ServerInfoResponse>>::error(e)),
        ),
    }
}

/// Get tasks by queue and state
pub async fn get_tasks(
    State(state): State<AppState>,
    Path((queue, state_str)): Path<(String, String)>,
) -> impl IntoResponse {
    let task_state = match state_str.as_str() {
        "pending" => TaskState::Pending,
        "active" => TaskState::Active,
        "scheduled" => TaskState::Scheduled,
        "retry" => TaskState::Retry,
        "archived" => TaskState::Archived,
        "completed" => TaskState::Completed,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<Vec<TaskInfoResponse>>::error(
                    "Invalid task state".to_string(),
                )),
            )
        }
    };

    match state.inspector.list_tasks(&queue, task_state).await {
        Ok(tasks) => {
            let response_tasks: Vec<TaskInfoResponse> =
                tasks.into_iter().map(|t| t.into()).collect();
            (StatusCode::OK, Json(ApiResponse::success(response_tasks)))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<TaskInfoResponse>>::error(e)),
        ),
    }
}

/// Pause a queue
pub async fn pause_queue(
    State(state): State<AppState>,
    Path(queue): Path<String>,
) -> impl IntoResponse {
    match state.inspector.pause_queue(&queue).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success("Paused".to_string())),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<String>::error(e)),
        ),
    }
}

/// Unpause a queue
pub async fn unpause_queue(
    State(state): State<AppState>,
    Path(queue): Path<String>,
) -> impl IntoResponse {
    match state.inspector.unpause_queue(&queue).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success("Resumed".to_string())),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<String>::error(e)),
        ),
    }
}
