use axum::{
    extract::{State, Path, Query},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::web::server::{AppState, ScanResult, ScanStatus, Finding};

// ============================================================================
// Secrets Management
// ============================================================================

#[derive(Serialize)]
pub struct SecretInfo {
    pub key: String,
    pub configured: bool,
    pub masked_value: String,
}

pub async fn list_secrets(State(state): State<AppState>) -> impl IntoResponse {
    let secrets = state.secrets.read().await;
    
    let known_services = vec![
        ("shodan", "Shodan", "SHODAN_API_KEY"),
        ("virustotal", "VirusTotal", "VIRUSTOTAL_API_KEY"),
        ("securitytrails", "SecurityTrails", "SECURITYTRAILS_API_KEY"),
        ("censys_id", "Censys", "CENSYS_API_ID"),
        ("hunter", "Hunter.io", "HUNTER_API_KEY"),
        ("github", "GitHub", "GITHUB_TOKEN"),
        ("abuseipdb", "AbuseIPDB", "ABUSEIPDB_API_KEY"),
        ("otx", "AlienVault OTX", "OTX_API_KEY"),
        ("binaryedge", "BinaryEdge", "BINARYEDGE_API_KEY"),
        ("urlscan", "URLScan", "URLSCAN_API_KEY"),
        ("whoisxml", "WhoisXML", "WHOISXML_API_KEY"),
        ("openai", "OpenAI", "OPENAI_API_KEY"),
    ];
    
    let mut result: Vec<serde_json::Value> = Vec::new();
    
    for (key, display_name, env_var) in known_services {
        let configured = secrets.contains_key(key);
        let masked = if configured {
            let val = secrets.get(key).unwrap();
            if val.len() > 8 {
                format!("{}...{}", &val[..4], &val[val.len()-4..])
            } else {
                "****".to_string()
            }
        } else {
            "Not configured".to_string()
        };
        
        result.push(serde_json::json!({
            "key": key,
            "display_name": display_name,
            "env_var": env_var,
            "configured": configured,
            "masked_value": masked
        }));
    }
    
    Json(serde_json::json!({
        "secrets": result,
        "total_configured": secrets.len()
    }))
}

#[derive(Deserialize)]
pub struct SetSecretRequest {
    pub key: String,
    pub value: String,
}

pub async fn set_secret(
    State(state): State<AppState>,
    Json(payload): Json<SetSecretRequest>,
) -> impl IntoResponse {
    let mut secrets = state.secrets.write().await;
    secrets.insert(payload.key.to_lowercase(), payload.value);
    
    Json(serde_json::json!({
        "success": true,
        "message": format!("Secret '{}' has been set", payload.key)
    }))
}

pub async fn delete_secret(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let mut secrets = state.secrets.write().await;
    let removed = secrets.remove(&key.to_lowercase()).is_some();
    
    if removed {
        Json(serde_json::json!({
            "success": true,
            "message": format!("Secret '{}' has been removed", key)
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "message": format!("Secret '{}' not found", key)
        }))
    }
}

// ============================================================================
// Scan Management
// ============================================================================

#[derive(Deserialize)]
pub struct StartScanRequest {
    pub target: String,
    pub script: Option<String>,
    pub scan_type: Option<String>,  // "subdomain", "vuln", "osint", "full"
    pub options: Option<HashMap<String, String>>,
}

pub async fn start_scan(
    State(state): State<AppState>,
    Json(payload): Json<StartScanRequest>,
) -> impl IntoResponse {
    let scan_id = Uuid::new_v4().to_string()[..8].to_string();
    let scan_type = payload.scan_type.unwrap_or_else(|| "osint".to_string());
    let script = payload.script.unwrap_or_else(|| {
        match scan_type.as_str() {
            "subdomain" => "examples/amass_osint.lua".to_string(),
            "vuln" => "examples/finalrecon_scanner.lua".to_string(),
            "full" => "examples/threat_intel_scanner.lua".to_string(),
            _ => "examples/bbot_scanner.lua".to_string(),
        }
    });
    
    // Create scan status
    let status = ScanStatus {
        id: scan_id.clone(),
        status: "running".to_string(),
        progress: 0,
        message: format!("Starting scan on {}", payload.target),
    };
    
    // Store active scan
    {
        let mut active_scans = state.active_scans.write().await;
        active_scans.insert(scan_id.clone(), status);
    }
    
    // Clone state for the spawned task
    let state_clone = state.clone();
    let target = payload.target.clone();
    let scan_id_clone = scan_id.clone();
    
    // Spawn scan in background
    tokio::spawn(async move {
        // Simulate scan progress (in real implementation, this would run actual scan)
        for progress in (0..=100).step_by(10) {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            let mut active_scans = state_clone.active_scans.write().await;
            if let Some(status) = active_scans.get_mut(&scan_id_clone) {
                status.progress = progress;
                status.message = format!("Scanning {} - {}%", target, progress);
            }
        }
        
        // Mark scan as complete
        {
            let mut active_scans = state_clone.active_scans.write().await;
            if let Some(status) = active_scans.get_mut(&scan_id_clone) {
                status.status = "completed".to_string();
                status.progress = 100;
                status.message = "Scan completed".to_string();
            }
        }
        
        // Add to results
        {
            let mut results = state_clone.scan_results.write().await;
            results.push(ScanResult {
                id: scan_id_clone,
                target: target.clone(),
                script: script,
                status: "completed".to_string(),
                findings: vec![
                    Finding {
                        severity: "info".to_string(),
                        title: "Subdomains discovered".to_string(),
                        description: format!("Found potential subdomains for {}", target),
                        url: None,
                        data: Some(serde_json::json!(["www", "api", "mail", "dev"])),
                    },
                ],
                started_at: chrono::Utc::now().to_rfc3339(),
                completed_at: Some(chrono::Utc::now().to_rfc3339()),
            });
        }
    });
    
    Json(serde_json::json!({
        "success": true,
        "scan_id": scan_id,
        "message": format!("Scan started for {}", payload.target)
    }))
}

pub async fn get_scan_status(
    State(state): State<AppState>,
    Path(scan_id): Path<String>,
) -> impl IntoResponse {
    let active_scans = state.active_scans.read().await;
    
    if let Some(status) = active_scans.get(&scan_id) {
        Json(serde_json::json!({
            "found": true,
            "scan": status
        }))
    } else {
        // Check completed results
        let results = state.scan_results.read().await;
        if let Some(result) = results.iter().find(|r| r.id == scan_id) {
            Json(serde_json::json!({
                "found": true,
                "scan": {
                    "id": result.id,
                    "status": result.status,
                    "progress": 100,
                    "message": "Scan completed"
                }
            }))
        } else {
            Json(serde_json::json!({
                "found": false,
                "message": "Scan not found"
            }))
        }
    }
}

pub async fn stop_scan(
    State(state): State<AppState>,
    Path(scan_id): Path<String>,
) -> impl IntoResponse {
    let mut active_scans = state.active_scans.write().await;
    
    if let Some(status) = active_scans.get_mut(&scan_id) {
        status.status = "stopped".to_string();
        status.message = "Scan stopped by user".to_string();
        Json(serde_json::json!({
            "success": true,
            "message": "Scan stopped"
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "message": "Scan not found or already completed"
        }))
    }
}

// ============================================================================
// Results
// ============================================================================

#[derive(Deserialize)]
pub struct ListResultsQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn list_results(
    State(state): State<AppState>,
    Query(params): Query<ListResultsQuery>,
) -> impl IntoResponse {
    let results = state.scan_results.read().await;
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    
    let paginated: Vec<_> = results.iter()
        .skip(offset)
        .take(limit)
        .collect();
    
    Json(serde_json::json!({
        "results": paginated,
        "total": results.len(),
        "limit": limit,
        "offset": offset
    }))
}

pub async fn get_result(
    State(state): State<AppState>,
    Path(result_id): Path<String>,
) -> impl IntoResponse {
    let results = state.scan_results.read().await;
    
    if let Some(result) = results.iter().find(|r| r.id == result_id) {
        Json(serde_json::json!({
            "found": true,
            "result": result
        }))
    } else {
        Json(serde_json::json!({
            "found": false,
            "message": "Result not found"
        }))
    }
}

// ============================================================================
// Scripts & Tools
// ============================================================================

pub async fn list_scripts() -> impl IntoResponse {
    let scripts = vec![
        serde_json::json!({
            "name": "threat_intel_scanner.lua",
            "path": "examples/threat_intel_scanner.lua",
            "description": "Comprehensive threat intelligence scanner",
            "category": "osint",
            "tools": ["amass", "theharvester", "shodan", "nuclei"]
        }),
        serde_json::json!({
            "name": "bbot_scanner.lua",
            "path": "examples/bbot_scanner.lua",
            "description": "BBOT recursive OSINT automation",
            "category": "osint",
            "tools": ["bbot"]
        }),
        serde_json::json!({
            "name": "amass_osint.lua",
            "path": "examples/amass_osint.lua",
            "description": "Subdomain enumeration with Amass",
            "category": "subdomain",
            "tools": ["amass"]
        }),
        serde_json::json!({
            "name": "finalrecon_scanner.lua",
            "path": "examples/finalrecon_scanner.lua",
            "description": "Web reconnaissance scanner",
            "category": "web",
            "tools": ["finalrecon"]
        }),
        serde_json::json!({
            "name": "spiderfoot_osint.lua",
            "path": "examples/spiderfoot_osint.lua",
            "description": "SpiderFoot OSINT integration",
            "category": "osint",
            "tools": ["spiderfoot"]
        }),
    ];
    
    Json(serde_json::json!({
        "scripts": scripts
    }))
}

pub async fn list_tools() -> impl IntoResponse {
    let tools = vec![
        serde_json::json!({
            "name": "BBOT",
            "description": "Recursive OSINT automation",
            "category": "osint",
            "install": "pip install bbot",
            "docs": "https://github.com/blacklanternsecurity/bbot"
        }),
        serde_json::json!({
            "name": "Amass",
            "description": "Subdomain enumeration",
            "category": "subdomain",
            "install": "go install -v github.com/owasp-amass/amass/v4/...@master",
            "docs": "https://github.com/OWASP/Amass"
        }),
        serde_json::json!({
            "name": "Nuclei",
            "description": "Vulnerability scanning",
            "category": "vuln",
            "install": "go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest",
            "docs": "https://github.com/projectdiscovery/nuclei"
        }),
        serde_json::json!({
            "name": "Shodan",
            "description": "Internet device search",
            "category": "osint",
            "install": "pip install shodan",
            "docs": "https://shodan.io"
        }),
        serde_json::json!({
            "name": "theHarvester",
            "description": "Email/subdomain harvesting",
            "category": "osint",
            "install": "pip install theHarvester",
            "docs": "https://github.com/laramies/theHarvester"
        }),
        serde_json::json!({
            "name": "FinalRecon",
            "description": "Web reconnaissance",
            "category": "web",
            "install": "pip install finalrecon",
            "docs": "https://github.com/thewhiteh4t/FinalRecon"
        }),
        serde_json::json!({
            "name": "SpiderFoot",
            "description": "Automated OSINT",
            "category": "osint",
            "install": "pip install spiderfoot",
            "docs": "https://github.com/smicallef/spiderfoot"
        }),
        serde_json::json!({
            "name": "Gitleaks",
            "description": "Secret scanning",
            "category": "secrets",
            "install": "go install github.com/gitleaks/gitleaks/v8@latest",
            "docs": "https://github.com/gitleaks/gitleaks"
        }),
    ];
    
    Json(serde_json::json!({
        "tools": tools
    }))
}
