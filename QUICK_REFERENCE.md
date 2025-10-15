# Quick Reference Guide - VS Code Latency Monitor

## 🚀 Quick Start Commands

### Starting All Services
```bash
cd /home/jeb/programs/vscode-latency-monitor-rs

# Start Rust dashboard
cargo run -- dashboard --port 3030 &

# Start Rust telemetry API
cargo run -- telemetry --port 8081 &

# Start Python wall notification system
cd system-notifications && python3 server.py &
```

### Health Checks
```bash
# Check all services
curl http://localhost:3030/health  # Dashboard
curl http://localhost:8081/health  # Telemetry
curl http://localhost:8888/        # Wall System
```

### Stopping Services
```bash
# Find and stop all monitoring services
pkill -f "vscode-latency-monitor"
pkill -f "server.py"
```

## 📁 Important Locations

### Project Directory
```
/home/jeb/programs/vscode-latency-monitor-rs/
```

### Key Files
- **Source Code**: `src/` directory (7 Rust modules)
- **Wall System**: `system-notifications/` directory
- **Documentation**: Root directory (12+ guides)
- **Database**: `~/.local/share/vscode-latency-monitor/metrics.db`
- **Wall Log**: `~/.enhanced-wall-notices.log`

### Configuration
- **Cargo.toml**: Rust dependencies and project metadata
- **static/dashboard.html**: Web dashboard interface

## 🌐 Service Endpoints

### Dashboard (Port 3030)
- **Main**: http://localhost:3030
- **Health**: http://localhost:3030/health
- **LAN**: http://192.168.1.143:3030

### Telemetry API (Port 8081)
- **Telemetry**: http://localhost:8081/api/telemetry
- **Metrics**: http://localhost:8081/api/metrics/raw
- **Summary**: http://localhost:8081/api/metrics/summary
- **System**: http://localhost:8081/api/system/resources
- **Status**: http://localhost:8081/api/monitoring/status
- **Health**: http://localhost:8081/health

### Wall Notification System (Port 8888)
- **Main**: http://localhost:8888
- **Auto-refresh**: Every 30 seconds
- **LAN**: http://192.168.1.143:8888

## 🛠️ Common Operations

### Build & Test
```bash
# Clean build
cargo clean && cargo build --release

# Run tests
cargo test

# Component testing
cargo run -- test --component vscode --iterations 3
```

### Git Operations
```bash
# Check status
git status

# View recent commits
git log --oneline -5

# Push to GitHub
git push origin master

# If authentication fails
unset GITHUB_TOKEN
git push origin master
```

### Database Operations
```bash
# View database location
echo ~/.local/share/vscode-latency-monitor/metrics.db

# Export data
cargo run -- export --format json --output metrics.json
cargo run -- export --format csv --output metrics.csv
```

## 📊 Monitoring

### System Status
```bash
# Check all services
ps aux | grep -E '(cargo run|python3 server|dashboard|telemetry)' | grep -v grep

# View logs
tail -f ~/.enhanced-wall-notices.log
tail -f telemetry.log
```

### Service Health
```bash
# Quick health check script
for port in 3030 8081 8888; do
    echo "Port $port: $(curl -s -w "%{http_code}" http://localhost:$port/health -o /dev/null 2>/dev/null || echo 'DOWN')"
done
```

## 🔧 Troubleshooting

### Port Already in Use
```bash
# Find process using port
lsof -i :3030
lsof -i :8081
lsof -i :8888

# Kill process
kill -9 <PID>
```

### Service Not Starting
```bash
# Check for compilation errors
cargo build

# Check Python dependencies
python3 --version
which python3

# View detailed errors
cargo run -- dashboard --port 3030  # Without & to see output
```

### Database Issues
```bash
# Check database exists
ls -lh ~/.local/share/vscode-latency-monitor/metrics.db

# Reset database (caution: deletes data)
rm ~/.local/share/vscode-latency-monitor/metrics.db
cargo run -- start  # Will recreate database
```

## 📝 Development Workflow

### Making Changes
```bash
# 1. Make code changes
# 2. Test locally
cargo build && cargo test

# 3. Run services to verify
cargo run -- dashboard --port 3030 &
cargo run -- telemetry --port 8081 &

# 4. Check health
curl http://localhost:3030/health

# 5. Commit changes
git add .
git commit -m "description"

# 6. Push to GitHub
git push origin master
```

### Documentation Updates
```bash
# Update relevant files:
# - README.md (main documentation)
# - CHANGELOG.md (version history)
# - Specific guides as needed

# Commit documentation
git add *.md
git commit -m "docs: Update documentation"
git push origin master
```

## 🎯 Key Commands Reference

### CLI Commands
| Command | Description | Example |
|---------|-------------|---------|
| `start` | Begin monitoring | `cargo run -- start --component vscode` |
| `stop` | Stop monitoring | `cargo run -- stop` |
| `dashboard` | Launch dashboard | `cargo run -- dashboard --port 3030` |
| `telemetry` | Start API | `cargo run -- telemetry --port 8081` |
| `report` | Generate report | `cargo run -- report --format json` |
| `export` | Export data | `cargo run -- export --format csv` |
| `config` | Manage config | `cargo run -- config list` |
| `status` | Show status | `cargo run -- status --verbose` |
| `test` | Run tests | `cargo run -- test --iterations 5` |

## 🔗 Important Links

- **GitHub Repository**: https://github.com/danindiana/vscode-latency-monitor
- **Calisota.ai**: Project organization
- **Documentation**: See README.md and comprehensive guides in project root

## ⚡ Quick Tips

1. **Always use absolute paths** when working with database files
2. **Check service health** before reporting issues
3. **Use --verbose flag** for detailed debugging output
4. **Monitor wall log** for system-wide events
5. **Keep documentation updated** with code changes
6. **Test after changes** before committing
7. **Use descriptive commit messages** for clarity
8. **Push regularly** to keep GitHub synchronized

## 🎓 Best Practices

- **Service Management**: Use background processes (&) for long-running services
- **Error Handling**: Always check service health after starting
- **Git Workflow**: Commit frequently with clear messages
- **Documentation**: Update docs alongside code changes
- **Testing**: Run component tests before major commits
- **Port Management**: Use standard ports (3030, 8081, 8888)
- **Security**: Never commit sensitive tokens or credentials

---

**Last Updated**: October 15, 2025  
**Version**: 1.0.0  
**Organization**: Calisota.ai

For detailed information, see the comprehensive guides in the project root directory.
