//! API handlers for the web interface

use actix_web::{web, HttpResponse, Result};
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
    state: web::Data<AppState>,
    req: web::Json<ConnectRequest>,
) -> Result<HttpResponse> {
    match state.inspector.initialize(req.redis_url.clone()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("Connected".to_string()))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(e))),
    }
}

/// Get all queues
pub async fn get_queues(state: web::Data<AppState>) -> Result<HttpResponse> {
    match state.inspector.get_queues().await {
        Ok(queues) => Ok(HttpResponse::Ok().json(ApiResponse::success(queues))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<String>>::error(e))),
    }
}

/// Get queue information
pub async fn get_queue_info(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    use asynq::task::QueueInfo;
    
    match state.inspector.get_queue_info(&path).await {
        Ok(info) => Ok(HttpResponse::Ok().json(ApiResponse::success(info))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<QueueInfo>::error(e))),
    }
}

/// Get server information
pub async fn get_servers(state: web::Data<AppState>) -> Result<HttpResponse> {
    match state.inspector.get_servers().await {
        Ok(servers) => {
            let response_servers: Vec<ServerInfoResponse> =
                servers.into_iter().map(|s| s.into()).collect();
            Ok(HttpResponse::Ok().json(ApiResponse::success(response_servers)))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<ServerInfoResponse>>::error(e))),
    }
}

/// Get tasks by queue and state
pub async fn get_tasks(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let (queue, state_str) = path.into_inner();
    
    let task_state = match state_str.as_str() {
        "pending" => TaskState::Pending,
        "active" => TaskState::Active,
        "scheduled" => TaskState::Scheduled,
        "retry" => TaskState::Retry,
        "archived" => TaskState::Archived,
        "completed" => TaskState::Completed,
        _ => {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<Vec<TaskInfoResponse>>::error(
                "Invalid task state".to_string(),
            )))
        }
    };

    match state.inspector.list_tasks(&queue, task_state).await {
        Ok(tasks) => {
            let response_tasks: Vec<TaskInfoResponse> =
                tasks.into_iter().map(|t| t.into()).collect();
            Ok(HttpResponse::Ok().json(ApiResponse::success(response_tasks)))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<TaskInfoResponse>>::error(e))),
    }
}

/// Pause a queue
pub async fn pause_queue(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    match state.inspector.pause_queue(&path).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("Paused".to_string()))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(e))),
    }
}

/// Unpause a queue
pub async fn unpause_queue(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    match state.inspector.unpause_queue(&path).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("Resumed".to_string()))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(e))),
    }
}
