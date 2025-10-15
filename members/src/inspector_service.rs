//! Inspector service for interacting with Asynq Inspector API

use asynq::base::keys::TaskState;
use asynq::inspector::Inspector;
use asynq::proto::ServerInfo;
use asynq::rdb::inspect::Pagination;
use asynq::redis::RedisConnectionConfig;
use asynq::task::{DailyStats, QueueInfo, TaskInfo};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Inspector service wrapper
pub struct InspectorService {
    inspector: Arc<RwLock<Option<Inspector>>>,
    redis_url: Arc<RwLock<String>>,
}

impl InspectorService {
    /// Create a new inspector service
    pub fn new() -> Self {
        Self {
            inspector: Arc::new(RwLock::new(None)),
            redis_url: Arc::new(RwLock::new("redis://127.0.0.1:6379".to_string())),
        }
    }

    /// Initialize the inspector with Redis connection
    pub async fn initialize(&self, redis_url: String) -> Result<(), String> {
        tracing::info!("Initializing Inspector with Redis URL: {}", redis_url);
        
        let redis_config = RedisConnectionConfig::single(redis_url.clone())
            .map_err(|e| format!("Failed to create Redis config: {}", e))?;
        
        let inspector = Inspector::new(redis_config)
            .await
            .map_err(|e| format!("Failed to create Inspector: {}", e))?;
        
        *self.inspector.write().await = Some(inspector);
        *self.redis_url.write().await = redis_url;
        
        tracing::info!("Inspector initialized successfully");
        Ok(())
    }

    /// Get Redis URL
    pub async fn get_redis_url(&self) -> String {
        self.redis_url.read().await.clone()
    }

    /// Check if inspector is initialized
    pub async fn is_initialized(&self) -> bool {
        self.inspector.read().await.is_some()
    }

    /// Get all queues
    pub async fn get_queues(&self) -> Result<Vec<String>, String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .get_queues()
            .await
            .map_err(|e| format!("Failed to get queues: {}", e))
    }

    /// Get queue information
    pub async fn get_queue_info(&self, queue: &str) -> Result<QueueInfo, String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .get_queue_info(queue)
            .await
            .map_err(|e| format!("Failed to get queue info: {}", e))
    }

    /// Get server information
    pub async fn get_servers(&self) -> Result<Vec<ServerInfo>, String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .get_servers()
            .await
            .map_err(|e| format!("Failed to get servers: {}", e))
    }

    /// List tasks by state
    pub async fn list_tasks(
        &self,
        queue: &str,
        state: TaskState,
    ) -> Result<Vec<TaskInfo>, String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .list_tasks(queue, state, Pagination::default())
            .await
            .map_err(|e| format!("Failed to list tasks: {}", e))
    }

    /// Get history for a queue
    pub async fn get_history(&self, queue: &str, days: i32) -> Result<Vec<DailyStats>, String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .get_history(queue, days)
            .await
            .map_err(|e| format!("Failed to get history: {}", e))
    }

    /// Pause a queue
    pub async fn pause_queue(&self, queue: &str) -> Result<(), String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .pause_queue(queue)
            .await
            .map_err(|e| format!("Failed to pause queue: {}", e))
    }

    /// Resume a queue
    pub async fn unpause_queue(&self, queue: &str) -> Result<(), String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .unpause_queue(queue)
            .await
            .map_err(|e| format!("Failed to resume queue: {}", e))
    }

    /// Delete a task
    pub async fn delete_task(&self, queue: &str, task_id: &str) -> Result<(), String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .delete_task(queue, task_id)
            .await
            .map_err(|e| format!("Failed to delete task: {}", e))
    }

    /// Archive all pending tasks in a queue
    pub async fn archive_all_pending_tasks(&self, queue: &str) -> Result<i64, String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .archive_all_pending_tasks(queue)
            .await
            .map_err(|e| format!("Failed to archive pending tasks: {}", e))
    }

    /// Run all archived tasks in a queue
    pub async fn run_all_archived_tasks(&self, queue: &str) -> Result<i64, String> {
        let inspector = self.inspector.read().await;
        let inspector = inspector
            .as_ref()
            .ok_or_else(|| "Inspector not initialized".to_string())?;
        
        inspector
            .run_all_archived_tasks(queue)
            .await
            .map_err(|e| format!("Failed to run archived tasks: {}", e))
    }
}

impl Default for InspectorService {
    fn default() -> Self {
        Self::new()
    }
}
