pub mod new;
pub mod scan;
pub mod serve;
use new::NewOpts;
use scan::UrlsOpts;
use serve::ServeOpts;
use structopt::StructOpt;

const ABOUT: &str = r#"
Atropos - The Thread Cutter • OSINT & Security Platform
========================================================

"She who cannot be turned" - Weave intelligence. Measure threats. Cut through the noise.

A powerful security automation framework using Lua scripts for:
  • OSINT and reconnaissance (subdomains, emails, infrastructure)
  • Threat intelligence gathering (Shodan, VirusTotal, etc.)
  • Web vulnerability scanning (XSS, SQLi, SSTI, etc.)
  • Secret detection and security automation

QUICK START:
  # Scan a target with scripts
  echo "example.com" | atropos scan osint_scanner.lua

  # Scan with output file
  echo "example.com" | atropos scan ./scripts/ -o results.json

  # Start web UI
  atropos serve

  # With verbose output and custom workers
  cat targets.txt | atropos scan ./scripts/ -v -w 20

SECRETS MANAGEMENT:
  Set API keys via environment variables:
    export SHODAN_API_KEY="your-key"
    export VIRUSTOTAL_API_KEY="your-key"
  
  Or create ~/.atropos_secrets.json (see docs/osint_integration.md)

Documentation: docs/lua_scripting.md, docs/osint_integration.md
"#;

const SCAN_ABOUT: &str = r#"
Run security scans using Lua scripts against target URLs/hosts.

EXAMPLES:
  # Basic URL scanning
  echo "https://target.com/page?id=1" | atropos scan xss_scanner.lua

  # OSINT reconnaissance
  echo "example.com" | atropos scan examples/threat_intel_scanner.lua

  # Scan with output file
  cat targets.txt | atropos scan ./vuln_scripts/ -o results.json

  # With proxy (for Burp/ZAP)
  echo "https://target.com" | atropos scan script.lua -p http://127.0.0.1:8080

  # Pass environment variables to scripts
  echo "target.com" | atropos scan script.lua --env-vars '{"API_KEY":"xxx"}'

  # Resume interrupted scan
  atropos scan ./scripts/ --urls targets.txt --resume resume.cfg

INPUT TYPES (set SCAN_TYPE in your Lua script):
  1 = HOSTS     - Domain/IP only (e.g., "example.com")
  2 = URLS      - Full URL with parameters (e.g., "https://example.com/?id=1")
  3 = PATHS     - URL paths without parameters
  4 = CUSTOM    - Custom input via --input-handler
"#;

const NEW_ABOUT: &str = r#"
Generate a new Lua script template for security scanning.

EXAMPLES:
  # Create a URL parameter scanner
  atropos new --type 2 -o my_scanner.lua

  # Create a host/domain scanner (for OSINT)
  atropos new --type 1 -o osint_recon.lua

SCRIPT TYPES:
  1 = HOST scanner   - For domain/IP reconnaissance
  2 = URL scanner    - For URL parameter testing
  3 = PATH scanner   - For path/directory scanning
  4 = CUSTOM scanner - For custom input handling
"#;

const SERVE_ABOUT: &str = r#"
Start the Lotus web UI server.

EXAMPLES:
  # Start on default port (8080)
  atropos serve

  # Start on custom port
  atropos serve --port 3000

  # Bind to all interfaces (for network access)
  atropos serve --host 0.0.0.0 --port 8080

  # Open browser automatically
  atropos serve --open

FEATURES:
  • Dashboard with scan statistics
  • Launch and monitor scans
  • Manage API keys
  • View scan results
  • Browse available tools and scripts
"#;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "atropos",
    about = "The Thread Cutter • OSINT & Security Platform",
    long_about = ABOUT,
    after_help = "Use 'atropos <command> --help' for more information about a command."
)]
pub enum Opts {
    #[structopt(about = "Create a new Lua script template", long_about = NEW_ABOUT)]
    New(NewOpts),
    #[structopt(
        name = "scan",
        about = "Run security scans with Lua scripts",
        long_about = SCAN_ABOUT,
        visible_alias = "s"
    )]
    Scan(UrlsOpts),
    #[structopt(
        name = "serve",
        about = "Start the web UI server",
        long_about = SERVE_ABOUT,
        visible_alias = "ui"
    )]
    Serve(ServeOpts),
}
