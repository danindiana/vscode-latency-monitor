use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub monitoring: MonitoringConfig,
    pub dashboard: DashboardConfig,
    pub storage: StorageConfig,
    pub integrations: IntegrationsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub interval_ms: u64,
    pub precision: String,
    pub buffer_size: usize,
    pub enabled_components: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub port: u16,
    pub auto_refresh_ms: u64,
    pub theme: String,
    pub enable_websocket: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database_path: PathBuf,
    pub retention_days: u32,
    pub archive_threshold: u64,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsConfig {
    pub wall_notification_system: bool,
    pub enhanced_logging: bool,
    pub copilot_tracking: bool,
    pub export_prometheus: bool,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        
        Self {
            monitoring: MonitoringConfig {
                interval_ms: 100,
                precision: "microsecond".to_string(),
                buffer_size: 10000,
                enabled_components: vec![
                    "vscode".to_string(),
                    "models".to_string(),
                    "terminal".to_string(),
                ],
            },
            dashboard: DashboardConfig {
                port: 3030,
                auto_refresh_ms: 1000,
                theme: "dark".to_string(),
                enable_websocket: true,
            },
            storage: StorageConfig {
                database_path: home_dir
                    .join(".local/share/vscode-latency-monitor/metrics.db"),
                retention_days: 30,
                archive_threshold: 100000,
                compression_enabled: true,
            },
            integrations: IntegrationsConfig {
                wall_notification_system: true,
                enhanced_logging: true,
                copilot_tracking: true,
                export_prometheus: false,
            },
        }
    }
}

impl Config {
    pub fn load(config_path: Option<PathBuf>) -> Result<Self> {
        let config_file = match config_path {
            Some(path) => path,
            None => {
                let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
                home_dir.join(".config/vscode-latency-monitor/config.toml")
            }
        };

        if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config file
            let config = Config::default();
            config.save(&config_file)?;
            Ok(config)
        }
    }

    pub fn save(&self, config_path: &Path) -> Result<()> {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        // Validate configuration values
        if self.monitoring.interval_ms == 0 {
            return Err(anyhow::anyhow!("Monitoring interval must be greater than 0"));
        }

        if self.monitoring.buffer_size == 0 {
            return Err(anyhow::anyhow!("Buffer size must be greater than 0"));
        }

        if self.dashboard.port < 1024 {
            return Err(anyhow::anyhow!("Dashboard port should be >= 1024"));
        }

        if self.storage.retention_days == 0 {
            return Err(anyhow::anyhow!("Retention days must be greater than 0"));
        }

        Ok(())
    }
}