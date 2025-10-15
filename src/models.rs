use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyEvent {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub component_type: ComponentType,
    pub event_source: EventSource,
    pub duration: Duration,
    pub description: String,
    pub metadata: serde_json::Value,
}

impl LatencyEvent {
    pub fn new(
        component_type: ComponentType,
        event_source: EventSource,
        duration: Duration,
        description: String,
    ) -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            component_type,
            event_source,
            duration,
            description,
            metadata: serde_json::Value::Null,
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn duration_ms(&self) -> u64 {
        self.duration.as_millis() as u64
    }

    pub fn duration_us(&self) -> u64 {
        self.duration.as_micros() as u64
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComponentType {
    VSCode,
    VSCodeExtension,
    GitHubCopilot,
    LocalModel,
    Terminal,
    FileSystem,
    Network,
    System,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentType::VSCode => write!(f, "VS Code"),
            ComponentType::VSCodeExtension => write!(f, "VS Code Extension"),
            ComponentType::GitHubCopilot => write!(f, "GitHub Copilot"),
            ComponentType::LocalModel => write!(f, "Local Model"),
            ComponentType::Terminal => write!(f, "Terminal"),
            ComponentType::FileSystem => write!(f, "File System"),
            ComponentType::Network => write!(f, "Network"),
            ComponentType::System => write!(f, "System"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventSource {
    ProcessMonitor,
    ExtensionHost,
    ModelProcess,
    CommandExecution,
    FileOperation,
    NetworkRequest,
    TestCommand,
    UserInteraction,
}

impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventSource::ProcessMonitor => write!(f, "Process Monitor"),
            EventSource::ExtensionHost => write!(f, "Extension Host"),
            EventSource::ModelProcess => write!(f, "Model Process"),
            EventSource::CommandExecution => write!(f, "Command Execution"),
            EventSource::FileOperation => write!(f, "File Operation"),
            EventSource::NetworkRequest => write!(f, "Network Request"),
            EventSource::TestCommand => write!(f, "Test Command"),
            EventSource::UserInteraction => write!(f, "User Interaction"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub component: ComponentType,
    pub total_events: u64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: u64,
    pub max_duration_ms: u64,
    pub p50_duration_ms: u64,
    pub p95_duration_ms: u64,
    pub p99_duration_ms: u64,
    pub events_per_second: f64,
    pub error_rate: f64,
    pub last_updated: DateTime<Utc>,
}

impl PerformanceMetrics {
    pub fn new(component: ComponentType) -> Self {
        Self {
            component,
            total_events: 0,
            avg_duration_ms: 0.0,
            min_duration_ms: 0,
            max_duration_ms: 0,
            p50_duration_ms: 0,
            p95_duration_ms: 0,
            p99_duration_ms: 0,
            events_per_second: 0.0,
            error_rate: 0.0,
            last_updated: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub summary: String,
    pub total_events: u64,
    pub active_monitors: Vec<String>,
    pub performance_metrics: Vec<PerformanceMetrics>,
    pub last_event_timestamp: Option<DateTime<Utc>>,
    pub uptime_seconds: u64,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f32,
}

impl SystemStatus {
    pub fn new() -> Self {
        Self {
            summary: "System initializing".to_string(),
            total_events: 0,
            active_monitors: Vec::new(),
            performance_metrics: Vec::new(),
            last_event_timestamp: None,
            uptime_seconds: 0,
            memory_usage_mb: 0,
            cpu_usage_percent: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetrics {
    pub model_type: String,
    pub estimated_tokens: u32,
    pub generation_time_ms: u64,
    pub tokens_per_second: f64,
    pub prompt_length: usize,
    pub response_length: usize,
    pub timestamp: DateTime<Utc>,
}

impl TokenMetrics {
    pub fn new(
        model_type: String,
        estimated_tokens: u32,
        generation_time_ms: u64,
        prompt_length: usize,
        response_length: usize,
    ) -> Self {
        let tokens_per_second = if generation_time_ms > 0 {
            (estimated_tokens as f64) / (generation_time_ms as f64 / 1000.0)
        } else {
            0.0
        };

        Self {
            model_type,
            estimated_tokens,
            generation_time_ms,
            tokens_per_second,
            prompt_length,
            response_length,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLatency {
    pub command: String,
    pub working_directory: String,
    pub exit_code: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub cpu_usage_percent: f32,
    pub memory_usage_kb: u64,
}

impl CommandLatency {
    pub fn new(
        command: String,
        working_directory: String,
        exit_code: i32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        duration_ms: u64,
    ) -> Self {
        Self {
            command,
            working_directory,
            exit_code,
            start_time,
            end_time,
            duration_ms,
            cpu_usage_percent: 0.0,
            memory_usage_kb: 0,
        }
    }

    pub fn with_resource_usage(mut self, cpu_percent: f32, memory_kb: u64) -> Self {
        self.cpu_usage_percent = cpu_percent;
        self.memory_usage_kb = memory_kb;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInteraction {
    pub model_type: String,
    pub interaction_type: ModelInteractionType,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub duration_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub context_length: Option<usize>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelInteractionType {
    Completion,
    ChatCompletion,
    CodeCompletion,
    Embedding,
    FineTuning,
    Other,
}

impl fmt::Display for ModelInteractionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelInteractionType::Completion => write!(f, "Completion"),
            ModelInteractionType::ChatCompletion => write!(f, "Chat Completion"),
            ModelInteractionType::CodeCompletion => write!(f, "Code Completion"),
            ModelInteractionType::Embedding => write!(f, "Embedding"),
            ModelInteractionType::FineTuning => write!(f, "Fine-tuning"),
            ModelInteractionType::Other => write!(f, "Other"),
        }
    }
}

impl ModelInteraction {
    pub fn new(
        model_type: String,
        interaction_type: ModelInteractionType,
        duration_ms: u64,
        success: bool,
    ) -> Self {
        Self {
            model_type,
            interaction_type,
            prompt_tokens: None,
            completion_tokens: None,
            total_tokens: None,
            duration_ms,
            success,
            error_message: None,
            timestamp: Utc::now(),
            context_length: None,
        }
    }

    pub fn tokens_per_second(&self) -> Option<f64> {
        if let Some(tokens) = self.total_tokens {
            if self.duration_ms > 0 {
                Some((tokens as f64) / (self.duration_ms as f64 / 1000.0))
            } else {
                None
            }
        } else {
            None
        }
    }
}