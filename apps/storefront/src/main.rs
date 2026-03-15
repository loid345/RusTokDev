use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::path::PathBuf;

fn static_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
}

async fn css_handler() -> impl IntoResponse {
    let css_path = static_dir().join("app.css");
    match tokio::fs::read(css_path).await {
        Ok(contents) => ([(header::CONTENT_TYPE, "text/css")], contents).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = rustok_storefront::router().route("/assets/app.css", get(css_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100").await?;
    println!("Storefront running on http://0.0.0.0:3100");
    axum::serve(listener, app).await?;
    Ok(())
}
