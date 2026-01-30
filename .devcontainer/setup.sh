#!/bin/bash
# GitHub Codespaces / DevContainer Setup

set -e

echo "🪷 Setting up Lotus OSINT Platform..."

# Install system dependencies
sudo apt-get update
sudo apt-get install -y libssl-dev pkg-config libluajit-5.1-dev

# Install Python tools
pip install --user bbot theHarvester shodan

# Install Go tools
go install github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest
go install github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest
go install github.com/projectdiscovery/httpx/cmd/httpx@latest
go install github.com/gitleaks/gitleaks/v8@latest

# Build Lotus
cargo build --release

# Create symlink
sudo ln -sf $(pwd)/target/release/lotus /usr/local/bin/lotus

echo ""
echo "✅ Setup complete!"
echo ""
echo "Quick start:"
echo "  lotus serve              # Start web UI at http://localhost:8080"
echo "  lotus scan --help        # See scan options"
echo ""
