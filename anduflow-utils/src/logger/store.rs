use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use rusqlite::Connection;
use crate::error::ExtractorResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogStatus {
    Started,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStore {
    // Core identifiers
    id: Uuid,
    parent_id: Option<Uuid>, // For hierarchical operations

    // Operation metadata
    operation: String,
    operation_type: String, // e.g., "extract", "transform", "load"

    // Status tracking
    status: LogStatus,
    error_message: Option<String>,

    // Timing metrics
    created_at: DateTime<Utc>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    elapsed_ms: Option<usize>, // Calculated from started_at/completed_at

    // Progress tracking
    total_items: Option<usize>,
    processed_items: Option<usize>,
    progress_percentage: Option<f64>,

    // Performance metrics
    items_per_second: Option<f64>,
    memory_usage_mb: Option<f64>,

    // Source/destination info
    source_uri: Option<String>,
    destination_uri: Option<String>,

    // Additional context
    metadata: serde_json::Value, // Flexible JSON for custom fields
    tags: Vec<String>,           // For categorization/filtering

    // System info
    hostname: Option<String>,
    process_id: Option<u32>,
}

impl LogStore {
    pub fn new(operation: String, operation_type: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent_id: None,
            operation,
            operation_type,
            status: LogStatus::Started,
            error_message: None,
            created_at: Utc::now(),
            started_at: Some(Utc::now()),
            completed_at: None,
            elapsed_ms: None,
            total_items: None,
            processed_items: None,
            progress_percentage: None,
            items_per_second: None,
            memory_usage_mb: None,
            source_uri: None,
            destination_uri: None,
            metadata: serde_json::Value::Null,
            tags: Vec::new(),
            hostname: Some(std::env::var("HOSTNAME").unwrap_or_default()),
            process_id: Some(std::process::id()),
        }
    }

    pub fn mark_in_progress(&mut self) {
        self.status = LogStatus::InProgress;
        self.started_at = Some(Utc::now());
    }

    pub fn mark_completed(&mut self) {
        self.status = LogStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.calculate_elapsed_time();
    }

    pub fn mark_failed(&mut self, error: String) {
        self.status = LogStatus::Failed;
        self.error_message = Some(error);
        self.completed_at = Some(Utc::now());
        self.calculate_elapsed_time();
    }

    pub fn update_progress(&mut self, processed: usize, total: usize) {
        self.processed_items = Some(processed);
        self.total_items = Some(total);
        self.progress_percentage = Some((processed as f64 / total.max(1) as f64) * 100.0);

        // Calculate items per second if we have timing data
        if let Some(started) = self.started_at {
            let elapsed = Utc::now().signed_duration_since(started);
            let seconds = elapsed.num_seconds().max(1) as f64;
            self.items_per_second = Some(processed as f64 / seconds);
        }
    }

    fn calculate_elapsed_time(&mut self) {
        if let (Some(started), Some(completed)) = (self.started_at, self.completed_at) {
            let elapsed = completed.signed_duration_since(started);
            self.elapsed_ms = Some(elapsed.num_milliseconds() as usize);
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn set_metadata(&mut self, metadata: serde_json::Value) {
        self.metadata = metadata;
    }

    pub fn set_source_destination(&mut self, source: Option<String>, destination: Option<String>) {
        self.source_uri = source;
        self.destination_uri = destination;
    }
}

// SQLite schema creation
pub fn create_table_sql() -> String {
    r#"
    CREATE TABLE IF NOT EXISTS etl_logs (
        id TEXT PRIMARY KEY,
        parent_id TEXT,
        operation TEXT NOT NULL,
        operation_type TEXT NOT NULL,
        status TEXT NOT NULL,
        error_message TEXT,
        created_at TEXT NOT NULL,
        started_at TEXT,
        completed_at TEXT,
        elapsed_ms INTEGER,
        total_items INTEGER,
        processed_items INTEGER,
        progress_percentage REAL,
        items_per_second REAL,
        memory_usage_mb REAL,
        source_uri TEXT,
        destination_uri TEXT,
        metadata TEXT,
        tags TEXT,
        hostname TEXT,
        process_id INTEGER
    )
    "#.to_string()
}


/// Check if a table exists in the SQLite database and create it if it doesn't.
/// 
/// # Arguments
/// 
/// * `conn` - A mutable reference to the SQLite connection wrapped in RwLock
/// * `table_name` - The name of the table to check/create
/// * `create_table_sql` - The SQL statement to create the table
/// 
/// # Returns
/// 
/// Returns `Ok(true)` if the table was created, `Ok(false)` if it already existed,
/// or an error if the operation failed.
pub fn ensure_table_exists(
    conn: &Arc<Connection>,
    table_name: &str,
    create_table_sql: &str,
) -> ExtractorResult<bool> {
    
    // Check if table exists
    let table_exists: bool = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name=?1",
        [table_name],
        |row| row.get::<_, String>(0),
    ).map(|name| name == table_name).unwrap_or(false);
    
    if !table_exists {
        // Create the table
        conn.execute(create_table_sql, [])?;
        Ok(true)
    } else {
        Ok(false)
    }
}


/// Convenience function specifically for the etl_logs table
pub fn ensure_etl_logs_table_exists(conn: &Arc<Connection>) -> ExtractorResult<bool> {
    ensure_table_exists(
        conn,
        "etl_logs",
        r#"
        CREATE TABLE etl_logs (
            id TEXT PRIMARY KEY,
            parent_id TEXT,
            operation TEXT NOT NULL,
            operation_type TEXT NOT NULL,
            status TEXT NOT NULL,
            error_message TEXT,
            created_at TEXT NOT NULL,
            started_at TEXT,
            completed_at TEXT,
            elapsed_ms INTEGER,
            total_items INTEGER,
            processed_items INTEGER,
            progress_percentage REAL,
            items_per_second REAL,
            memory_usage_mb REAL,
            source_uri TEXT,
            destination_uri TEXT,
            metadata TEXT,
            tags TEXT,
            hostname TEXT,
            process_id INTEGER
        )
        "#
    )
}
