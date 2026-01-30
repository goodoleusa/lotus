# ⚔️ Atropos OSINT Platform - Installation Guide

```
██╗      ██████╗ ████████╗██╗   ██╗███████╗
██║     ██╔═══██╗╚══██╔══╝██║   ██║██╔════╝
██║     ██║   ██║   ██║   ██║   ██║███████╗
██║     ██║   ██║   ██║   ██║   ██║╚════██║
███████╗╚██████╔╝   ██║   ╚██████╔╝███████║
╚══════╝ ╚═════╝    ╚═╝    ╚═════╝ ╚══════╝
        OSINT & THREAT INTEL PLATFORM
```

## 📋 Table of Contents

- [Quick Install (5 minutes)](#-quick-install)
- [Full Installation](#-full-installation)
- [Docker Installation](#-docker-installation)
- [Installing OSINT Tools](#-installing-osint-tools)
- [API Keys Setup](#-api-keys-setup)
- [Running the Web UI](#-running-the-web-ui)
- [Troubleshooting](#-troubleshooting)

---

## ⚡ Quick Install

### One-Line Install (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/BugBlocker/atropos/master/install.sh | bash
```

### One-Line Install (Windows PowerShell)

Run as Administrator:
```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/BugBlocker/atropos/master/install.ps1'))
```

### Manual Quick Install

```bash
# 1. Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install system dependencies
# Ubuntu/Debian:
sudo apt install -y libssl-dev pkg-config libluajit-5.1-dev

# macOS:
brew install openssl luajit pkg-config

# 3. Install Atropos
cargo install --git https://github.com/BugBlocker/atropos

# 4. Verify installation
atropos --help
```

---

## 📦 Full Installation

### Prerequisites

| Requirement | Version | Check Command |
|-------------|---------|---------------|
| Rust | 1.70+ | `rustc --version` |
| Cargo | 1.70+ | `cargo --version` |
| OpenSSL | 1.1+ | `openssl version` |
| LuaJIT | 2.0+ | `luajit -v` |
| Git | 2.0+ | `git --version` |

### Step 1: Install Rust

```bash
# Install Rust via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH
source $HOME/.cargo/env

# Verify
rustc --version
cargo --version
```

### Step 2: Install System Dependencies

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libluajit-5.1-dev \
    luajit \
    git \
    curl
```

#### Fedora/RHEL/CentOS
```bash
sudo dnf install -y \
    gcc \
    openssl-devel \
    pkgconfig \
    luajit-devel \
    git \
    curl
```

#### Arch Linux
```bash
sudo pacman -S \
    base-devel \
    openssl \
    pkgconf \
    luajit \
    git \
    curl
```

#### macOS
```bash
# Install Homebrew if not installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install openssl luajit pkg-config
```

#### Windows (Native)

**Option A: Using Chocolatey (Recommended)**
```powershell
# Install Chocolatey (run as Administrator)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install dependencies
choco install git rustup visualstudio2022buildtools -y

# Install Rust
rustup-init -y

# Restart terminal, then build Atropos
git clone https://github.com/BugBlocker/atropos.git
cd atropos
cargo build --release --features vendored
```

**Option B: Using winget**
```powershell
# Install dependencies
winget install Git.Git
winget install Rustlang.Rustup
winget install Microsoft.VisualStudio.2022.BuildTools

# Configure VS Build Tools (run VS Installer and add C++ workload)

# Restart terminal, then build Atropos
git clone https://github.com/BugBlocker/atropos.git
cd atropos
cargo build --release --features vendored
```

**Option C: Using WSL2 (Linux environment on Windows)**
```powershell
# Install WSL2
wsl --install

# Then follow Ubuntu instructions inside WSL2
```

**Windows Environment Variables:**
```powershell
# Add to PowerShell profile or set permanently
$env:SHODAN_API_KEY = "your-key"
$env:VIRUSTOTAL_API_KEY = "your-key"

# Or set permanently via System Properties > Environment Variables
[Environment]::SetEnvironmentVariable("SHODAN_API_KEY", "your-key", "User")
```

### Step 3: Clone and Build

```bash
# Clone the repository
git clone https://github.com/BugBlocker/atropos.git
cd atropos

# Build release version
cargo build --release

# Install to PATH
cargo install --path .

# Or copy binary manually
sudo cp target/release/atropos /usr/local/bin/
```

### Step 4: Verify Installation

```bash
# Check version
atropos --version

# View help
atropos --help

# Run a test scan
echo "example.com" | atropos scan examples/bbot_scanner.lua --help
```

---

## 🐳 Docker Installation

### Using Docker

```bash
# Build the image
docker build -t atropos-osint .

# Run interactive
docker run -it --rm atropos-osint

# Run with mounted scripts
docker run -it --rm \
    -v $(pwd)/scripts:/scripts \
    -v $(pwd)/results:/results \
    atropos-osint scan /scripts/scanner.lua -o /results/output.json
```

### Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'
services:
  atropos:
    build: .
    volumes:
      - ./scripts:/app/scripts
      - ./results:/app/results
      - ./secrets:/app/secrets
    environment:
      - SHODAN_API_KEY=${SHODAN_API_KEY}
      - VIRUSTOTAL_API_KEY=${VIRUSTOTAL_API_KEY}
    ports:
      - "8080:8080"  # Web UI
```

Run:
```bash
docker-compose up -d
```

### Dockerfile

```dockerfile
FROM rust:1.75-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    libluajit-5.1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libssl3 \
    libluajit-5.1-2 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/atropos /usr/local/bin/
COPY --from=builder /app/examples /app/examples

WORKDIR /app
ENTRYPOINT ["atropos"]
```

---

## 🛠️ Installing OSINT Tools

Atropos integrates with many external tools. Install the ones you need:

### Core Tools (Recommended)

```bash
# BBOT - Recursive OSINT automation
pip install bbot

# Amass - Subdomain enumeration
go install -v github.com/owasp-amass/amass/v4/...@master

# Nuclei - Vulnerability scanning
go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest

# Subfinder - Fast subdomain discovery
go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest

# httpx - HTTP probing
go install -v github.com/projectdiscovery/httpx/cmd/httpx@latest
```

### Additional Tools

```bash
# theHarvester - Email/subdomain harvesting
pip install theHarvester

# Shodan CLI
pip install shodan

# FinalRecon - Web reconnaissance
pip install finalrecon

# SpiderFoot
pip install spiderfoot

# Gitleaks - Secret scanning
go install github.com/gitleaks/gitleaks/v8@latest

# TruffleHog - Secret detection
pip install trufflehog
```

### All-in-One Script

```bash
#!/bin/bash
# install_tools.sh - Install all OSINT tools

echo "🛠️ Installing OSINT Tools..."

# Python tools
pip install --user bbot theHarvester shodan spiderfoot finalrecon trufflehog

# Go tools (requires Go 1.21+)
go install -v github.com/owasp-amass/amass/v4/...@master
go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest
go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest
go install -v github.com/projectdiscovery/httpx/cmd/httpx@latest
go install github.com/gitleaks/gitleaks/v8@latest

echo "✅ Installation complete!"
echo "Run 'atropos scan --help' to get started"
```

---

## 🔑 API Keys Setup

### Option 1: Environment Variables (Recommended)

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
# OSINT API Keys
export SHODAN_API_KEY="your-shodan-key"
export VIRUSTOTAL_API_KEY="your-virustotal-key"
export SECURITYTRAILS_API_KEY="your-securitytrails-key"
export CENSYS_API_ID="your-censys-id"
export CENSYS_API_SECRET="your-censys-secret"
export HUNTER_API_KEY="your-hunter-key"
export GITHUB_TOKEN="your-github-token"
export ABUSEIPDB_API_KEY="your-abuseipdb-key"
export OTX_API_KEY="your-alienvault-key"
```

Then reload:
```bash
source ~/.bashrc
```

### Option 2: Secrets File

Create `~/.atropos_secrets.json`:

```json
{
    "shodan": "your-shodan-key",
    "virustotal": "your-virustotal-key",
    "securitytrails": "your-securitytrails-key",
    "censys_id": "your-censys-id",
    "censys_secret": "your-censys-secret",
    "hunter": "your-hunter-key",
    "github": "your-github-token",
    "abuseipdb": "your-abuseipdb-key",
    "otx": "your-alienvault-key"
}
```

Set permissions:
```bash
chmod 600 ~/.atropos_secrets.json
```

### Option 3: .env File

Create `.env` in your project directory:

```env
SHODAN_API_KEY=your-shodan-key
VIRUSTOTAL_API_KEY=your-virustotal-key
GITHUB_TOKEN=your-github-token
```

### Where to Get API Keys

| Service | Free Tier | Get Key |
|---------|-----------|---------|
| Shodan | Yes (limited) | https://account.shodan.io |
| VirusTotal | Yes | https://www.virustotal.com/gui/my-apikey |
| SecurityTrails | Yes (limited) | https://securitytrails.com/app/signup |
| Censys | Yes (limited) | https://search.censys.io/account/api |
| Hunter.io | Yes (limited) | https://hunter.io/api-keys |
| GitHub | Yes | https://github.com/settings/tokens |
| AbuseIPDB | Yes | https://www.abuseipdb.com/account/api |
| AlienVault OTX | Yes | https://otx.alienvault.com/api |

---

## 🖥️ Running the Web UI

### Start the Web Server

```bash
# Start with default port (8080)
atropos serve

# Or specify a port
atropos serve --port 3000
```

### Access the UI

Open your browser and navigate to:
```
http://localhost:8080
```

### Features

- **Dashboard**: Overview of scans and statistics
- **New Scan**: Configure and launch scans
- **Results**: View scan findings
- **API Keys**: Manage your API credentials
- **Tools**: See available integrations
- **Scripts**: Browse available scan scripts

---

## 🔧 Troubleshooting

### Common Issues

#### "LuaJIT not found"

**Linux (Ubuntu/Debian):**
```bash
sudo apt install libluajit-5.1-dev
export PKG_CONFIG_PATH="/usr/lib/pkgconfig:/usr/local/lib/pkgconfig"
```

**macOS:**
```bash
brew install luajit
```

**Windows:**
```powershell
# Use vendored feature to bundle Lua
cargo build --release --features vendored
```

#### "OpenSSL not found"

**Linux (Ubuntu/Debian):**
```bash
sudo apt install libssl-dev
```

**macOS:**
```bash
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
```

**Windows:**
```powershell
# OpenSSL is typically bundled with Rust on Windows
# If issues persist, install via vcpkg:
vcpkg install openssl:x64-windows
$env:OPENSSL_DIR = "C:\vcpkg\installed\x64-windows"
```

#### "LINK : fatal error LNK1181: cannot open input file" (Windows)
```powershell
# Install Visual Studio Build Tools with C++ workload
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"

# Or via VS Installer: Add "Desktop development with C++"
```

#### "Permission denied" on install
```bash
# Linux/macOS: Use --path instead of system install
cargo install --path . --root ~/.local
export PATH="$HOME/.local/bin:$PATH"
```

```powershell
# Windows: Install to user directory
cargo install --path . --root $env:USERPROFILE\.cargo
```

#### "Tool not found" (e.g., amass, nuclei)

**Linux/macOS:**
```bash
export PATH="$HOME/go/bin:$PATH"
echo 'export PATH="$HOME/go/bin:$PATH"' >> ~/.bashrc
```

**Windows:**
```powershell
# Add Go bin to PATH
$env:Path += ";$env:USERPROFILE\go\bin"
# Make permanent
[Environment]::SetEnvironmentVariable("Path", $env:Path, "User")
```

#### Scan hangs or times out
```bash
# Increase timeout
atropos scan script.lua -t 60

# Check network connectivity
curl -I https://example.com

# Run with verbose mode
atropos scan script.lua -v --log debug.log
```

#### Windows Defender blocks execution
```powershell
# Add exclusion for Atropos directory
Add-MpPreference -ExclusionPath "$env:USERPROFILE\.atropos"
Add-MpPreference -ExclusionPath "$env:USERPROFILE\.cargo\bin\atropos.exe"
```

#### PowerShell execution policy error
```powershell
# Allow script execution for current session
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass

# Or permanently for current user
Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
```

### Getting Help

```bash
# View help
atropos --help
atropos scan --help

# Enable debug logging
atropos scan script.lua -v --log debug.log

# Check tool versions
atropos --version
amass -version
nuclei -version
```

### Community Support

- **GitHub Issues**: https://github.com/BugBlocker/atropos/issues
- **Discord**: https://discord.gg/nBYDPTzjSq
- **Documentation**: https://atropos.knas.me

---

## 📝 Quick Reference

```bash
# Basic scan
echo "example.com" | atropos scan examples/bbot_scanner.lua

# Scan with output file
echo "example.com" | atropos scan examples/threat_intel_scanner.lua -o results.json

# Scan multiple targets
cat targets.txt | atropos scan examples/amass_osint.lua -w 20

# Scan with proxy
echo "example.com" | atropos scan script.lua -p http://127.0.0.1:8080

# Pass environment variables to script
echo "example.com" | atropos scan script.lua --env-vars '{"DEBUG":"true"}'

# Start Web UI
atropos serve --port 8080
```

---

**⚔️ Happy Hunting!**
