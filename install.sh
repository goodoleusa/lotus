#!/bin/bash

# ============================================
# Atropos OSINT Platform - Installation Script
# ============================================

set -e

# Colors
PINK='\033[38;5;205m'
CYAN='\033[38;5;51m'
PURPLE='\033[38;5;141m'
GREEN='\033[38;5;156m'
YELLOW='\033[38;5;229m'
RED='\033[38;5;203m'
NC='\033[0m'

# ASCII Art
echo -e "${PINK}"
cat << 'EOF'
 ██╗      ██████╗ ████████╗██╗   ██╗███████╗
 ██║     ██╔═══██╗╚══██╔══╝██║   ██║██╔════╝
 ██║     ██║   ██║   ██║   ██║   ██║███████╗
 ██║     ██║   ██║   ██║   ██║   ██║╚════██║
 ███████╗╚██████╔╝   ██║   ╚██████╔╝███████║
 ╚══════╝ ╚═════╝    ╚═╝    ╚═════╝ ╚══════╝
         OSINT & THREAT INTEL PLATFORM
EOF
echo -e "${NC}"

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}Welcome to the Atropos Installation Script${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [ -f /etc/debian_version ]; then
            echo "debian"
        elif [ -f /etc/fedora-release ]; then
            echo "fedora"
        elif [ -f /etc/arch-release ]; then
            echo "arch"
        else
            echo "linux"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "win32" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

OS=$(detect_os)
echo -e "${PURPLE}Detected OS: ${OS}${NC}"

# Check for required tools
check_command() {
    if command -v "$1" &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} $1 found"
        return 0
    else
        echo -e "  ${RED}✗${NC} $1 not found"
        return 1
    fi
}

# Install system dependencies
install_deps() {
    echo -e "\n${CYAN}[1/5] Installing system dependencies...${NC}"
    
    case $OS in
        debian)
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                libssl-dev \
                pkg-config \
                libluajit-5.1-dev \
                luajit \
                git \
                curl
            ;;
        fedora)
            sudo dnf install -y \
                gcc \
                openssl-devel \
                pkgconfig \
                luajit-devel \
                git \
                curl
            ;;
        arch)
            sudo pacman -S --noconfirm \
                base-devel \
                openssl \
                pkgconf \
                luajit \
                git \
                curl
            ;;
        macos)
            if ! command -v brew &> /dev/null; then
                echo -e "${YELLOW}Installing Homebrew...${NC}"
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi
            brew install openssl luajit pkg-config
            ;;
        windows)
            echo -e "${YELLOW}Windows detected. For best experience, use the PowerShell installer:${NC}"
            echo -e "${CYAN}  powershell -ExecutionPolicy Bypass -File install.ps1${NC}"
            echo ""
            echo -e "${YELLOW}Or use WSL2 for a Linux environment:${NC}"
            echo -e "${CYAN}  wsl --install${NC}"
            echo ""
            echo -e "Attempting to continue with Git Bash/MSYS2..."
            
            # Try pacman (MSYS2)
            if command -v pacman &> /dev/null; then
                pacman -S --noconfirm \
                    mingw-w64-x86_64-toolchain \
                    mingw-w64-x86_64-openssl \
                    mingw-w64-x86_64-luajit \
                    git \
                    curl
            else
                echo -e "${RED}Please install MSYS2 or use the PowerShell installer.${NC}"
                exit 1
            fi
            ;;
        *)
            echo -e "${RED}Unsupported OS. Please install dependencies manually.${NC}"
            echo "Required: OpenSSL dev, pkg-config, LuaJIT dev, git, curl"
            echo ""
            echo "For Windows, use: powershell -ExecutionPolicy Bypass -File install.ps1"
            exit 1
            ;;
    esac
    
    echo -e "${GREEN}✓ System dependencies installed${NC}"
}

# Install Rust
install_rust() {
    echo -e "\n${CYAN}[2/5] Checking Rust installation...${NC}"
    
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        echo -e "${GREEN}✓ Rust $RUST_VERSION already installed${NC}"
    else
        echo -e "${YELLOW}Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo -e "${GREEN}✓ Rust installed${NC}"
    fi
}

# Install Atropos
install_atropos() {
    echo -e "\n${CYAN}[3/5] Installing Atropos...${NC}"
    
    if [ -d "atropos" ]; then
        echo -e "${YELLOW}Updating existing Atropos installation...${NC}"
        cd atropos
        git pull origin master
    else
        echo -e "${YELLOW}Cloning Atropos repository...${NC}"
        git clone https://github.com/BugBlocker/atropos.git
        cd atropos
    fi
    
    echo -e "${YELLOW}Building Atropos (this may take a few minutes)...${NC}"
    cargo build --release
    
    # Install binary
    if [ -w "/usr/local/bin" ]; then
        sudo cp target/release/atropos /usr/local/bin/
    else
        mkdir -p "$HOME/.local/bin"
        cp target/release/atropos "$HOME/.local/bin/"
        echo -e "${YELLOW}Added atropos to ~/.local/bin${NC}"
        echo -e "${YELLOW}Make sure ~/.local/bin is in your PATH${NC}"
    fi
    
    cd ..
    echo -e "${GREEN}✓ Atropos installed${NC}"
}

# Install OSINT tools
install_osint_tools() {
    echo -e "\n${CYAN}[4/5] Installing OSINT tools (optional)...${NC}"
    
    read -p "Install recommended OSINT tools? [y/N] " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Installing Python tools...${NC}"
        pip3 install --user bbot theHarvester shodan || true
        
        if command -v go &> /dev/null; then
            echo -e "${YELLOW}Installing Go tools...${NC}"
            go install -v github.com/owasp-amass/amass/v4/...@master || true
            go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest || true
            go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest || true
            go install -v github.com/projectdiscovery/httpx/cmd/httpx@latest || true
            go install github.com/gitleaks/gitleaks/v8@latest || true
        else
            echo -e "${YELLOW}Go not found, skipping Go-based tools${NC}"
        fi
        
        echo -e "${GREEN}✓ OSINT tools installed${NC}"
    else
        echo -e "${YELLOW}Skipping OSINT tools installation${NC}"
    fi
}

# Setup secrets
setup_secrets() {
    echo -e "\n${CYAN}[5/5] Setting up secrets configuration...${NC}"
    
    CONFIG_DIR="$HOME/.config/atropos"
    mkdir -p "$CONFIG_DIR"
    
    if [ ! -f "$HOME/.atropos_secrets.json" ]; then
        cat > "$HOME/.atropos_secrets.json" << 'SECRETS'
{
    "shodan": "",
    "virustotal": "",
    "securitytrails": "",
    "censys_id": "",
    "censys_secret": "",
    "hunter": "",
    "github": "",
    "abuseipdb": "",
    "otx": ""
}
SECRETS
        chmod 600 "$HOME/.atropos_secrets.json"
        echo -e "${GREEN}✓ Created ~/.atropos_secrets.json template${NC}"
        echo -e "${YELLOW}  Edit this file to add your API keys${NC}"
    else
        echo -e "${GREEN}✓ Secrets file already exists${NC}"
    fi
}

# Print completion message
print_complete() {
    echo
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Installation Complete!${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
    echo -e "${PURPLE}Quick Start:${NC}"
    echo -e "  ${CYAN}atropos --help${NC}              Show help"
    echo -e "  ${CYAN}atropos serve${NC}               Start web UI"
    echo -e "  ${CYAN}atropos scan script.lua${NC}     Run a scan"
    echo
    echo -e "${PURPLE}Configure API Keys:${NC}"
    echo -e "  ${CYAN}export SHODAN_API_KEY=\"your-key\"${NC}"
    echo -e "  Or edit: ${CYAN}~/.atropos_secrets.json${NC}"
    echo
    echo -e "${PURPLE}Documentation:${NC}"
    echo -e "  ${CYAN}https://github.com/BugBlocker/atropos${NC}"
    echo
    echo -e "${PINK}⚔️ Happy Hunting!${NC}"
    echo
}

# Main installation
main() {
    install_deps
    install_rust
    install_atropos
    install_osint_tools
    setup_secrets
    print_complete
}

main "$@"
