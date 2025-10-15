use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::get,
    Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

use crate::config::Config;
use crate::storage::MetricsStorage;

pub struct DashboardServer {
    config: Config,
    storage: MetricsStorage,
    realtime_enabled: bool,
}

#[derive(Clone)]
struct AppState {
    storage: MetricsStorage,
    config: Config,
}

impl DashboardServer {
    pub async fn new(
        config: Config,
        storage: MetricsStorage,
        realtime_enabled: bool,
    ) -> Result<Self> {
        Ok(Self {
            config,
            storage,
            realtime_enabled,
        })
    }

    pub async fn serve(self, port: u16) -> Result<()> {
        let state = AppState {
            storage: self.storage,
            config: self.config,
        };

        let app = Router::new()
            .route("/", get(dashboard_html))
            .route("/api/status", get(api_status))
            .route("/api/events", get(api_events))
            .route("/api/metrics", get(api_metrics))
            .route("/health", get(health_check))
            .layer(CorsLayer::permissive())
            .with_state(state);

        let addr = format!("0.0.0.0:{}", port);
        info!("Starting dashboard server on http://{}", addr);

        let listener = TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn dashboard_html() -> Html<&'static str> {
    Html(include_str!("../static/dashboard.html"))
}

async fn api_status(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.storage.get_system_status().await {
        Ok(status) => Ok(Json(json!(status))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn api_events(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.storage.get_recent_events(50).await {
        Ok(events) => Ok(Json(json!(events))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn api_metrics(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.storage.get_performance_metrics().await {
        Ok(metrics) => Ok(Json(json!(metrics))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0"
    }))
}