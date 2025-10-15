use anyhow::Result;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};
use sysinfo::System;
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::storage::MetricsStorage;
use crate::config::Config;
use crate::models::{LatencyEvent, ComponentType, EventSource};

pub struct LatencyMonitor {
    config: Config,
    storage: MetricsStorage,
    event_sender: Sender<LatencyEvent>,
    event_receiver: Receiver<LatencyEvent>,
    system: System,
    running: bool,
}

impl LatencyMonitor {
    pub async fn new(config: Config, storage: MetricsStorage) -> Result<Self> {
        let (sender, receiver) = unbounded();
        let mut system = System::new_all();
        system.refresh_all();

        Ok(Self {
            config,
            storage,
            event_sender: sender,
            event_receiver: receiver,
            system,
            running: false,
        })
    }

    pub async fn start_vscode_monitoring(&mut self, interval_ms: u64) -> Result<()> {
        info!("Starting VS Code process monitoring");
        
        let sender = self.event_sender.clone();
        let interval = Duration::from_millis(interval_ms);
        
        tokio::spawn(async move {
            loop {
                let start_time = Instant::now();
                
                // Monitor VS Code processes
                let mut system = System::new_all();
                system.refresh_processes();
                
                let vscode_processes: Vec<_> = system.processes()
                    .iter()
                    .filter(|(_, proc)| {
                        let name = proc.name().to_lowercase();
                        name.contains("code") && 
                        (name.contains("code-server") || 
                         name.contains("code.exe") || 
                         name == "code")
                    })
                    .collect();

                for (pid, process) in &vscode_processes {
                    let cpu_usage = process.cpu_usage();
                    let memory = process.memory();

                    // Create latency event for process metrics
                    let event = LatencyEvent::new(
                        ComponentType::VSCode,
                        EventSource::ProcessMonitor,
                        start_time.elapsed(),
                        format!("Process {} - CPU: {:.1}%, Memory: {}KB", 
                                pid, cpu_usage, memory / 1024),
                    );

                    if let Err(e) = sender.send(event) {
                        warn!("Failed to send VS Code monitoring event: {}", e);
                    }
                }

                // Monitor VS Code extension host processes
                let extension_hosts: Vec<_> = system.processes()
                    .iter()
                    .filter(|(_, proc)| {
                        proc.name().to_lowercase().contains("extensionhost") ||
                        proc.cmd().iter().any(|arg| arg.contains("extensionHost"))
                    })
                    .collect();

                for (pid, process) in &extension_hosts {
                    let event = LatencyEvent::new(
                        ComponentType::VSCodeExtension,
                        EventSource::ExtensionHost,
                        start_time.elapsed(),
                        format!("Extension Host {} - CPU: {:.1}%", pid, process.cpu_usage()),
                    );

                    if let Err(e) = sender.send(event) {
                        warn!("Failed to send extension host event: {}", e);
                    }
                }

                sleep(interval).await;
            }
        });

        Ok(())
    }

    pub async fn start_model_monitoring(&mut self, interval_ms: u64) -> Result<()> {
        info!("Starting AI model interaction monitoring");
        
        let sender = self.event_sender.clone();
        let interval = Duration::from_millis(interval_ms);
        
        tokio::spawn(async move {
            loop {
                let start_time = Instant::now();
                
                // Monitor GitHub Copilot processes
                let mut system = System::new_all();
                system.refresh_processes();
                
                // Look for Copilot-related processes
                let copilot_processes: Vec<_> = system.processes()
                    .iter()
                    .filter(|(_, proc)| {
                        let name = proc.name().to_lowercase();
                        let cmd_line = proc.cmd()
                            .join(" ")
                            .to_lowercase();
                        
                        name.contains("copilot") || 
                        cmd_line.contains("github.copilot") ||
                        cmd_line.contains("copilot-agent")
                    })
                    .collect();

                for (pid, process) in &copilot_processes {
                    let event = LatencyEvent::new(
                        ComponentType::GitHubCopilot,
                        EventSource::ModelProcess,
                        start_time.elapsed(),
                        format!("Copilot Process {} - CPU: {:.1}%", pid, process.cpu_usage()),
                    );

                    if let Err(e) = sender.send(event) {
                        warn!("Failed to send Copilot monitoring event: {}", e);
                    }
                }

                // Monitor local model processes (ollama, etc.)
                let local_model_patterns = ["ollama", "llama", "gpt4all", "localai"];
                
                for pattern in &local_model_patterns {
                    let matching_processes: Vec<_> = system.processes()
                        .iter()
                        .filter(|(_, proc)| {
                            proc.name().to_lowercase().contains(pattern) ||
                            proc.cmd().iter().any(|arg| arg.to_lowercase().contains(pattern))
                        })
                        .collect();

                    for (pid, process) in &matching_processes {
                        let event = LatencyEvent::new(
                            ComponentType::LocalModel,
                            EventSource::ModelProcess,
                            start_time.elapsed(),
                            format!("Local Model ({}) {} - CPU: {:.1}%", 
                                    pattern, pid, process.cpu_usage()),
                        );

                        if let Err(e) = sender.send(event) {
                            warn!("Failed to send local model event: {}", e);
                        }
                    }
                }

                sleep(interval).await;
            }
        });

        Ok(())
    }

    pub async fn start_terminal_monitoring(&mut self, interval_ms: u64) -> Result<()> {
        info!("Starting terminal command monitoring");
        
        let sender = self.event_sender.clone();
        let interval = Duration::from_millis(interval_ms);
        
        tokio::spawn(async move {
            loop {
                let start_time = Instant::now();
                
                // Monitor terminal processes
                let mut system = System::new_all();
                system.refresh_processes();
                
                let terminal_processes: Vec<_> = system.processes()
                    .iter()
                    .filter(|(_, proc)| {
                        let name = proc.name().to_lowercase();
                        name == "bash" || name == "zsh" || name == "fish" || 
                        name == "sh" || name.contains("terminal") ||
                        name.contains("gnome-terminal") || name.contains("konsole")
                    })
                    .collect();

                for (pid, process) in &terminal_processes {
                    if process.cpu_usage() > 0.1 { // Only log active terminals
                        let event = LatencyEvent::new(
                            ComponentType::Terminal,
                            EventSource::ProcessMonitor,
                            start_time.elapsed(),
                            format!("Terminal {} - CPU: {:.1}%", pid, process.cpu_usage()),
                        );

                        if let Err(e) = sender.send(event) {
                            warn!("Failed to send terminal monitoring event: {}", e);
                        }
                    }
                }

                sleep(interval).await;
            }
        });

        Ok(())
    }

    pub async fn start_all_monitoring(&mut self, interval_ms: u64) -> Result<()> {
        info!("Starting comprehensive monitoring for all components");
        
        self.start_vscode_monitoring(interval_ms).await?;
        self.start_model_monitoring(interval_ms * 2).await?; // Models less frequently
        self.start_terminal_monitoring(interval_ms).await?;
        
        Ok(())
    }

    pub async fn run_daemon(&mut self) -> Result<()> {
        info!("Running latency monitor as daemon");
        self.running = true;

        // Start event processing task
        let storage = self.storage.clone();
        let receiver = self.event_receiver.clone();
        
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv() {
                debug!("Processing latency event: {:?}", event);
                
                if let Err(e) = storage.store_event(&event).await {
                    warn!("Failed to store event: {}", e);
                }
            }
        });

        // Keep daemon running
        while self.running {
            sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }

    pub async fn run_foreground(&mut self) -> Result<()> {
        info!("Running latency monitor in foreground");
        self.running = true;

        // Start event processing
        let storage = self.storage.clone();
        let receiver = self.event_receiver.clone();
        
        let processing_task = tokio::spawn(async move {
            while let Ok(event) = receiver.recv() {
                debug!("Processing latency event: {:?}", event);
                
                if let Err(e) = storage.store_event(&event).await {
                    warn!("Failed to store event: {}", e);
                } else {
                    // Print to console for immediate feedback
                    println!("[{}] {} - {}ms - {}", 
                        event.timestamp.format("%H:%M:%S"),
                        event.component_type,
                        event.duration.as_millis(),
                        event.description
                    );
                }
            }
        });

        // Wait for shutdown signal (Ctrl+C)
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("Received shutdown signal");
                self.running = false;
            }
            _ = processing_task => {
                info!("Event processing task completed");
            }
        }

        Ok(())
    }

    pub async fn measure_command_latency<F, Fut, T>(&self, 
        component: ComponentType,
        source: EventSource,
        description: String,
        operation: F
    ) -> Result<T> 
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let start_time = Instant::now();
        
        let result = operation().await?;
        
        let duration = start_time.elapsed();
        let event = LatencyEvent::new(component, source, duration, description);
        
        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to send latency measurement event: {}", e);
        }
        
        Ok(result)
    }

    pub async fn test_vscode_monitoring(&self, iterations: usize) -> Result<()> {
        info!("Testing VS Code monitoring for {} iterations", iterations);
        
        for i in 0..iterations {
            let start_time = Instant::now();
            
            // Simulate VS Code command execution
            sleep(Duration::from_millis(10 + (i % 50) as u64)).await;
            
            let duration = start_time.elapsed();
            let event = LatencyEvent::new(
                ComponentType::VSCode,
                EventSource::TestCommand,
                duration,
                format!("Test VS Code command #{}", i + 1),
            );

            self.event_sender.send(event)?;
        }
        
        Ok(())
    }

    pub async fn test_model_monitoring(&self, iterations: usize) -> Result<()> {
        info!("Testing model monitoring for {} iterations", iterations);
        
        for i in 0..iterations {
            let start_time = Instant::now();
            
            // Simulate model interaction
            sleep(Duration::from_millis(100 + (i % 200) as u64)).await;
            
            let duration = start_time.elapsed();
            let event = LatencyEvent::new(
                ComponentType::GitHubCopilot,
                EventSource::TestCommand,
                duration,
                format!("Test model interaction #{}", i + 1),
            );

            self.event_sender.send(event)?;
        }
        
        Ok(())
    }

    pub async fn test_terminal_monitoring(&self, iterations: usize) -> Result<()> {
        info!("Testing terminal monitoring for {} iterations", iterations);
        
        for i in 0..iterations {
            let start_time = Instant::now();
            
            // Simulate terminal command
            sleep(Duration::from_millis(20 + (i % 80) as u64)).await;
            
            let duration = start_time.elapsed();
            let event = LatencyEvent::new(
                ComponentType::Terminal,
                EventSource::TestCommand,
                duration,
                format!("Test terminal command #{}", i + 1),
            );

            self.event_sender.send(event)?;
        }
        
        Ok(())
    }

    pub async fn test_all_components(&self, iterations: usize) -> Result<()> {
        info!("Testing all components for {} iterations each", iterations);
        
        self.test_vscode_monitoring(iterations).await?;
        self.test_model_monitoring(iterations).await?;
        self.test_terminal_monitoring(iterations).await?;
        
        // Wait for events to be processed
        sleep(Duration::from_millis(500)).await;
        
        Ok(())
    }

    pub fn stop(&mut self) {
        info!("Stopping latency monitor");
        self.running = false;
    }
}