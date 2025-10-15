use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePool, Row};
use std::path::Path;
use tracing::{debug, info, warn};

use crate::models::{LatencyEvent, SystemStatus, PerformanceMetrics, ComponentType};

#[derive(Clone)]
pub struct MetricsStorage {
    pool: SqlitePool,
}

impl MetricsStorage {
    pub async fn new(database_path: &Path) -> Result<Self> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = database_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let database_url = format!("sqlite:{}", database_path.display());
        let pool = SqlitePool::connect(&database_url).await?;

        let storage = Self { pool };
        storage.initialize_schema().await?;

        info!("Metrics storage initialized at: {}", database_path.display());
        Ok(storage)
    }

    async fn initialize_schema(&self) -> Result<()> {
        // Create tables for latency events
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS latency_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                component_type TEXT NOT NULL,
                event_source TEXT NOT NULL,
                duration_us INTEGER NOT NULL,
                description TEXT NOT NULL,
                metadata TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create index for performance
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_latency_events_timestamp 
            ON latency_events(timestamp)
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_latency_events_component 
            ON latency_events(component_type)
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create performance metrics table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS performance_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                component TEXT NOT NULL,
                total_events INTEGER NOT NULL,
                avg_duration_ms REAL NOT NULL,
                min_duration_ms INTEGER NOT NULL,
                max_duration_ms INTEGER NOT NULL,
                p50_duration_ms INTEGER NOT NULL,
                p95_duration_ms INTEGER NOT NULL,
                p99_duration_ms INTEGER NOT NULL,
                events_per_second REAL NOT NULL,
                error_rate REAL NOT NULL,
                last_updated TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn store_event(&self, event: &LatencyEvent) -> Result<()> {
        let metadata_json = serde_json::to_string(&event.metadata)?;
        
        sqlx::query(
            r#"
            INSERT INTO latency_events 
            (timestamp, component_type, event_source, duration_us, description, metadata)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(event.timestamp.to_rfc3339())
        .bind(format!("{:?}", event.component_type))
        .bind(format!("{:?}", event.event_source))
        .bind(event.duration_us())
        .bind(&event.description)
        .bind(metadata_json)
        .execute(&self.pool)
        .await?;

        debug!("Stored latency event: {:?}", event.component_type);
        Ok(())
    }

    pub async fn get_recent_events(&self, limit: u32) -> Result<Vec<LatencyEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT id, timestamp, component_type, event_source, duration_us, description, metadata
            FROM latency_events 
            ORDER BY timestamp DESC 
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut events = Vec::new();
        for row in rows {
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)?
                .with_timezone(&Utc);
            
            let duration_us: i64 = row.get("duration_us");
            let duration = std::time::Duration::from_micros(duration_us as u64);
            
            let component_type_str: String = row.get("component_type");
            let event_source_str: String = row.get("event_source");
            let metadata_str: String = row.get("metadata");
            
            // Parse component type (simplified)
            let component_type = match component_type_str.as_str() {
                "VSCode" => ComponentType::VSCode,
                "VSCodeExtension" => ComponentType::VSCodeExtension,
                "GitHubCopilot" => ComponentType::GitHubCopilot,
                "LocalModel" => ComponentType::LocalModel,
                "Terminal" => ComponentType::Terminal,
                "FileSystem" => ComponentType::FileSystem,
                "Network" => ComponentType::Network,
                _ => ComponentType::System,
            };

            // Parse event source (simplified)
            let event_source = crate::models::EventSource::ProcessMonitor; // Default

            let metadata: serde_json::Value = serde_json::from_str(&metadata_str)
                .unwrap_or(serde_json::Value::Null);

            let event = LatencyEvent {
                id: Some(row.get("id")),
                timestamp,
                component_type,
                event_source,
                duration,
                description: row.get("description"),
                metadata,
            };

            events.push(event);
        }

        Ok(events)
    }

    pub async fn get_performance_metrics(&self) -> Result<Vec<PerformanceMetrics>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                component_type,
                COUNT(*) as total_events,
                AVG(duration_us) / 1000.0 as avg_duration_ms,
                MIN(duration_us) / 1000 as min_duration_ms,
                MAX(duration_us) / 1000 as max_duration_ms
            FROM latency_events 
            WHERE timestamp > datetime('now', '-1 hour')
            GROUP BY component_type
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut metrics = Vec::new();
        for row in rows {
            let component_type_str: String = row.get("component_type");
            let component_type = match component_type_str.as_str() {
                "VSCode" => ComponentType::VSCode,
                "VSCodeExtension" => ComponentType::VSCodeExtension,
                "GitHubCopilot" => ComponentType::GitHubCopilot,
                "LocalModel" => ComponentType::LocalModel,
                "Terminal" => ComponentType::Terminal,
                "FileSystem" => ComponentType::FileSystem,
                "Network" => ComponentType::Network,
                _ => ComponentType::System,
            };

            let metric = PerformanceMetrics {
                component: component_type,
                total_events: row.get::<i64, _>("total_events") as u64,
                avg_duration_ms: row.get("avg_duration_ms"),
                min_duration_ms: row.get::<i64, _>("min_duration_ms") as u64,
                max_duration_ms: row.get::<i64, _>("max_duration_ms") as u64,
                p50_duration_ms: 0, // TODO: Calculate percentiles
                p95_duration_ms: 0,
                p99_duration_ms: 0,
                events_per_second: 0.0, // TODO: Calculate
                error_rate: 0.0,
                last_updated: Utc::now(),
            };

            metrics.push(metric);
        }

        Ok(metrics)
    }

    pub async fn get_system_status(&self) -> Result<SystemStatus> {
        let total_events: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM latency_events"
        )
        .fetch_one(&self.pool)
        .await?;

        let last_event_row = sqlx::query(
            "SELECT timestamp FROM latency_events ORDER BY timestamp DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        let last_event_timestamp = if let Some(row) = last_event_row {
            let timestamp_str: String = row.get("timestamp");
            Some(DateTime::parse_from_rfc3339(&timestamp_str)?.with_timezone(&Utc))
        } else {
            None
        };

        let performance_metrics = self.get_performance_metrics().await?;

        let status = SystemStatus {
            summary: "System operational".to_string(),
            total_events: total_events as u64,
            active_monitors: vec![
                "VS Code Monitor".to_string(),
                "Model Monitor".to_string(),
                "Terminal Monitor".to_string(),
            ],
            performance_metrics,
            last_event_timestamp,
            uptime_seconds: 0, // TODO: Track uptime
            memory_usage_mb: 0, // TODO: Get actual memory usage
            cpu_usage_percent: 0.0, // TODO: Get actual CPU usage
        };

        Ok(status)
    }

    pub async fn generate_report(&self, _since: &str, format: &str) -> Result<String> {
        match format {
            "json" => {
                let events = self.get_recent_events(100).await?;
                let json = serde_json::to_string_pretty(&events)?;
                Ok(json)
            }
            "csv" => {
                let events = self.get_recent_events(100).await?;
                let mut csv = String::from("timestamp,component,duration_ms,description\n");
                
                for event in events {
                    csv.push_str(&format!(
                        "{},{},{},{}\n",
                        event.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        event.component_type,
                        event.duration_ms(),
                        event.description.replace(',', ";")
                    ));
                }
                
                Ok(csv)
            }
            _ => Err(anyhow::anyhow!("Unsupported format: {}", format)),
        }
    }

    pub async fn export_metrics(&self, format: &str, _since: Option<String>) -> Result<Vec<u8>> {
        match format {
            "json" => {
                let events = self.get_recent_events(1000).await?;
                let json = serde_json::to_string(&events)?;
                Ok(json.into_bytes())
            }
            "sqlite" => {
                // For SQLite export, we could copy the database file
                // For now, return a simple message
                Ok("SQLite export not yet implemented".into())
            }
            _ => Err(anyhow::anyhow!("Unsupported export format: {}", format)),
        }
    }

    pub async fn cleanup_old_events(&self, retention_days: u32) -> Result<()> {
        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days as i64);
        
        let deleted = sqlx::query(
            "DELETE FROM latency_events WHERE timestamp < ?"
        )
        .bind(cutoff_date.to_rfc3339())
        .execute(&self.pool)
        .await?;

        info!("Cleaned up {} old events", deleted.rows_affected());
        Ok(())
    }
}