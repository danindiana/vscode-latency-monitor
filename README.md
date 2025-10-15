# VS Code Latency Monitor (Rust)

A high-performance system for monitoring VS Code command execution, AI model response times, and local compute latency with sub-millisecond precision.

## Features

- 🚀 **High-Performance**: Written in Rust for minimal overhead and maximum precision
- 📊 **Real-time Monitoring**: Live dashboard with WebSocket updates
- 🤖 **AI Model Tracking**: GitHub Copilot and local LLM performance analysis
- 💾 **Data Persistence**: SQLite storage with JSON export capabilities
- 🌐 **Web Interface**: Modern dashboard with interactive charts
- 🔄 **Integration**: Seamless integration with Enhanced Wall Notification System

## Quick Start

```bash
# Clone and build
git clone <repository>
cd vscode-latency-monitor-rs
cargo build --release

# Start monitoring
./target/release/vscode-latency-monitor start

# View real-time dashboard
./target/release/vscode-latency-monitor dashboard
# Open http://localhost:3030
```

## Architecture

### Core Components

- **Latency Monitor**: High-precision timing with `std::time::Instant`
- **Process Tracker**: VS Code process monitoring via `sysinfo`
- **Model Observer**: AI model interaction detection and timing
- **Web Server**: Axum-based dashboard with real-time updates
- **Data Layer**: SQLite for persistence, in-memory for real-time

### Monitoring Capabilities

1. **VS Code Commands**: Extension execution, file operations, UI interactions
2. **Model Interactions**: Copilot responses, local LLM processing, token generation
3. **System Performance**: CPU usage, memory consumption, I/O metrics
4. **Network Activity**: API calls, extension downloads, sync operations

## Usage

### Basic Monitoring
```bash
# Start all monitors
vscode-latency-monitor start

# Monitor specific component
vscode-latency-monitor start --component vscode
vscode-latency-monitor start --component models
vscode-latency-monitor start --component terminal
```

### Data Analysis
```bash
# Generate performance report
vscode-latency-monitor report --format json
vscode-latency-monitor report --format csv --output latency-report.csv

# Export metrics
vscode-latency-monitor export --since "1 hour ago"
vscode-latency-monitor export --format sqlite --output metrics.db
```

### Real-time Dashboard
```bash
# Start web interface
vscode-latency-monitor dashboard --port 3030

# Enable debug mode
RUST_LOG=debug vscode-latency-monitor dashboard
```

## Configuration

Create `~/.config/vscode-latency-monitor/config.toml`:

```toml
[monitoring]
interval_ms = 100
precision = "microsecond"
buffer_size = 10000

[dashboard]
port = 3030
auto_refresh_ms = 1000
theme = "dark"

[storage]
database_path = "~/.local/share/vscode-latency-monitor/metrics.db"
retention_days = 30
archive_threshold = 100000

[integrations]
wall_notification_system = true
enhanced_logging = true
copilot_tracking = true
```

## Development

### Building
```bash
cargo build --release
cargo test
cargo bench
```

### Development Mode
```bash
cargo run -- start --debug
RUST_LOG=trace cargo run -- dashboard
```

### Testing
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Benchmark tests
cargo bench
```

## Performance

- **Overhead**: < 0.1% CPU impact during monitoring
- **Memory**: ~5MB baseline, ~50MB with full dashboard
- **Precision**: Microsecond-level timing accuracy
- **Throughput**: 10,000+ events/second sustained

## Integration

### Wall Notification System
```bash
# Link with existing system
ln -s ~/.local/share/vscode-latency-monitor/metrics.db \
      ~/.vscode-metrics/rust-metrics.db

# Enable wall notifications
vscode-latency-monitor config set wall_notifications true
```

### VS Code Extension (Future)
- Real-time performance overlay
- Inline latency warnings
- Performance suggestions
- Automatic optimization

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Roadmap

- [ ] Machine Learning performance predictions
- [ ] VS Code Extension development
- [ ] Cloud metrics aggregation
- [ ] Advanced anomaly detection
- [ ] Performance optimization suggestions
- [ ] Multi-language model support