use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, warn, error};

mod monitor;
mod models;
mod dashboard;
mod storage;
mod config;

use monitor::LatencyMonitor;
use dashboard::DashboardServer;
use storage::MetricsStorage;
use config::Config;

#[derive(Parser)]
#[command(
    name = "vscode-latency-monitor",
    version = "1.0.0",
    about = "High-performance VS Code latency monitoring system",
    long_about = "Monitor VS Code command execution, AI model responses, and system performance with microsecond precision."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable debug logging
    #[arg(short, long, global = true)]
    debug: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start latency monitoring
    Start {
        /// Component to monitor (vscode, models, terminal, all)
        #[arg(short, long, default_value = "all")]
        component: String,

        /// Monitoring interval in milliseconds
        #[arg(short, long, default_value = "100")]
        interval: u64,

        /// Run in background
        #[arg(short, long)]
        daemon: bool,
    },

    /// Stop monitoring processes
    Stop {
        /// Force stop all processes
        #[arg(short, long)]
        force: bool,
    },

    /// Start web dashboard
    Dashboard {
        /// Port to serve dashboard
        #[arg(short, long, default_value = "3030")]
        port: u16,

        /// Enable real-time WebSocket updates
        #[arg(short, long)]
        realtime: bool,
    },

    /// Generate performance reports
    Report {
        /// Output format (json, csv, html)
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Time range (e.g., "1h", "24h", "7d")
        #[arg(short, long, default_value = "1h")]
        since: String,
    },

    /// Export metrics data
    Export {
        /// Export format (sqlite, json, csv)
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Time range filter
        #[arg(short, long)]
        since: Option<String>,
    },

    /// Configuration management
    Config {
        /// Configuration action (get, set, list, reset)
        action: String,

        /// Configuration key (for get/set)
        key: Option<String>,

        /// Configuration value (for set)
        value: Option<String>,
    },

    /// Show system status
    Status {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Test monitoring components
    Test {
        /// Component to test
        component: Option<String>,

        /// Number of test iterations
        #[arg(short, long, default_value = "10")]
        iterations: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    init_tracing(cli.debug)?;

    // Load configuration
    let config = Config::load(cli.config)?;

    match cli.command {
        Commands::Start { component, interval, daemon } => {
            start_monitoring(&config, &component, interval, daemon).await?;
        }
        
        Commands::Stop { force } => {
            stop_monitoring(force).await?;
        }
        
        Commands::Dashboard { port, realtime } => {
            start_dashboard(&config, port, realtime).await?;
        }
        
        Commands::Report { format, output, since } => {
            generate_report(&config, &format, output, &since).await?;
        }
        
        Commands::Export { format, output, since } => {
            export_metrics(&config, &format, output, since).await?;
        }
        
        Commands::Config { action, key, value } => {
            handle_config(&action, key, value)?;
        }
        
        Commands::Status { verbose } => {
            show_status(&config, verbose).await?;
        }
        
        Commands::Test { component, iterations } => {
            run_tests(&config, component, iterations).await?;
        }
    }

    Ok(())
}

fn init_tracing(debug: bool) -> Result<()> {
    let level = if debug { 
        tracing::Level::DEBUG 
    } else { 
        tracing::Level::INFO 
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(debug)
        .with_line_number(debug)
        .init();

    info!("VS Code Latency Monitor starting...");
    Ok(())
}

async fn start_monitoring(
    config: &Config, 
    component: &str, 
    interval: u64, 
    daemon: bool
) -> Result<()> {
    info!("Starting latency monitoring for component: {}", component);
    
    let storage = MetricsStorage::new(&config.storage.database_path).await?;
    let mut monitor = LatencyMonitor::new(config.clone(), storage).await?;

    match component {
        "vscode" => {
            monitor.start_vscode_monitoring(interval).await?;
        }
        "models" => {
            monitor.start_model_monitoring(interval).await?;
        }
        "terminal" => {
            monitor.start_terminal_monitoring(interval).await?;
        }
        "all" => {
            monitor.start_all_monitoring(interval).await?;
        }
        _ => {
            error!("Unknown component: {}", component);
            return Err(anyhow::anyhow!("Invalid component specified"));
        }
    }

    if daemon {
        info!("Running in daemon mode...");
        monitor.run_daemon().await?;
    } else {
        info!("Running in foreground mode. Press Ctrl+C to stop.");
        monitor.run_foreground().await?;
    }

    Ok(())
}

async fn stop_monitoring(force: bool) -> Result<()> {
    info!("Stopping latency monitoring processes...");
    
    // Implementation for graceful shutdown
    if force {
        warn!("Force stopping all monitoring processes");
        // Kill all related processes
    } else {
        info!("Gracefully stopping monitoring processes");
        // Send shutdown signals
    }

    Ok(())
}

async fn start_dashboard(config: &Config, port: u16, realtime: bool) -> Result<()> {
    info!("Starting web dashboard on port {}", port);
    
    let storage = MetricsStorage::new(&config.storage.database_path).await?;
    let dashboard = DashboardServer::new(config.clone(), storage, realtime).await?;
    
    dashboard.serve(port).await?;
    
    Ok(())
}

async fn generate_report(
    config: &Config,
    format: &str,
    output: Option<PathBuf>,
    since: &str
) -> Result<()> {
    info!("Generating performance report in {} format", format);
    
    let storage = MetricsStorage::new(&config.storage.database_path).await?;
    let report = storage.generate_report(since, format).await?;
    
    match output {
        Some(path) => {
            std::fs::write(path, report)?;
            info!("Report saved to file");
        }
        None => {
            println!("{}", report);
        }
    }
    
    Ok(())
}

async fn export_metrics(
    config: &Config,
    format: &str,
    output: Option<PathBuf>,
    since: Option<String>
) -> Result<()> {
    info!("Exporting metrics in {} format", format);
    
    let storage = MetricsStorage::new(&config.storage.database_path).await?;
    let data = storage.export_metrics(format, since).await?;
    
    match output {
        Some(path) => {
            std::fs::write(path, data)?;
            info!("Metrics exported to file");
        }
        None => {
            println!("{}", String::from_utf8_lossy(&data));
        }
    }
    
    Ok(())
}

fn handle_config(action: &str, key: Option<String>, value: Option<String>) -> Result<()> {
    match action {
        "get" => {
            if let Some(k) = key {
                // Get configuration value
                info!("Getting config value for key: {}", k);
            } else {
                error!("Key required for get action");
            }
        }
        "set" => {
            if let (Some(k), Some(v)) = (key, value) {
                // Set configuration value
                info!("Setting config {}={}", k, v);
            } else {
                error!("Key and value required for set action");
            }
        }
        "list" => {
            // List all configuration
            info!("Listing all configuration values");
        }
        "reset" => {
            // Reset configuration to defaults
            warn!("Resetting configuration to defaults");
        }
        _ => {
            error!("Unknown config action: {}", action);
        }
    }
    
    Ok(())
}

async fn show_status(config: &Config, verbose: bool) -> Result<()> {
    info!("Showing system status...");
    
    let storage = MetricsStorage::new(&config.storage.database_path).await?;
    let status = storage.get_system_status().await?;
    
    if verbose {
        println!("Detailed System Status:\n{:#?}", status);
    } else {
        println!("System Status: {}", status.summary);
    }
    
    Ok(())
}

async fn run_tests(
    config: &Config,
    component: Option<String>,
    iterations: usize
) -> Result<()> {
    info!("Running tests for {} iterations", iterations);
    
    let storage = MetricsStorage::new(&config.storage.database_path).await?;
    let monitor = LatencyMonitor::new(config.clone(), storage).await?;
    
    match component.as_deref() {
        Some("vscode") => {
            monitor.test_vscode_monitoring(iterations).await?;
        }
        Some("models") => {
            monitor.test_model_monitoring(iterations).await?;
        }
        Some("terminal") => {
            monitor.test_terminal_monitoring(iterations).await?;
        }
        None => {
            monitor.test_all_components(iterations).await?;
        }
        Some(comp) => {
            error!("Unknown test component: {}", comp);
            return Err(anyhow::anyhow!("Invalid test component"));
        }
    }
    
    info!("Tests completed successfully");
    Ok(())
}
