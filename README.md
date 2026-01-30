<p align="center">
<img src="https://raw.githubusercontent.com/BugBlocker/lotus/master/logo/lotus_logo.png" width="370px" alt="Lotus Logo">
</p>

<h1 align="center">Lotus OSINT & Security Platform</h1>

<p align="center">
<strong>A powerful security automation framework for OSINT, reconnaissance, and vulnerability scanning</strong>
</p>

<p align="center">
<a href="#-quick-install">Install</a> •
<a href="#-features">Features</a> •
<a href="#-web-ui">Web UI</a> •
<a href="#-osint-tools">OSINT Tools</a> •
<a href="#-documentation">Docs</a> •
<a href="https://discord.gg/nBYDPTzjSq">Discord</a>
</p>

---

## 🌸 What is Lotus?

**Lotus** is an advanced security automation platform that combines:

- 🔍 **OSINT & Reconnaissance** - Integrated with 14+ security tools
- 🌐 **Web Vulnerability Scanning** - XSS, SQLi, SSTI, and more
- 🎯 **Threat Intelligence** - Shodan, VirusTotal, SecurityTrails
- 🖥️ **Beautiful Web UI** - Vaporwave-themed dashboard
- 📜 **Lua Scripting** - Powerful, chainable API for custom automation

Write security scripts in just a few lines of code:

```lua
-- OSINT reconnaissance
local bbot = BBOT()
local results = bbot:subdomain_enum("example.com")

-- Vulnerability scanning
local nuclei = Nuclei()
nuclei:scan({ target = "https://example.com", severity = "critical,high" })

-- Secret detection
local gitleaks = Gitleaks()
gitleaks:scan({ path = "./repo", report = "secrets.json" })
```

---

## ⚡ Quick Install

### Linux / macOS

```bash
curl -sSL https://raw.githubusercontent.com/BugBlocker/lotus/master/install.sh | bash
```

### Windows (PowerShell as Admin)

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/BugBlocker/lotus/master/install.ps1'))
```

### Using Cargo

```bash
cargo install --git https://github.com/BugBlocker/lotus
```

<details>
<summary><b>📦 Platform-Specific Instructions</b></summary>

#### Ubuntu / Debian
```bash
sudo apt update
sudo apt install -y build-essential libssl-dev pkg-config libluajit-5.1-dev git curl
cargo install --git https://github.com/BugBlocker/lotus
```

#### Fedora / RHEL
```bash
sudo dnf install -y gcc openssl-devel pkgconfig luajit-devel git curl
cargo install --git https://github.com/BugBlocker/lotus
```

#### Arch Linux
```bash
sudo pacman -S base-devel openssl pkgconf luajit git curl
cargo install --git https://github.com/BugBlocker/lotus
```

#### macOS
```bash
brew install openssl luajit pkg-config
cargo install --git https://github.com/BugBlocker/lotus
```

#### Windows (Native)
```powershell
# Install Chocolatey, then:
choco install git rustup visualstudio2022buildtools -y
rustup-init -y
git clone https://github.com/BugBlocker/lotus.git
cd lotus
cargo build --release --features vendored
```

#### Docker
```bash
docker build -t lotus-osint .
docker run -it lotus-osint scan --help
```

</details>

See [INSTALL.md](INSTALL.md) for detailed installation instructions.

### ☁️ One-Click Cloud Deploy

| Platform | Deploy | Free Tier |
|----------|--------|-----------|
| **Railway** | [![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/template/lotus) | $5/mo credit |
| **Render** | [![Deploy to Render](https://render.com/images/deploy-to-render-button.svg)](https://render.com/deploy?repo=https://github.com/BugBlocker/lotus) | 750 hrs/mo |
| **GitHub Codespaces** | [![Open in Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/BugBlocker/lotus) | 60 hrs/mo |

<details>
<summary><b>🚀 Manual Cloud Deployment</b></summary>

#### Railway (Fastest)
```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and deploy
railway login
railway init
railway up
```

#### Render
1. Go to [render.com](https://render.com)
2. New → Web Service → Connect GitHub repo
3. It auto-detects `render.yaml` - just click Deploy!

#### Fly.io
```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Deploy
flyctl auth login
flyctl launch --copy-config
flyctl secrets set SHODAN_API_KEY=xxx VIRUSTOTAL_API_KEY=xxx
flyctl deploy
```

#### GitHub Codespaces (Instant Dev Environment)
1. Click "Code" → "Codespaces" → "Create codespace"
2. Wait ~2 min for setup
3. Run `lotus serve` - UI opens automatically!

</details>

---

## ✨ Features

### 🔍 Integrated OSINT Tools

| Tool | Category | Description |
|------|----------|-------------|
| **BBOT** | OSINT | Recursive subdomain enumeration |
| **Amass** | Subdomain | OWASP subdomain discovery |
| **Nuclei** | Vulnerability | Template-based scanning |
| **Shodan** | Intelligence | Internet device search |
| **theHarvester** | OSINT | Email & subdomain harvesting |
| **FinalRecon** | Web Recon | Full web reconnaissance |
| **SpiderFoot** | OSINT | Automated intelligence |
| **Gitleaks** | Secrets | Git secret detection |
| **TruffleHog** | Secrets | Credential scanning |
| **Subfinder** | Subdomain | Fast subdomain discovery |
| **httpx** | Probing | HTTP toolkit |
| **DNSMonster** | DNS | DNS traffic analysis |
| **RITA** | Threat | C2 beacon detection |
| **Zeek** | Network | Network analysis |

### 🔗 Chainable Lua API

```lua
-- String operations
str("  HELLO  "):trim():lower():replace("hello", "world"):value()
-- → "world"

-- HTML parsing with CSS selectors
html(body):select("a.external"):each(function(el)
    println(el:attr("href"):value())
end)

-- JSON navigation
json(resp.body):get("data.users.0.name"):value()

-- Built-in encoding
str(payload):url_encode():base64_encode():value()
```

### 🔐 Secrets Management

```lua
-- Load API keys automatically
local secrets = Secrets()
secrets:load_file("~/.lotus_secrets.json")

-- Use in your scripts
local shodan = Shodan()
shodan:init(secrets:shodan_key())
local results = shodan:search("apache")
```

Environment variables or config file:
```bash
export SHODAN_API_KEY="your-key"
export VIRUSTOTAL_API_KEY="your-key"
```

---

## 🖥️ Web UI

Launch the vaporwave-themed web interface:

```bash
lotus serve
# Open http://localhost:8080
```

```bash
# Custom port
lotus serve --port 3000

# Open browser automatically
lotus serve --open

# Bind to all interfaces
lotus serve --host 0.0.0.0
```

**Features:**
- 📊 Dashboard with scan statistics
- 🚀 Launch and monitor scans
- 🔑 Manage API keys
- 📁 View scan results
- 🛠️ Browse available tools
- 📜 Script library

---

## 🚀 Usage

### Basic Scanning

```bash
# Scan with a script
echo "example.com" | lotus scan examples/bbot_scanner.lua

# Scan with output file
echo "example.com" | lotus scan examples/threat_intel_scanner.lua -o results.json

# Multiple targets
cat targets.txt | lotus scan examples/amass_osint.lua -w 20

# With proxy (for Burp/ZAP)
echo "https://target.com" | lotus scan script.lua -p http://127.0.0.1:8080
```

### Create New Scripts

```bash
# Create host scanner template (OSINT)
lotus new --type 1 -o my_osint.lua

# Create URL scanner template (Web vuln)
lotus new --type 2 -o my_scanner.lua
```

### Example Scripts

<details>
<summary><b>BBOT Subdomain Enumeration</b></summary>

```lua
SCAN_TYPE = 1  -- Host scanning

function main(host)
    local bbot = BBOT()
    
    -- Run subdomain enumeration
    local result = bbot:subdomain_enum(host)
    
    if result:success() then
        local data = result:json()
        for _, finding in ipairs(data) do
            println("[+] " .. finding.host)
        end
    end
end
```
</details>

<details>
<summary><b>Threat Intelligence Scanner</b></summary>

```lua
SCAN_TYPE = 1

function main(host)
    local secrets = Secrets()
    
    -- Shodan lookup
    local shodan = Shodan()
    shodan:init(secrets:shodan_key())
    local info = shodan:host(host)
    
    -- Nuclei vulnerability scan
    local nuclei = Nuclei()
    local vulns = nuclei:scan({
        target = host,
        severity = "critical,high"
    })
    
    -- Report findings
    if vulns:success() then
        report_vuln({
            name = "Vulnerabilities Found",
            host = host,
            data = vulns:json()
        })
    end
end
```
</details>

<details>
<summary><b>Secret Detection</b></summary>

```lua
SCAN_TYPE = 1

function main(repo_url)
    -- Gitleaks scan
    local gitleaks = Gitleaks()
    local result = gitleaks:scan({
        repo_url = repo_url,
        report = "gitleaks_report.json"
    })
    
    -- TruffleHog scan
    local trufflehog = TruffleHog()
    local secrets = trufflehog:github(repo_url)
    
    if secrets:success() then
        for _, secret in ipairs(secrets:lines()) do
            println("[SECRET] " .. secret)
        end
    end
end
```
</details>

---

## 🔑 API Keys Setup

### Option 1: Environment Variables

```bash
# Add to ~/.bashrc or ~/.zshrc
export SHODAN_API_KEY="your-key"
export VIRUSTOTAL_API_KEY="your-key"
export GITHUB_TOKEN="your-token"
export SECURITYTRAILS_API_KEY="your-key"
```

### Option 2: Config File

Create `~/.lotus_secrets.json`:
```json
{
    "shodan": "your-shodan-key",
    "virustotal": "your-virustotal-key",
    "github": "your-github-token",
    "securitytrails": "your-securitytrails-key"
}
```

### Where to Get API Keys

| Service | Free Tier | URL |
|---------|-----------|-----|
| Shodan | ✅ Limited | https://account.shodan.io |
| VirusTotal | ✅ | https://virustotal.com/gui/my-apikey |
| SecurityTrails | ✅ Limited | https://securitytrails.com/app/signup |
| GitHub | ✅ | https://github.com/settings/tokens |
| Hunter.io | ✅ Limited | https://hunter.io/api-keys |

---

## 🔄 Sample Workflows

### GitHub Actions: Automated Security Scan

```yaml
# .github/workflows/security-scan.yml
name: Security Scan

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * *'  # Daily at midnight

jobs:
  scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Lotus
        run: |
          curl -sSL https://raw.githubusercontent.com/BugBlocker/lotus/master/install.sh | bash
          
      - name: Run OSINT Scan
        env:
          SHODAN_API_KEY: ${{ secrets.SHODAN_API_KEY }}
          VIRUSTOTAL_API_KEY: ${{ secrets.VIRUSTOTAL_API_KEY }}
        run: |
          echo "${{ vars.TARGET_DOMAIN }}" | lotus scan examples/threat_intel_scanner.lua -o results.json
          
      - name: Upload Results
        uses: actions/upload-artifact@v4
        with:
          name: scan-results
          path: results.json
```

### GitHub Actions: Secret Detection on PRs

```yaml
# .github/workflows/secret-scan.yml
name: Secret Detection

on:
  pull_request:
    branches: [main]

jobs:
  secrets:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
          
      - name: Install Lotus
        run: curl -sSL https://raw.githubusercontent.com/BugBlocker/lotus/master/install.sh | bash
        
      - name: Install Gitleaks
        run: go install github.com/gitleaks/gitleaks/v8@latest
        
      - name: Scan for Secrets
        run: |
          cat << 'EOF' > secret_scan.lua
          SCAN_TYPE = 4
          function main(input)
              local gitleaks = Gitleaks()
              local result = gitleaks:scan({ path = ".", report = "secrets.json" })
              if result:exit_code() ~= 0 then
                  println("[!] Secrets detected!")
                  os.exit(1)
              end
          end
          EOF
          echo "." | lotus scan secret_scan.lua
```

### GitHub Actions: Subdomain Monitoring

```yaml
# .github/workflows/subdomain-monitor.yml
name: Subdomain Monitor

on:
  schedule:
    - cron: '0 */6 * * *'  # Every 6 hours
  workflow_dispatch:

jobs:
  monitor:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup
        run: |
          curl -sSL https://raw.githubusercontent.com/BugBlocker/lotus/master/install.sh | bash
          pip install bbot
          
      - name: Enumerate Subdomains
        run: |
          echo "${{ vars.TARGET_DOMAIN }}" | lotus scan examples/bbot_scanner.lua -o new_subs.json
          
      - name: Compare with Previous
        run: |
          if [ -f previous_subs.json ]; then
            diff previous_subs.json new_subs.json > changes.txt || true
            if [ -s changes.txt ]; then
              echo "New subdomains detected!"
              cat changes.txt
            fi
          fi
          cp new_subs.json previous_subs.json
          
      - name: Commit Changes
        run: |
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add previous_subs.json
          git diff --cached --quiet || git commit -m "Update subdomain list"
          git push
```

### GitLab CI: Vulnerability Scan

```yaml
# .gitlab-ci.yml
stages:
  - scan
  - report

vulnerability_scan:
  stage: scan
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y libssl-dev pkg-config libluajit-5.1-dev
    - cargo install --git https://github.com/BugBlocker/lotus
    - go install github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest
  script:
    - echo "$TARGET_URL" | lotus scan examples/nuclei_scanner.lua -o vulns.json
  artifacts:
    paths:
      - vulns.json
    expire_in: 1 week
  rules:
    - if: $CI_PIPELINE_SOURCE == "schedule"
```

### Docker Compose: Continuous Monitoring Stack

```yaml
# docker-compose.monitoring.yml
version: '3.8'

services:
  lotus:
    build: .
    environment:
      - SHODAN_API_KEY=${SHODAN_API_KEY}
      - VIRUSTOTAL_API_KEY=${VIRUSTOTAL_API_KEY}
    volumes:
      - ./scripts:/app/scripts
      - ./results:/app/results
      - ./targets.txt:/app/targets.txt
    command: >
      sh -c "while true; do
        cat /app/targets.txt | lotus scan /app/scripts/monitor.lua -o /app/results/scan_$(date +%Y%m%d_%H%M%S).json;
        sleep 3600;
      done"

  lotus-ui:
    build: .
    ports:
      - "8080:8080"
    environment:
      - SHODAN_API_KEY=${SHODAN_API_KEY}
    command: serve --host 0.0.0.0 --port 8080

  results-server:
    image: nginx:alpine
    volumes:
      - ./results:/usr/share/nginx/html:ro
    ports:
      - "8081:80"
```

### Bash: Quick Recon Workflow

```bash
#!/bin/bash
# recon.sh - Quick reconnaissance workflow

TARGET="$1"
OUTPUT_DIR="./recon_${TARGET}_$(date +%Y%m%d)"

mkdir -p "$OUTPUT_DIR"

echo "[*] Starting recon for $TARGET"

# Step 1: Subdomain enumeration
echo "[1/4] Subdomain enumeration..."
echo "$TARGET" | lotus scan examples/bbot_scanner.lua -o "$OUTPUT_DIR/subdomains.json"

# Step 2: HTTP probing
echo "[2/4] HTTP probing..."
cat "$OUTPUT_DIR/subdomains.json" | jq -r '.host' | httpx -silent -o "$OUTPUT_DIR/live_hosts.txt"

# Step 3: Vulnerability scan
echo "[3/4] Vulnerability scanning..."
cat "$OUTPUT_DIR/live_hosts.txt" | lotus scan examples/nuclei_scanner.lua -o "$OUTPUT_DIR/vulns.json" -w 10

# Step 4: Threat intelligence
echo "[4/4] Threat intelligence..."
echo "$TARGET" | lotus scan examples/threat_intel_scanner.lua -o "$OUTPUT_DIR/intel.json"

echo "[+] Recon complete! Results in $OUTPUT_DIR"
```

### PowerShell: Windows Automation

```powershell
# recon.ps1 - Windows reconnaissance workflow

param(
    [Parameter(Mandatory=$true)]
    [string]$Target
)

$OutputDir = ".\recon_${Target}_$(Get-Date -Format 'yyyyMMdd')"
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

Write-Host "[*] Starting recon for $Target" -ForegroundColor Cyan

# Subdomain enumeration
Write-Host "[1/3] Subdomain enumeration..." -ForegroundColor Yellow
$Target | lotus scan examples/bbot_scanner.lua -o "$OutputDir\subdomains.json"

# Vulnerability scan
Write-Host "[2/3] Vulnerability scanning..." -ForegroundColor Yellow
$Target | lotus scan examples/nuclei_scanner.lua -o "$OutputDir\vulns.json"

# Generate report
Write-Host "[3/3] Generating report..." -ForegroundColor Yellow
$results = Get-Content "$OutputDir\vulns.json" | ConvertFrom-Json
$report = @"
# Recon Report: $Target
Generated: $(Get-Date)

## Summary
- Subdomains found: $((Get-Content "$OutputDir\subdomains.json" | ConvertFrom-Json).Count)
- Vulnerabilities: $($results.Count)

## Critical Findings
$($results | Where-Object { $_.severity -eq "critical" } | ForEach-Object { "- $($_.name): $($_.host)" })
"@
$report | Out-File "$OutputDir\report.md"

Write-Host "[+] Recon complete! Results in $OutputDir" -ForegroundColor Green
```

### Cron Job: Scheduled Monitoring

```bash
# Add to crontab: crontab -e

# Daily full scan at 2 AM
0 2 * * * /usr/local/bin/lotus scan /opt/lotus/scripts/daily_scan.lua -o /var/log/lotus/daily_$(date +\%Y\%m\%d).json 2>&1

# Hourly quick check
0 * * * * echo "critical-asset.com" | /usr/local/bin/lotus scan /opt/lotus/scripts/quick_check.lua >> /var/log/lotus/hourly.log 2>&1

# Weekly comprehensive report
0 3 * * 0 /opt/lotus/scripts/weekly_report.sh >> /var/log/lotus/weekly.log 2>&1
```

### Lua: Custom Multi-Stage Workflow

```lua
-- workflows/full_recon.lua
SCAN_TYPE = 1

function main(target)
    local results = {
        target = target,
        timestamp = timestamp(),
        stages = {}
    }
    
    -- Stage 1: Passive reconnaissance
    println("[Stage 1] Passive recon...")
    local harvester = TheHarvester()
    local passive = harvester:search({
        domain = target,
        source = "all",
        limit = 500
    })
    results.stages.passive = passive:json()
    
    -- Stage 2: Active subdomain enumeration
    println("[Stage 2] Subdomain enumeration...")
    local bbot = BBOT()
    local subs = bbot:subdomain_enum(target)
    results.stages.subdomains = subs:json()
    
    -- Stage 3: Service detection
    println("[Stage 3] Service detection...")
    local shodan = Shodan()
    local secrets = Secrets()
    shodan:init(secrets:shodan_key())
    
    for _, sub in ipairs(results.stages.subdomains or {}) do
        local info = shodan:host(sub.host)
        if info:success() then
            sub.services = info:json()
        end
    end
    
    -- Stage 4: Vulnerability assessment
    println("[Stage 4] Vulnerability scan...")
    local nuclei = Nuclei()
    local vulns = nuclei:scan({
        target = target,
        severity = "critical,high,medium"
    })
    results.stages.vulnerabilities = vulns:json()
    
    -- Stage 5: Secret detection (if git repo)
    println("[Stage 5] Secret detection...")
    local gitleaks = Gitleaks()
    local github_url = "https://github.com/" .. target:gsub("%.com$", "")
    local secrets_found = gitleaks:scan({ repo_url = github_url })
    results.stages.secrets = secrets_found:json()
    
    -- Generate report
    println("[+] Recon complete!")
    println("    Subdomains: " .. #(results.stages.subdomains or {}))
    println("    Vulnerabilities: " .. #(results.stages.vulnerabilities or {}))
    
    -- Output JSON results
    write_json("recon_" .. target .. ".json", results)
    
    return results
end
```

---

## 📖 Documentation

| Document | Description |
|----------|-------------|
| [INSTALL.md](INSTALL.md) | Detailed installation guide |
| [docs/lua_scripting.md](docs/lua_scripting.md) | Lua API reference |
| [docs/osint_integration.md](docs/osint_integration.md) | OSINT tools guide |
| [examples/](examples/) | Example scripts |

**Online Docs:** https://lotus.knas.me

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. 🐛 Report bugs via [GitHub Issues](https://github.com/BugBlocker/lotus/issues)
2. 💡 Suggest features or improvements
3. 🔧 Submit pull requests
4. 📜 Write example scripts
5. 📖 Improve documentation

Join our [Discord](https://discord.gg/nBYDPTzjSq) community!

---

## 📜 License

Lotus is released under the [GPL v2 License](LICENSE).

---

<p align="center">
<b>🪷 Happy Hunting!</b>
</p>
