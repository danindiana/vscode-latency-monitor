# Enhanced Wall Notification System Integration

## Overview

This directory contains the Enhanced Wall Notification System v3.0, integrated as a complementary web interface for the VS Code Latency Monitor project. It provides a real-time web dashboard for system monitoring that works alongside the Rust-based monitoring services.

## Features

- **Real-time System Monitoring**: CPU, memory, disk usage, and system uptime
- **NTP Time Synchronization**: Live display of NTP peer status and time sync information
- **Service Status**: Monitor system services including SSH, Docker, and the VS Code Latency Monitor
- **Network Integration**: Shows status of Rust monitoring services (ports 3030, 8081)
- **Enhanced Logging**: Displays recent wall notifications from `~/.enhanced-wall-notices.log`
- **Auto-refresh**: Updates every 30 seconds automatically
- **Calisota.ai Integration**: Professional branding and project attribution

## Quick Start

### Start the Wall Notification Server

```bash
# From the VS Code Latency Monitor root directory
cd system-notifications
python3 server.py
```

The server will start on port 8888 by default.

### Access the Interface

- **Local**: http://localhost:8888
- **LAN**: http://[YOUR_IP]:8888
- **Auto-refresh**: Page updates every 30 seconds

## Integration with VS Code Latency Monitor

The wall notification system is designed to work seamlessly with the main Rust application:

### Service Integration
- **Dashboard Link**: Direct links to Rust dashboard (port 3030)
- **Telemetry Link**: Direct links to telemetry API (port 8081)
- **Status Monitoring**: Shows operational status of Rust services
- **GitHub Integration**: Links to the main project repository

### Complementary Monitoring
- **System-Level**: Wall notifications show OS-level metrics
- **Application-Level**: Rust monitor shows VS Code specific latency
- **Combined View**: Both services provide comprehensive monitoring coverage

## Configuration

### Port Configuration
Default port is 8888. To use a different port, modify the `PORT` variable in `server.py`:

```python
PORT = 8888  # Change to desired port
```

### Log File Location
The system reads wall notifications from:
```
~/.enhanced-wall-notices.log
```

This log file is automatically updated by the Enhanced Wall Notification System and the VS Code Latency Monitor.

## Architecture

```
VS Code Latency Monitor Project
├── src/                          # Rust source code
├── system-notifications/         # Wall notification system
│   ├── server.py                # Python web server
│   └── README.md               # This file
├── docs/                        # Documentation
└── Cargo.toml                  # Rust project configuration
```

## Dependencies

### Python Requirements
- Python 3.6+
- Standard library modules (no external dependencies)

### System Requirements
- Linux/Unix system with standard utilities (`uptime`, `free`, `df`, etc.)
- NTP daemon for time synchronization monitoring
- Access to system logs and status information

## Security Considerations

- **LAN Access**: Server binds to all interfaces (0.0.0.0) for LAN accessibility
- **No Authentication**: Designed for trusted network environments
- **Read-Only**: Server only reads system information, no write operations
- **Subprocess Calls**: Uses subprocess for system information gathering

## Customization

### Adding Custom Panels
Modify the `generate_wall_interface()` method to add custom monitoring panels:

```python
def generate_wall_interface(self):
    # Add custom system checks here
    custom_metric = subprocess.getoutput("your-command-here")
    
    # Include in HTML template
    # ... rest of method
```

### Styling Modifications
The CSS is embedded in the HTML template. Key style classes:
- `.panel`: Individual monitoring panels
- `.status`: Service status indicators
- `.active/.inactive`: Status color coding
- `.calisota-branding`: Footer branding section

## Integration Examples

### Starting All Services
```bash
# Terminal 1: Start Rust dashboard
cargo run -- dashboard --port 3030 &

# Terminal 2: Start Rust telemetry
cargo run -- telemetry --port 8081 &

# Terminal 3: Start wall notifications
cd system-notifications && python3 server.py &
```

### Health Check Script
```bash
#!/bin/bash
echo "🔍 Checking all monitoring services..."

# Check Rust services
curl -s http://localhost:3030/health && echo "✅ Dashboard OK"
curl -s http://localhost:8081/health && echo "✅ Telemetry OK"

# Check wall notification service
curl -s http://localhost:8888/ >/dev/null && echo "✅ Wall System OK"
```

## Troubleshooting

### Common Issues

1. **Port Already in Use**
   - Check for existing processes: `lsof -i :8888`
   - Kill existing process or change port

2. **Permission Denied**
   - Ensure script is executable: `chmod +x server.py`
   - Check Python installation: `python3 --version`

3. **NTP Data Missing**
   - Install NTP daemon: `sudo apt install ntp`
   - Check NTP status: `systemctl status ntp`

4. **System Commands Failed**
   - Verify required utilities are installed
   - Check user permissions for system information access

## Development

### Adding New Features
1. Modify `server.py` for backend changes
2. Update HTML template for frontend changes
3. Test with different system configurations
4. Update this README with new features

### Contributing
This is part of the larger VS Code Latency Monitor project. See the main project README for contribution guidelines.

## License

This integration is part of the VS Code Latency Monitor project and follows the same MIT License. See the main project LICENSE file for details.

## Links

- **Main Project**: VS Code Latency Monitor (Rust)
- **GitHub Repository**: https://github.com/danindiana/vscode-latency-monitor
- **Calisota.ai**: Project sponsor and development organization
- **Enhanced Wall Notification System**: Original inspiration for this integration

---

**Built with ❤️ by Calisota.ai | Part of the VS Code Latency Monitor ecosystem**