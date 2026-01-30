use axum::{
    body::{boxed, Full},
    http::{header, Response, StatusCode, Uri},
    response::IntoResponse,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/web/static/"]
struct Asset;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    // Default to index.html for root or unknown paths (SPA support)
    let path = if path.is_empty() || !path.contains('.') {
        "index.html"
    } else {
        path
    };
    
    match Asset::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(boxed(Full::from(content.data)))
                .unwrap()
        }
        None => {
            // Try index.html for SPA routing
            match Asset::get("index.html") {
                Some(content) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(boxed(Full::from(content.data)))
                    .unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(boxed(Full::from("404 Not Found")))
                    .unwrap(),
            }
        }
    }
}
