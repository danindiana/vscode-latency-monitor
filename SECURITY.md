# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Security Model

### LAN-First Design
This project is designed for **trusted Local Area Network (LAN) environments** such as:
- Home networks
- Office networks  
- Development environments
- Isolated network segments

### Security Assumptions
- **Trusted Network**: All devices on the network are trusted
- **Internal Use**: Not designed for internet-facing deployment
- **No Authentication**: Services accessible without credentials within the LAN
- **Plain HTTP**: No encryption for local network communication

## Security Features

### Network Security
- **LAN Binding**: Services bind to `0.0.0.0` for LAN access only
- **Port Management**: Automatic conflict detection and resolution
- **CORS Enabled**: Permissive for local development and monitoring tools
- **Health Endpoints**: Service discovery and health monitoring

### Data Security
- **Local Storage**: All data stored locally on the monitoring host
- **SQLite Database**: File-based storage with standard filesystem permissions
- **No Remote Data**: No data transmitted outside the local network
- **Memory Safety**: Rust's memory safety prevents common vulnerabilities

### Process Security
- **Minimal Privileges**: Runs with user-level permissions
- **Resource Limits**: Bounded memory usage (<5MB typical)
- **Clean Shutdown**: Graceful cleanup of resources and connections
- **Error Handling**: Comprehensive error handling prevents crashes

## Deployment Recommendations

### Secure LAN Deployment
```bash
# Use specific interface binding for additional security
cargo run -- dashboard --bind 192.168.1.100 --port 3030

# Run with restricted user account
sudo -u monitoring cargo run -- start --daemon

# Use firewall rules to limit access
sudo ufw allow from 192.168.1.0/24 to any port 3030
sudo ufw allow from 192.168.1.0/24 to any port 8081
```

### Network Isolation
- Deploy on isolated network segments
- Use VPN for remote access instead of internet exposure
- Implement network-level access controls
- Monitor network traffic for anomalies

## Reporting a Vulnerability

### What to Report
Please report security vulnerabilities for:
- **Authentication bypass** in LAN environment
- **Code injection** vulnerabilities
- **Memory safety** issues
- **Privilege escalation** potential
- **Data exposure** beyond intended scope
- **Denial of service** vulnerabilities

### How to Report
1. **Email**: Send details to [security@your-domain.com]
2. **Private Issue**: Use GitHub's private vulnerability reporting
3. **Encrypted Communication**: Use GPG if handling sensitive details

### What to Include
- **Description**: Clear description of the vulnerability
- **Impact**: Potential impact and attack scenarios  
- **Reproduction**: Steps to reproduce the issue
- **Environment**: OS, Rust version, network configuration
- **Proof of Concept**: Code or commands demonstrating the issue

### Response Timeline
- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Fix Development**: Depends on severity and complexity
- **Public Disclosure**: After fix is available and deployed

## Security Guidelines for Users

### Safe Deployment
```bash
# Check network configuration
ip route show
netstat -tulpn | grep -E ':(3030|8081)'

# Verify service binding
ss -tlnp | grep -E ':(3030|8081)'

# Test access controls
curl http://localhost:3030/health   # Should work
curl http://external-ip:3030/health # Should not work from internet
```

### Monitoring Security
```bash
# Monitor access logs
journalctl -f | grep -E '(3030|8081)'

# Check for unusual connections
netstat -an | grep -E ':(3030|8081)'

# Verify process permissions
ps aux | grep vscode-latency-monitor
```

### Security Checklist
- [ ] **Network Isolation**: Confirm services not accessible from internet
- [ ] **Firewall Rules**: Implement appropriate network controls
- [ ] **User Permissions**: Run with minimal required privileges
- [ ] **Log Monitoring**: Monitor access and error logs
- [ ] **Regular Updates**: Keep dependencies and Rust toolchain updated
- [ ] **Backup Strategy**: Secure backup of configuration and data

## Threat Model

### In Scope
- **Malicious LAN Users**: Protection against compromised devices on network
- **Process Isolation**: Prevent interference with other system processes
- **Data Integrity**: Ensure monitoring data accuracy and completeness
- **Service Availability**: Prevent denial of service attacks

### Out of Scope
- **Internet-based Attacks**: Not designed for internet-facing deployment
- **Physical Security**: Host system physical security is user responsibility
- **Network Infrastructure**: Router, switch, and network equipment security
- **Operating System**: Underlying OS security and patch management

### Known Limitations
1. **No Authentication**: Services accessible to all LAN users
2. **Plain HTTP**: No encryption for network communication
3. **CORS Permissive**: Allows cross-origin requests within LAN
4. **Debug Information**: Verbose error messages may leak system information

## Security Updates

### Dependency Management
```bash
# Check for security advisories
cargo audit

# Update dependencies
cargo update

# Review dependency licenses
cargo tree --format "{p} {l}"
```

### Update Process
1. **Monitor Advisories**: Track Rust security advisories
2. **Dependency Updates**: Regular updates of critical dependencies
3. **Version Pinning**: Pin major versions for stability
4. **Testing**: Comprehensive testing after security updates

## Compliance and Standards

### Development Security
- **Memory Safety**: Rust language provides memory safety guarantees
- **Input Validation**: All user inputs validated and sanitized
- **Error Handling**: Comprehensive error handling prevents information leakage
- **Code Review**: All changes reviewed for security implications

### Data Handling
- **Local Storage**: All data stored locally with standard filesystem permissions
- **No Sensitive Data**: System monitoring data only, no user credentials
- **Data Retention**: Configurable retention periods for monitoring data
- **Clean Shutdown**: Proper cleanup of temporary data and connections

## Contact Information

For security-related questions or concerns:
- **Security Team**: [security@your-domain.com]
- **Project Maintainer**: [maintainer@your-domain.com]
- **GitHub Security**: Use GitHub's private vulnerability reporting

---

**Last Updated**: 2025-10-15  
**Security Model**: LAN-trusted environment  
**Threat Level**: Low (internal monitoring system)  
**Review Schedule**: Quarterly security reviews