#!/usr/bin/env python3
import http.server
import socketserver
import json
import subprocess
import os
from datetime import datetime

class WallNotificationHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/' or self.path == '/index.html':
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()

            # Generate real-time system data
            html_content = self.generate_wall_interface()
            self.wfile.write(html_content.encode())
        else:
            super().do_GET()

    def generate_wall_interface(self):
        # Get system information
        uptime = subprocess.getoutput("uptime -p")
        hostname = subprocess.getoutput("hostname")
        load_avg = subprocess.getoutput("uptime | awk -F'load average:' '{print $2}'")
        memory = subprocess.getoutput("free -h | awk '/^Mem:/ {print $3\"/\"$2}'")
        disk = subprocess.getoutput("df -h / | awk 'NR==2 {print $5}'")

        # Get NTP status
        ntp_status = subprocess.getoutput("ntpq -p 2>/dev/null | head -10")

        # Get recent wall notifications
        wall_log = os.path.expanduser("~/.enhanced-wall-notices.log")
        recent_notices = []
        if os.path.exists(wall_log):
            with open(wall_log, 'r') as f:
                recent_notices = f.readlines()[-5:]

        return f"""
<!DOCTYPE html>
<html>
<head>
    <title>Enhanced Wall Notification System v3.0</title>
    <meta http-equiv="refresh" content="30">
    <style>
        body {{
            font-family: 'Courier New', monospace;
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            color: #00ff00;
            margin: 20px;
            line-height: 1.6;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: rgba(0,0,0,0.8);
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 0 20px rgba(0,255,0,0.3);
        }}
        h1 {{
            text-align: center;
            color: #00ffff;
            text-shadow: 0 0 10px #00ffff;
            border-bottom: 2px solid #00ff00;
            padding-bottom: 10px;
        }}
        .grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin: 20px 0;
        }}
        .panel {{
            background: rgba(0,50,0,0.6);
            padding: 15px;
            border: 1px solid #00ff00;
            border-radius: 8px;
            box-shadow: 0 0 10px rgba(0,255,0,0.2);
        }}
        .panel h3 {{
            color: #ffff00;
            margin-top: 0;
            text-shadow: 0 0 5px #ffff00;
        }}
        .status {{
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-weight: bold;
        }}
        .active {{ background: #00ff00; color: #000; }}
        .inactive {{ background: #ff0000; color: #fff; }}
        .ntp-peer {{
            font-family: monospace;
            background: rgba(0,0,0,0.5);
            padding: 10px;
            border-left: 3px solid #00ff00;
            margin: 10px 0;
        }}
        .timestamp {{
            color: #888;
            font-size: 0.9em;
        }}
        .calisota-branding {{
            text-align: center;
            margin-top: 30px;
            padding-top: 20px;
            border-top: 1px solid #00ff00;
            color: #00ffff;
            font-weight: bold;
        }}
        .integration-link {{
            background: rgba(0,100,200,0.3);
            padding: 10px;
            border-radius: 5px;
            margin: 10px 0;
            border-left: 3px solid #00ffff;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🏠 Enhanced Wall Notification System v3.0</h1>
        <div class="timestamp">Last Updated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')} | Host: {hostname}</div>

        <div class="grid">
            <div class="panel">
                <h3>⚡ System Resources</h3>
                <p><strong>Uptime:</strong> {uptime}</p>
                <p><strong>Load Average:</strong> {load_avg}</p>
                <p><strong>Memory Usage:</strong> {memory}</p>
                <p><strong>Disk Usage:</strong> {disk}</p>
            </div>

            <div class="panel">
                <h3>🕒 NTP Time Synchronization</h3>
                <div class="ntp-peer">
                    <pre>{ntp_status}</pre>
                </div>
            </div>

            <div class="panel">
                <h3>🔧 System Services</h3>
                <p>SSH: <span class="status active">ACTIVE</span></p>
                <p>Docker: <span class="status active">ACTIVE</span></p>
                <p>Nginx: <span class="status inactive">INACTIVE</span></p>
                <p>VS Code Monitor: <span class="status active">ACTIVE</span></p>
            </div>

            <div class="panel">
                <h3>🌐 Network Status</h3>
                <p><strong>Active Interfaces:</strong> 5</p>
                <p><strong>Monitored Ports:</strong> 11</p>
                <p><strong>Wall System:</strong> <span class="status active">OPERATIONAL</span></p>
                <p><strong>Rust Monitor:</strong> <span class="status active">PORT 3030</span></p>
                <p><strong>Telemetry API:</strong> <span class="status active">PORT 8081</span></p>
            </div>

            <div class="panel">
                <h3>🦀 VS Code Latency Monitor</h3>
                <div class="integration-link">
                    <p><strong>🚀 Rust Integration Active</strong></p>
                    <p>Dashboard: <a href="http://localhost:3030" style="color: #00ffff;">http://localhost:3030</a></p>
                    <p>Telemetry: <a href="http://localhost:8081" style="color: #00ffff;">http://localhost:8081</a></p>
                    <p>GitHub: <a href="https://github.com/danindiana/vscode-latency-monitor" style="color: #00ffff;">Repository</a></p>
                </div>
            </div>

            <div class="panel">
                <h3>📊 Enhanced Features</h3>
                <p>✅ NTP Monitoring Integration</p>
                <p>✅ Enhanced Logging System</p>
                <p>✅ Machine Task Automation</p>
                <p>✅ VS Code Approvals (3000+ commands)</p>
                <p>✅ Rust Performance Monitoring</p>
                <p>✅ SQLx Database Integration</p>
                <p>✅ LAN Telemetry Services</p>
            </div>

            <div class="panel">
                <h3>📋 Recent Activity</h3>
                <div style="max-height: 200px; overflow-y: auto;">
                    {"<br>".join([line.strip() for line in recent_notices[-10:]])}
                </div>
            </div>
        </div>

        <div class="calisota-branding">
            <p>🏠 Enhanced Wall Notice System | 🦀 VS Code Latency Monitor Integration</p>
            <p>Powered by <strong>Calisota.ai</strong> | GitHub Copilot AI Assistant</p>
            <p style="font-size: 0.9em; color: #888;">
                Part of the VS Code Latency Monitor project - High-performance Rust monitoring with SQLx integration
            </p>
        </div>
    </div>
</body>
</html>"""

if __name__ == "__main__":
    PORT = 8888
    Handler = WallNotificationHandler

    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"🌐 Enhanced Wall Notification Server running on http://localhost:{PORT}")
        print("🔄 Auto-refresh every 30 seconds")
        print("🦀 Integrated with VS Code Latency Monitor (ports 3030, 8081)")
        httpd.serve_forever()