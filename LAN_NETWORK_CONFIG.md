# LAN Network Service Configuration

## Port Allocation Summary

This document outlines the current port allocations for the Enhanced Wall Notification System ecosystem to prevent conflicts and enable proper LAN access.

### Current Port Usage (as of 2025-10-15)

#### **Active Monitoring Services**
- **Port 3030**: VS Code Latency Monitor Dashboard (Rust)
  - Service: Real-time monitoring dashboard with Chart.js
  - Access: `http://192.168.1.143:3030`
  - Protocol: HTTP (Axum web server)
  - CORS: Enabled for LAN access
  - Status: ✅ OPERATIONAL

- **Port 8081**: Telemetry API Service (Rust) 
  - Service: Machine-readable telemetry data for external systems
  - Access: `http://192.168.1.143:8081`
  - Protocol: HTTP REST API
  - CORS: Permissive for cross-network access
  - Status: ✅ OPERATIONAL

#### **Reserved/Planned Services**
- **Port 8888**: Enhanced Wall Notification Web Interface (Python)
  - Service: Legacy wall notification system with NTP integration
  - Status: 🔄 CONFIGURED BUT NOT RUNNING

#### **System Services (In Use)**
- **Port 53**: DNS (system)
- **Port 22**: SSH (system) 
- **Port 111**: RPC (system)
- **Port 631**: CUPS printing (system)
- **Port 4317**: OpenTelemetry (system)
- **Port 8125**: StatsD (system)
- **Port 11434**: Ollama AI service (system)
- **Port 19999**: Netdata monitoring (system)

#### **Avoided Ports (Conflicts Detected)**
- **Port 8080**: Initially planned for telemetry but conflicted with existing service
  - Resolution: Moved telemetry to port 8081

## LAN Access Configuration

### Network Details
- **LAN IP**: `192.168.1.143`
- **Hostname**: `worlock`
- **Network**: All services bound to `0.0.0.0` for LAN accessibility
- **Firewall**: Services accessible across local network

### Service URLs for LAN Access

#### **Primary Monitoring Interface**
```
Dashboard: http://192.168.1.143:3030
- Real-time VS Code latency monitoring
- Interactive charts and system status
- WebSocket updates for live data
```

#### **Telemetry API for External Integration**
```
Base API: http://192.168.1.143:8081
Main UI: http://192.168.1.143:8081/

API Endpoints:
- GET /api/telemetry - Complete system telemetry
- GET /api/metrics/raw - Raw performance data (1000 events)
- GET /api/metrics/summary - Aggregated metrics with component breakdown
- GET /api/system/resources - Real-time system resource usage
- GET /api/monitoring/status - Monitor status and health
- GET /health - Simple health check
```

#### **Enhanced Wall Notifications (Planned)**
```
Wall Interface: http://192.168.1.143:8888
- System status with NTP synchronization data
- Enhanced logging integration
- Auto-refresh web interface
```

## Service Integration Matrix

| Service | Port | Technology | Purpose | LAN Access | API Available |
|---------|------|------------|---------|------------|---------------|
| VS Code Monitor Dashboard | 3030 | Rust/Axum | Interactive monitoring | ✅ | ✅ REST |
| Telemetry Service | 8081 | Rust/Axum | Machine data export | ✅ | ✅ REST |
| Wall Notifications | 8888 | Python/HTTP | System status display | ✅ | ❌ |

## External System Integration

### For Monitoring Tools (Grafana, Prometheus, etc.)
```bash
# Telemetry endpoint for external monitoring
curl http://192.168.1.143:8081/api/telemetry

# Raw metrics for time-series databases
curl http://192.168.1.143:8081/api/metrics/raw

# System resources for infrastructure monitoring  
curl http://192.168.1.143:8081/api/system/resources
```

### For Dashboard Integration
```bash
# Real-time dashboard data
curl http://192.168.1.143:3030/api/events

# System status
curl http://192.168.1.143:3030/api/status
```

### For Automation Scripts
```bash
# Health checks
curl http://192.168.1.143:8081/health
curl http://192.168.1.143:3030/health

# Monitoring status
curl http://192.168.1.143:8081/api/monitoring/status
```

## Port Conflict Resolution Process

1. **Check Current Usage**: `netstat -tulpn | grep :[PORT]`
2. **Identify Service**: `ps aux | grep [PID]`
3. **Choose Alternative**: Increment port or use designated ranges
4. **Update Configuration**: Modify service config and documentation
5. **Test LAN Access**: Verify accessibility from network devices

## Future Port Allocation Strategy

### Recommended Port Ranges
- **3000-3099**: Main dashboard services
- **8000-8099**: API and telemetry services  
- **8800-8899**: Web interfaces and status pages
- **9000-9099**: Reserved for specialized monitoring tools

### Port Allocation Rules
1. Check system services first (`netstat -tulpn`)
2. Avoid common application ports (8080, 8000, etc.)
3. Document all allocations in this file
4. Test LAN accessibility before deployment
5. Use incremental ports for related services

## Security Considerations

### LAN Security
- Services bind to `0.0.0.0` for network access
- CORS enabled permissively for local network
- No authentication required (trusted LAN environment)
- All data transmitted in plaintext HTTP

### Access Control
- Services accessible to entire local network (192.168.1.0/24)
- No IP filtering or authentication implemented
- Suitable for trusted home/office networks
- Consider VPN/firewall for external access

## Troubleshooting

### Service Not Accessible
```bash
# Check if service is running
netstat -tulpn | grep :[PORT]

# Check if bound to correct interface
ss -tlnp | grep :[PORT]

# Test local access first
curl localhost:[PORT]/health

# Test LAN access
curl 192.168.1.143:[PORT]/health
```

### Port Conflicts
```bash
# Find what's using a port
lsof -i :[PORT]

# Kill conflicting process
sudo kill [PID]

# Start service on alternative port
cargo run -- telemetry --port [NEW_PORT]
```

---
**Last Updated**: 2025-10-15 03:11 UTC  
**Network**: 192.168.1.0/24  
**Services Active**: Dashboard (3030), Telemetry (8081)  
**Status**: ✅ All services operational and LAN-accessible