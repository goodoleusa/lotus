# 📱 Atropos on Android (Termux + proot-distro Ubuntu)

Complete setup guide for running Atropos on Android without root access.

## Prerequisites

- Android phone/tablet
- [Termux from F-Droid](https://f-droid.org/packages/com.termux/) (NOT Play Store version)
- ~2GB free storage
- Patience (compiling takes time on mobile)

---

## 🚀 Quick Setup (Copy-Paste Ready)

### Step 1: Termux Setup

```bash
# Update Termux packages
pkg update && pkg upgrade -y

# Install proot-distro
pkg install proot-distro git -y

# Install Ubuntu
proot-distro install ubuntu

# Login to Ubuntu
proot-distro login ubuntu
```

### Step 2: Inside Ubuntu (proot)

```bash
# Update packages (no sudo needed in proot!)
apt update && apt upgrade -y

# Install dependencies
apt install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libluajit-5.1-dev \
    luajit \
    git \
    curl \
    python3 \
    python3-pip \
    golang-go \
    ca-certificates

# Install Rust (user install, no root)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Verify
rustc --version
```

### Step 3: Install Atropos

```bash
# Clone and build
git clone https://github.com/BugBlocker/atropos.git
cd atropos
cargo build --release

# Add to PATH
mkdir -p ~/.local/bin
cp target/release/atropos ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify
atropos --version
```

### Step 4: Install OSINT Tools (Optional)

```bash
# Python tools (user install)
pip3 install --user bbot theHarvester shodan

# Go tools (user install)
export GOPATH="$HOME/go"
export PATH="$PATH:$GOPATH/bin"
echo 'export GOPATH="$HOME/go"' >> ~/.bashrc
echo 'export PATH="$PATH:$GOPATH/bin"' >> ~/.bashrc

go install github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest
go install github.com/projectdiscovery/httpx/cmd/httpx@latest
go install github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest
```

---

## 📁 Git Setup for Termux (Better than MGit/GitNex)

### Option 1: Pure Terminal Git (Recommended)

```bash
# In Termux (not proot), install git
pkg install git openssh -y

# Configure git
git config --global user.name "Your Name"
git config --global user.email "your@email.com"

# Generate SSH key (no passphrase for convenience)
ssh-keygen -t ed25519 -C "termux@android" -N "" -f ~/.ssh/id_ed25519

# Show public key - add this to GitHub
cat ~/.ssh/id_ed25519.pub

# Test connection
ssh -T git@github.com
```

### Option 2: GitHub CLI (Easiest Auth)

```bash
# In Termux
pkg install gh -y

# Login (opens browser for OAuth)
gh auth login

# Now you can clone with HTTPS easily
gh repo clone username/repo
```

### Option 3: Git Credential Helper

```bash
# Store credentials (saves typing password)
git config --global credential.helper store

# First push/pull will ask for credentials, then saves them
# Use a Personal Access Token as password (not your GitHub password)
# Create token at: https://github.com/settings/tokens
```

---

## 🔧 Recommended Termux Setup

### Essential Packages

```bash
pkg install -y \
    git \
    gh \
    openssh \
    nano \
    vim \
    tmux \
    wget \
    curl \
    jq \
    ripgrep \
    fd \
    fzf \
    htop
```

### Nice Terminal Experience

```bash
# Better shell prompt
pkg install zsh -y
chsh -s zsh

# Oh My Zsh (optional but nice)
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Or just a simple .bashrc improvement
cat >> ~/.bashrc << 'EOF'
# Colors
PS1='\[\033[01;32m\]\u@termux\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '

# Aliases
alias ll='ls -la'
alias gs='git status'
alias gp='git pull'
alias gc='git commit'
alias gd='git diff'
alias ubuntu='proot-distro login ubuntu'
alias atropos-ui='proot-distro login ubuntu -- atropos serve'

# PATH
export PATH="$HOME/.local/bin:$HOME/go/bin:$PATH"
EOF
source ~/.bashrc
```

### Quick Access Script

Create `~/atropos-start.sh`:
```bash
cat > ~/atropos-start.sh << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash
echo "⚔️ Starting Atropos OSINT Platform..."
proot-distro login ubuntu -- bash -c "cd ~/atropos && atropos serve --host 0.0.0.0 --port 8080"
EOF
chmod +x ~/atropos-start.sh
```

Now just run: `~/atropos-start.sh`

---

## 🌐 Access Web UI on Phone

### From Same Device

1. Start Atropos: `atropos serve`
2. Open browser: `http://localhost:8080`

### From Another Device (LAN)

```bash
# Find your IP
ip addr | grep inet

# Start with host binding
atropos serve --host 0.0.0.0 --port 8080

# Access from other device: http://YOUR_PHONE_IP:8080
```

### With Termux:Widget (Home Screen Shortcut)

```bash
# Install widget addon from F-Droid
pkg install termux-widget

# Create shortcut script
mkdir -p ~/.shortcuts
cat > ~/.shortcuts/Atropos << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash
proot-distro login ubuntu -- atropos serve
EOF
chmod +x ~/.shortcuts/Atropos

# Add widget to home screen, select "Atropos"
```

---

## 📱 Git Workflow on Termux

### Daily Workflow

```bash
# Enter Ubuntu environment
proot-distro login ubuntu

# Navigate to project
cd ~/atropos

# Check status
git status

# Pull latest
git pull origin master

# Make changes...
nano src/something.rs

# Stage and commit
git add -A
git commit -m "Your message"

# Push
git push origin master
```

### Using GitHub CLI (Easiest)

```bash
# Clone a repo
gh repo clone owner/repo

# Create PR
gh pr create --title "Title" --body "Description"

# View PRs
gh pr list

# Check out PR locally
gh pr checkout 123

# Merge PR
gh pr merge 123
```

### Sync Fork

```bash
# Add upstream
git remote add upstream https://github.com/original/repo.git

# Fetch and merge
git fetch upstream
git merge upstream/master

# Push to your fork
git push origin master
```

---

## 🔋 Performance Tips

### Reduce Build Time

```bash
# Use fewer parallel jobs (saves memory)
export CARGO_BUILD_JOBS=2

# Or in ~/.cargo/config.toml
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[build]
jobs = 2

[net]
git-fetch-with-cli = true
EOF
```

### Keep Termux Awake

```bash
# Prevent Android from killing Termux
termux-wake-lock

# Release when done
termux-wake-unlock
```

### Use Swap (if builds fail from OOM)

```bash
# In Termux (not proot)
pkg install tsu -y

# Only if rooted - skip if not rooted
# Termux handles memory okay for most phones with 4GB+ RAM
```

---

## 🛠️ Troubleshooting

### "cargo build" kills due to memory

```bash
# Reduce parallel jobs
CARGO_BUILD_JOBS=1 cargo build --release

# Or build without optimizations first (faster, uses less memory)
cargo build
# Then release when you need it
```

### Git SSL certificate errors

```bash
# Update certificates
apt install ca-certificates -y
update-ca-certificates
```

### "permission denied" errors

```bash
# In proot, you're technically root but sandboxed
# Files from Termux shared storage need fixing:
chmod -R 755 ~/atropos
```

### Slow git clone

```bash
# Shallow clone (faster)
git clone --depth 1 https://github.com/BugBlocker/atropos.git

# Or use SSH (sometimes faster)
git clone git@github.com:BugBlocker/atropos.git
```

### Can't access localhost from browser

```bash
# Make sure you're binding to all interfaces
atropos serve --host 0.0.0.0 --port 8080

# Check if port is in use
ss -tlnp | grep 8080
```

---

## 📋 Complete Setup Script

Save this and run it:

```bash
cat > ~/setup_atropos.sh << 'SCRIPT'
#!/bin/bash
set -e

echo "⚔️ Atropos OSINT Platform - Termux Setup"
echo "======================================="

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}[1/5] Installing packages...${NC}"
apt update
apt install -y build-essential libssl-dev pkg-config libluajit-5.1-dev \
    git curl python3 python3-pip golang-go ca-certificates

echo -e "${CYAN}[2/5] Installing Rust...${NC}"
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo -e "${CYAN}[3/5] Cloning Atropos...${NC}"
cd ~
if [ ! -d "atropos" ]; then
    git clone --depth 1 https://github.com/BugBlocker/atropos.git
fi
cd atropos

echo -e "${CYAN}[4/5] Building (this takes a while)...${NC}"
CARGO_BUILD_JOBS=2 cargo build --release

echo -e "${CYAN}[5/5] Installing...${NC}"
mkdir -p ~/.local/bin
cp target/release/atropos ~/.local/bin/

# Update PATH
grep -q 'local/bin' ~/.bashrc || echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc

echo -e "${GREEN}"
echo "✅ Setup complete!"
echo ""
echo "Commands:"
echo "  atropos --help     Show help"
echo "  atropos serve      Start web UI"
echo "  atropos scan       Run scans"
echo ""
echo "Web UI: http://localhost:8080"
echo -e "${NC}"
SCRIPT
chmod +x ~/setup_atropos.sh
```

Then in proot Ubuntu:
```bash
~/setup_atropos.sh
```

---

## 🎯 TL;DR

```bash
# Termux
pkg update && pkg install proot-distro git gh -y
proot-distro install ubuntu
proot-distro login ubuntu

# Ubuntu (proot) - copy paste this whole block
apt update && apt install -y build-essential libssl-dev pkg-config libluajit-5.1-dev git curl && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
source "$HOME/.cargo/env" && \
git clone --depth 1 https://github.com/BugBlocker/atropos.git && \
cd atropos && \
CARGO_BUILD_JOBS=2 cargo build --release && \
mkdir -p ~/.local/bin && \
cp target/release/atropos ~/.local/bin/ && \
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc && \
source ~/.bashrc && \
atropos --version
```

**For Git: Use `gh` (GitHub CLI) - it's the easiest on mobile!**
