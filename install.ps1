# ============================================
# Lotus OSINT Platform - Windows Installation Script
# ============================================
# Run as Administrator: powershell -ExecutionPolicy Bypass -File install.ps1

$ErrorActionPreference = "Stop"

# Colors
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) { Write-Output $args }
    $host.UI.RawUI.ForegroundColor = $fc
}

# ASCII Art
Write-Host @"

 ██╗      ██████╗ ████████╗██╗   ██╗███████╗
 ██║     ██╔═══██╗╚══██╔══╝██║   ██║██╔════╝
 ██║     ██║   ██║   ██║   ██║   ██║███████╗
 ██║     ██║   ██║   ██║   ██║   ██║╚════██║
 ███████╗╚██████╔╝   ██║   ╚██████╔╝███████║
 ╚══════╝ ╚═════╝    ╚═╝    ╚═════╝ ╚══════╝
         OSINT & THREAT INTEL PLATFORM

"@ -ForegroundColor Magenta

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "Welcome to the Lotus Installation Script for Windows" -ForegroundColor Yellow
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "WARNING: Not running as Administrator. Some features may not work." -ForegroundColor Yellow
    Write-Host "Consider running: Start-Process powershell -Verb runAs -ArgumentList '-File install.ps1'" -ForegroundColor Yellow
    Write-Host ""
}

# Check for Chocolatey
function Install-Chocolatey {
    Write-Host "[1/6] Checking for Chocolatey..." -ForegroundColor Cyan
    
    if (Get-Command choco -ErrorAction SilentlyContinue) {
        Write-Host "  ✓ Chocolatey already installed" -ForegroundColor Green
    } else {
        Write-Host "  Installing Chocolatey..." -ForegroundColor Yellow
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
        
        # Refresh environment
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
        Write-Host "  ✓ Chocolatey installed" -ForegroundColor Green
    }
}

# Install build tools
function Install-BuildTools {
    Write-Host "`n[2/6] Installing build tools..." -ForegroundColor Cyan
    
    # Check for Visual Studio Build Tools
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    $hasBuildTools = $false
    
    if (Test-Path $vsWhere) {
        $installed = & $vsWhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        if ($installed) {
            $hasBuildTools = $true
            Write-Host "  ✓ Visual Studio Build Tools found" -ForegroundColor Green
        }
    }
    
    if (-not $hasBuildTools) {
        Write-Host "  Installing Visual Studio Build Tools..." -ForegroundColor Yellow
        choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended" -y
        Write-Host "  ✓ Build Tools installed" -ForegroundColor Green
    }
    
    # Install Git
    if (Get-Command git -ErrorAction SilentlyContinue) {
        Write-Host "  ✓ Git already installed" -ForegroundColor Green
    } else {
        Write-Host "  Installing Git..." -ForegroundColor Yellow
        choco install git -y
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
        Write-Host "  ✓ Git installed" -ForegroundColor Green
    }
}

# Install Rust
function Install-Rust {
    Write-Host "`n[3/6] Checking Rust installation..." -ForegroundColor Cyan
    
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        $rustVersion = rustc --version
        Write-Host "  ✓ $rustVersion already installed" -ForegroundColor Green
    } else {
        Write-Host "  Installing Rust..." -ForegroundColor Yellow
        
        # Download and run rustup-init
        $rustupInit = "$env:TEMP\rustup-init.exe"
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupInit
        Start-Process -FilePath $rustupInit -ArgumentList "-y" -Wait -NoNewWindow
        Remove-Item $rustupInit
        
        # Add to PATH
        $env:Path += ";$env:USERPROFILE\.cargo\bin"
        [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", "User")
        
        Write-Host "  ✓ Rust installed" -ForegroundColor Green
    }
}

# Install LuaJIT (using pre-built binaries)
function Install-LuaJIT {
    Write-Host "`n[4/6] Setting up LuaJIT..." -ForegroundColor Cyan
    
    $luaDir = "$env:USERPROFILE\.lotus\lua"
    
    if (Test-Path "$luaDir\lua51.dll") {
        Write-Host "  ✓ LuaJIT already set up" -ForegroundColor Green
    } else {
        Write-Host "  Downloading LuaJIT..." -ForegroundColor Yellow
        
        New-Item -ItemType Directory -Force -Path $luaDir | Out-Null
        
        # Download LuaJIT binaries
        $luaZip = "$env:TEMP\luajit.zip"
        Invoke-WebRequest -Uri "https://github.com/nicknisi/luajit-rocks/releases/download/v2.1-20220915/luajit-2.1.0-beta3-windows-amd64.zip" -OutFile $luaZip -ErrorAction SilentlyContinue
        
        if (Test-Path $luaZip) {
            Expand-Archive -Path $luaZip -DestinationPath $luaDir -Force
            Remove-Item $luaZip
        } else {
            # Fallback: Install via choco
            choco install luajit -y
        }
        
        Write-Host "  ✓ LuaJIT set up" -ForegroundColor Green
    }
    
    # Set environment variables for Rust build
    $env:LUA_INC = $luaDir
    $env:LUA_LIB = $luaDir
    $env:LUA_LIB_NAME = "lua51"
}

# Install Lotus
function Install-Lotus {
    Write-Host "`n[5/6] Installing Lotus..." -ForegroundColor Cyan
    
    $lotusDir = "$env:USERPROFILE\.lotus"
    $repoDir = "$lotusDir\lotus-repo"
    
    New-Item -ItemType Directory -Force -Path $lotusDir | Out-Null
    
    if (Test-Path $repoDir) {
        Write-Host "  Updating existing installation..." -ForegroundColor Yellow
        Push-Location $repoDir
        git pull origin master
    } else {
        Write-Host "  Cloning repository..." -ForegroundColor Yellow
        git clone https://github.com/BugBlocker/lotus.git $repoDir
        Push-Location $repoDir
    }
    
    Write-Host "  Building Lotus (this may take several minutes)..." -ForegroundColor Yellow
    
    # Build with vendored Lua to avoid dependency issues
    cargo build --release --features vendored
    
    # Copy binary to a location in PATH
    $binDir = "$env:USERPROFILE\.cargo\bin"
    Copy-Item "target\release\lotus.exe" "$binDir\lotus.exe" -Force
    
    Pop-Location
    
    Write-Host "  ✓ Lotus installed to $binDir\lotus.exe" -ForegroundColor Green
}

# Setup secrets
function Setup-Secrets {
    Write-Host "`n[6/6] Setting up configuration..." -ForegroundColor Cyan
    
    $configDir = "$env:USERPROFILE\.lotus"
    $secretsFile = "$env:USERPROFILE\.lotus_secrets.json"
    
    New-Item -ItemType Directory -Force -Path $configDir | Out-Null
    
    if (-not (Test-Path $secretsFile)) {
        $secretsTemplate = @"
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
"@
        $secretsTemplate | Out-File -FilePath $secretsFile -Encoding UTF8
        Write-Host "  ✓ Created $secretsFile" -ForegroundColor Green
        Write-Host "    Edit this file to add your API keys" -ForegroundColor Yellow
    } else {
        Write-Host "  ✓ Secrets file already exists" -ForegroundColor Green
    }
}

# Install OSINT tools (optional)
function Install-OSINTTools {
    Write-Host "`nInstall recommended OSINT tools? (y/N): " -ForegroundColor Cyan -NoNewline
    $response = Read-Host
    
    if ($response -eq 'y' -or $response -eq 'Y') {
        Write-Host "Installing OSINT tools..." -ForegroundColor Yellow
        
        # Python tools
        if (Get-Command pip -ErrorAction SilentlyContinue) {
            Write-Host "  Installing Python tools..." -ForegroundColor Yellow
            pip install bbot theHarvester shodan --quiet 2>$null
        } else {
            Write-Host "  Python/pip not found, installing..." -ForegroundColor Yellow
            choco install python -y
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
            pip install bbot theHarvester shodan --quiet 2>$null
        }
        
        # Go tools
        if (Get-Command go -ErrorAction SilentlyContinue) {
            Write-Host "  Installing Go tools..." -ForegroundColor Yellow
            go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest 2>$null
            go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest 2>$null
            go install -v github.com/projectdiscovery/httpx/cmd/httpx@latest 2>$null
        } else {
            Write-Host "  Go not found, skipping Go-based tools" -ForegroundColor Yellow
            Write-Host "  Install Go from: https://go.dev/dl/" -ForegroundColor Yellow
        }
        
        Write-Host "  ✓ OSINT tools installation complete" -ForegroundColor Green
    } else {
        Write-Host "Skipping OSINT tools installation" -ForegroundColor Yellow
    }
}

# Print completion message
function Print-Complete {
    Write-Host ""
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
    Write-Host "Installation Complete!" -ForegroundColor Green
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Quick Start:" -ForegroundColor Magenta
    Write-Host "  lotus --help              " -ForegroundColor Cyan -NoNewline
    Write-Host "Show help"
    Write-Host "  lotus serve               " -ForegroundColor Cyan -NoNewline
    Write-Host "Start web UI"
    Write-Host "  lotus scan script.lua     " -ForegroundColor Cyan -NoNewline
    Write-Host "Run a scan"
    Write-Host ""
    Write-Host "Configure API Keys:" -ForegroundColor Magenta
    Write-Host '  $env:SHODAN_API_KEY="your-key"' -ForegroundColor Cyan
    Write-Host "  Or edit: $env:USERPROFILE\.lotus_secrets.json" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "NOTE: You may need to restart your terminal for PATH changes." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "🪷 Happy Hunting!" -ForegroundColor Magenta
    Write-Host ""
}

# Main
try {
    Install-Chocolatey
    Install-BuildTools
    Install-Rust
    Install-LuaJIT
    Install-Lotus
    Setup-Secrets
    Install-OSINTTools
    Print-Complete
} catch {
    Write-Host "`nError: $_" -ForegroundColor Red
    Write-Host "Installation failed. Please check the error message above." -ForegroundColor Red
    exit 1
}
