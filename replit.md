# Atropos OSINT Platform

## Overview

Atropos is an advanced security automation platform built in Rust that combines OSINT (Open Source Intelligence) reconnaissance, web vulnerability scanning, and threat intelligence capabilities. Named after the Greek Fate who "cuts the thread of life," the platform provides a unified interface for security professionals to automate reconnaissance and vulnerability detection workflows.

The platform features:
- Integration with 14+ security tools (BBOT, Nuclei, Shodan, VirusTotal, etc.)
- Lua scripting engine with a chainable fluent API for custom automation
- Vaporwave-themed web UI dashboard
- CLI interface for scanning operations
- Centralized secrets management for API keys

## User Preferences

Preferred communication style: Simple, everyday language.

## System Architecture

### Core Runtime
- **Language**: Rust for the core platform, providing memory safety and performance
- **Scripting Engine**: LuaJIT integration for user-defined security scripts
- **Binary Name**: `atropos` (previously called "lotus" in some documentation)

### Web Interface
- **Server**: Built-in HTTP server (`atropos serve`) on port 8080
- **Frontend**: Single-page application with vanilla JavaScript
- **Styling**: Custom CSS with vaporwave aesthetic, using Orbitron and VT323 fonts
- **API**: RESTful endpoints under `/api/` prefix with health check at `/api/health`

### Lua Scripting System
Scripts define a `SCAN_TYPE` (1-4) to specify input handling:
1. Hosts only
2. Full URLs with parameters
3. URL paths without parameters  
4. Custom input handler

The fluent API provides chainable utilities:
- `str()` - String manipulation and encoding
- `html()` - CSS selector-based HTML parsing
- `json()` - JSON navigation with dot notation
- `tbl()` - Functional array operations (map, filter, etc.)

### Tool Integrations
External security tools are wrapped with Lua APIs:
- **Reconnaissance**: BBOT, Amass, theHarvester, FinalRecon
- **Scanning**: Nuclei, Gitleaks
- **Threat Intel**: Shodan, VirusTotal, SecurityTrails
- **Network**: Zeek, RITA, DNSMonster

### Secrets Management
API keys load from (in priority order):
1. Environment variables (e.g., `SHODAN_API_KEY`)
2. JSON config file (`.atropos_secrets.json`)
3. KEY=VALUE file (`.atropos_secrets`)

## External Dependencies

### System Requirements
- **OpenSSL**: TLS/crypto functionality (`libssl-dev`)
- **LuaJIT**: Scripting engine (`libluajit-5.1-dev`)
- **pkg-config**: Build configuration

### Rust Crates (Notable)
- `tokio` - Async runtime with full features
- `serde` / `serde_derive` - Serialization
- `ring` - Cryptography
- `http` - HTTP types

### Deployment Options
- **Railway**: Dockerfile-based deployment with health checks
- **Docker**: Containerized deployment available
- **Termux**: Android support via proot-distro Ubuntu
- **Dev Containers**: VS Code devcontainer with Rust, Go, and Python

### External Tool Dependencies
The platform orchestrates external security tools that must be installed separately:
- Go-based: Nuclei, BBOT, Amass
- Python-based: theHarvester, SpiderFoot
- API services: Shodan, VirusTotal, SecurityTrails, AbuseIPDB