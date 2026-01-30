use axum::{
    routing::{get, post, delete},
    Router,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use crate::web::api;
use crate::web::static_files::static_handler;

// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub secrets: Arc<RwLock<HashMap<String, String>>>,
    pub scan_results: Arc<RwLock<Vec<ScanResult>>>,
    pub active_scans: Arc<RwLock<HashMap<String, ScanStatus>>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub id: String,
    pub target: String,
    pub script: String,
    pub status: String,
    pub findings: Vec<Finding>,
    pub started_at: String,
    pub completed_at: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: String,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScanStatus {
    pub id: String,
    pub status: String,  // "running", "completed", "failed"
    pub progress: u32,
    pub message: String,
}

impl Default for AppState {
    fn default() -> Self {
        // Load secrets from environment
        let mut secrets = HashMap::new();
        let env_keys = vec![
            ("SHODAN_API_KEY", "shodan"),
            ("VIRUSTOTAL_API_KEY", "virustotal"),
            ("GITHUB_TOKEN", "github"),
            ("ABUSEIPDB_API_KEY", "abuseipdb"),
            ("SECURITYTRAILS_API_KEY", "securitytrails"),
            ("CENSYS_API_ID", "censys_id"),
            ("HUNTER_API_KEY", "hunter"),
            ("OTX_API_KEY", "otx"),
        ];
        
        for (env_var, key) in env_keys {
            if let Ok(value) = std::env::var(env_var) {
                if !value.is_empty() {
                    secrets.insert(key.to_string(), value);
                }
            }
        }
        
        Self {
            secrets: Arc::new(RwLock::new(secrets)),
            scan_results: Arc::new(RwLock::new(Vec::new())),
            active_scans: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn start_server(host: &str, port: u16) {
    let state = AppState::default();
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        // API routes
        .route("/api/health", get(health_check))
        .route("/api/secrets", get(api::list_secrets))
        .route("/api/secrets", post(api::set_secret))
        .route("/api/secrets/:key", delete(api::delete_secret))
        .route("/api/scan", post(api::start_scan))
        .route("/api/scan/:id", get(api::get_scan_status))
        .route("/api/scan/:id/stop", post(api::stop_scan))
        .route("/api/results", get(api::list_results))
        .route("/api/results/:id", get(api::get_result))
        .route("/api/scripts", get(api::list_scripts))
        .route("/api/tools", get(api::list_tools))
        // Static files (frontend)
        .fallback(static_handler)
        .layer(cors)
        .with_state(state);
    
    let addr: std::net::SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address");
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    LOTUS OSINT PLATFORM                      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Web UI:  http://{}:{}                              ║", host, port);
    println!("║  API:     http://{}:{}/api                          ║", host, port);
    println!("╚══════════════════════════════════════════════════════════════╝");
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "name": "Lotus OSINT Platform"
    }))
}
