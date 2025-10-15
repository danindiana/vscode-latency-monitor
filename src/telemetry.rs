use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::json;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::config::Config;
use crate::storage::MetricsStorage;

#[derive(Clone)]
pub struct TelemetryServer {
    storage: MetricsStorage,
    config: Config,
    lan_ip: String,
}

#[derive(Clone)]
struct TelemetryState {
    storage: MetricsStorage,
    config: Config,
    lan_ip: String,
}

impl TelemetryServer {
    pub async fn new(config: Config, storage: MetricsStorage) -> Result<Self> {
        let lan_ip = get_lan_ip().await?;
        info!("Detected LAN IP: {}", lan_ip);
        
        Ok(Self {
            config,
            storage,
            lan_ip,
        })
    }

    pub async fn serve(&self, port: u16) -> Result<()> {
        let state = TelemetryState {
            storage: self.storage.clone(),
            config: self.config.clone(),
            lan_ip: self.lan_ip.clone(),
        };

        let app = Router::new()
            .route("/", get(telemetry_home))
            .route("/api/telemetry", get(api_telemetry))
            .route("/api/metrics/raw", get(api_raw_metrics))
            .route("/api/metrics/summary", get(api_metrics_summary))
            .route("/api/system/resources", get(api_system_resources))
            .route("/api/monitoring/status", get(api_monitoring_status))
            .route("/health", get(telemetry_health))
            .layer(CorsLayer::permissive())
            .with_state(state);

        let addr = format!("0.0.0.0:{}", port);
        info!("🌐 Starting telemetry server on LAN: http://{}:{}", self.lan_ip, port);
        info!("📊 Telemetry endpoints available at:");
        info!("  - Main: http://{}:{}/", self.lan_ip, port);
        info!("  - API: http://{}:{}/api/telemetry", self.lan_ip, port);
        info!("  - Raw Metrics: http://{}:{}/api/metrics/raw", self.lan_ip, port);

        let listener = TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn get_lan_ip() -> Result<String> {
    // Try to get the LAN IP using a simple approach
    use std::process::Command;
    
    let output = Command::new("ip")
        .args(&["route", "get", "8.8.8.8"])
        .output()?;
    
    let output_str = String::from_utf8(output.stdout)?;
    for line in output_str.lines() {
        if let Some(src_pos) = line.find("src ") {
            let ip_start = src_pos + 4;
            if let Some(ip_end) = line[ip_start..].find(' ') {
                return Ok(line[ip_start..ip_start + ip_end].to_string());
            }
        }
    }
    
    // Fallback to localhost if we can't detect LAN IP
    Ok("127.0.0.1".to_string())
}

async fn telemetry_home(State(state): State<TelemetryState>) -> axum::response::Html<String> {
    let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>VS Code Latency Monitor - Telemetry Service</title>
    <meta http-equiv="refresh" content="10">
    <style>
        body {{
            font-family: 'Monaco', 'Consolas', monospace;
            background: linear-gradient(135deg, #0f0f23 0%, #1a1a3e 100%);
            color: #00ff88;
            margin: 0;
            padding: 20px;
            line-height: 1.6;
        }}
        .container {{
            max-width: 1400px;
            margin: 0 auto;
            background: rgba(0,0,0,0.85);
            padding: 30px;
            border-radius: 15px;
            box-shadow: 0 0 30px rgba(0,255,136,0.3);
            border: 1px solid #00ff88;
        }}
        h1 {{
            text-align: center;
            color: #00ffff;
            text-shadow: 0 0 15px #00ffff;
            border-bottom: 3px solid #00ff88;
            padding-bottom: 15px;
            margin-bottom: 30px;
        }}
        .grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 25px;
            margin: 25px 0;
        }}
        .panel {{
            background: rgba(0,20,40,0.7);
            padding: 20px;
            border: 2px solid #00ff88;
            border-radius: 12px;
            box-shadow: 0 0 15px rgba(0,255,136,0.2);
        }}
        .panel h3 {{
            color: #ffff00;
            margin-top: 0;
            text-shadow: 0 0 8px #ffff00;
            border-bottom: 1px solid #ffff00;
            padding-bottom: 8px;
        }}
        .endpoint {{
            background: rgba(0,0,0,0.6);
            padding: 10px;
            margin: 8px 0;
            border-left: 4px solid #00ff88;
            border-radius: 6px;
            font-family: monospace;
        }}
        .endpoint a {{
            color: #00ffff;
            text-decoration: none;
        }}
        .endpoint a:hover {{
            color: #ffff00;
            text-shadow: 0 0 5px #ffff00;
        }}
        .status {{
            display: inline-block;
            padding: 4px 12px;
            border-radius: 6px;
            font-weight: bold;
            font-size: 0.9em;
        }}
        .active {{ background: #00ff88; color: #000; }}
        .monitoring {{ background: #ffff00; color: #000; }}
        .info {{
            background: rgba(0,100,200,0.3);
            padding: 15px;
            border-radius: 8px;
            border-left: 5px solid #0088ff;
            margin: 20px 0;
        }}
        .timestamp {{
            color: #888;
            font-size: 0.9em;
            text-align: center;
            margin-top: 20px;
        }}
        .lan-info {{
            background: rgba(0,255,136,0.1);
            padding: 15px;
            border-radius: 8px;
            border: 1px dashed #00ff88;
            margin: 15px 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🛰️ VS Code Latency Monitor - Telemetry Service</h1>
        
        <div class="lan-info">
            <strong>🌐 LAN Access:</strong> This service is accessible across your network at <code>{}</code>
        </div>

        <div class="grid">
            <div class="panel">
                <h3>📡 Telemetry Endpoints</h3>
                <div class="endpoint">
                    <a href="/api/telemetry">📊 /api/telemetry</a> - Complete telemetry data
                </div>
                <div class="endpoint">
                    <a href="/api/metrics/raw">📈 /api/metrics/raw</a> - Raw performance metrics
                </div>
                <div class="endpoint">
                    <a href="/api/metrics/summary">📋 /api/metrics/summary</a> - Summarized metrics
                </div>
                <div class="endpoint">
                    <a href="/api/system/resources">💻 /api/system/resources</a> - System resources
                </div>
                <div class="endpoint">
                    <a href="/api/monitoring/status">⚡ /api/monitoring/status</a> - Monitor status
                </div>
            </div>

            <div class="panel">
                <h3>🔧 Service Status</h3>
                <p>Telemetry Server: <span class="status active">ACTIVE</span></p>
                <p>LAN Broadcasting: <span class="status active">ENABLED</span></p>
                <p>Data Collection: <span class="status monitoring">MONITORING</span></p>
                <p>API Endpoints: <span class="status active">6 AVAILABLE</span></p>
            </div>

            <div class="panel">
                <h3>🌐 Network Configuration</h3>
                <p><strong>Local IP:</strong> {}</p>
                <p><strong>Telemetry Port:</strong> 8080</p>
                <p><strong>Dashboard Port:</strong> 3030</p>
                <p><strong>CORS:</strong> Permissive (LAN access)</p>
            </div>

            <div class="panel">
                <h3>📊 Data Integration</h3>
                <p>✅ VS Code Latency Monitoring</p>
                <p>✅ System Resource Tracking</p>
                <p>✅ Performance Metrics Collection</p>
                <p>✅ Real-time Telemetry Streaming</p>
                <p>✅ Cross-platform Compatibility</p>
            </div>

            <div class="panel">
                <h3>🔗 Related Services</h3>
                <div class="endpoint">
                    <a href="http://{}:3030">🎛️ Main Dashboard (Port 3030)</a>
                </div>
                <div class="endpoint">
                    <a href="http://{}:8888">🏠 Wall Notifications (Port 8888)</a>
                </div>
                <div class="info">
                    <strong>Integration:</strong> This telemetry service provides machine-readable data for external monitoring systems, dashboards, and automation tools.
                </div>
            </div>

            <div class="panel">
                <h3>⚡ Quick Stats</h3>
                <p id="live-events">Loading events...</p>
                <p id="live-metrics">Loading metrics...</p>
                <p id="live-status">Loading status...</p>
            </div>
        </div>

        <div class="timestamp">
            Last Updated: {} | Auto-refresh every 10 seconds
        </div>
    </div>

    <script>
        // Auto-refresh live stats
        async function updateStats() {{
            try {{
                const response = await fetch('/api/telemetry');
                const data = await response.json();
                
                document.getElementById('live-events').textContent = 
                    `Recent Events: ${{data.recent_events?.length || 0}}`;
                document.getElementById('live-metrics').textContent = 
                    `Active Monitors: ${{data.system_status?.active_monitors?.length || 0}}`;
                document.getElementById('live-status').textContent = 
                    `System Status: ${{data.system_status?.summary || 'Unknown'}}`;
            }} catch (e) {{
                console.log('Stats update failed:', e);
            }}
        }}
        
        // Update stats every 5 seconds
        setInterval(updateStats, 5000);
        updateStats(); // Initial load
    </script>
</body>
</html>
"#, 
        state.lan_ip,    // LAN Access code
        state.lan_ip,    // Local IP
        state.lan_ip,    // Dashboard link  
        state.lan_ip,    // Wall notifications link
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")  // Timestamp
    );

    axum::response::Html(html)
}

async fn api_telemetry(State(state): State<TelemetryState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let system_status = state.storage.get_system_status().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let recent_events = state.storage.get_recent_events(100).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let performance_metrics = state.storage.get_performance_metrics().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "service": "vscode-latency-monitor-telemetry",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now(),
        "lan_ip": state.lan_ip,
        "system_status": system_status,
        "recent_events": recent_events,
        "performance_metrics": performance_metrics,
        "telemetry_metadata": {
            "collection_interval": "real-time",
            "data_retention": "persistent",
            "api_version": "v1",
            "cors_enabled": true
        }
    })))
}

async fn api_raw_metrics(State(state): State<TelemetryState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let events = state.storage.get_recent_events(1000).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "raw_metrics": events,
        "total_count": events.len(),
        "timestamp": chrono::Utc::now(),
        "source": state.lan_ip
    })))
}

async fn api_metrics_summary(State(state): State<TelemetryState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let metrics = state.storage.get_performance_metrics().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let events = state.storage.get_recent_events(100).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut component_stats = HashMap::new();
    for event in &events {
        let counter = component_stats.entry(event.component_type.to_string()).or_insert(0);
        *counter += 1;
    }

    Ok(Json(json!({
        "summary": {
            "total_metrics": metrics.len(),
            "recent_events": events.len(),
            "component_breakdown": component_stats,
            "collection_time": chrono::Utc::now()
        },
        "performance_metrics": metrics,
        "source_ip": state.lan_ip
    })))
}

async fn api_system_resources(State(_state): State<TelemetryState>) -> Result<Json<serde_json::Value>, StatusCode> {
    use sysinfo::System;
    
    let mut sys = System::new_all();
    sys.refresh_all();

    let load = System::load_average();

    Ok(Json(json!({
        "system_resources": {
            "memory": {
                "total": sys.total_memory(),
                "used": sys.used_memory(),
                "available": sys.available_memory()
            },
            "cpu": {
                "cpu_count": sys.cpus().len()
            },
            "load_average": {
                "one_minute": load.one,
                "five_minutes": load.five,
                "fifteen_minutes": load.fifteen
            },
            "processes": sys.processes().len(),
            "uptime": System::uptime()
        },
        "timestamp": chrono::Utc::now()
    })))
}

async fn api_monitoring_status(State(state): State<TelemetryState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let status = state.storage.get_system_status().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "monitoring_status": status,
        "telemetry_info": {
            "lan_accessible": true,
            "lan_ip": state.lan_ip,
            "service_port": 8080,
            "dashboard_port": 3030
        },
        "timestamp": chrono::Utc::now()
    })))
}

async fn telemetry_health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "telemetry",
        "timestamp": chrono::Utc::now()
    }))
}