use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    #[allow(dead_code)]
    path: PathBuf,
}
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("serve http server at port: {}, with path: {:?}", port, path);

    let shared_state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    // axum router
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(shared_state));

    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(shared_data): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    format!(
        "Hello, the path is {:?}, current access url:{:?}",
        shared_data, path
    );
    let p = std::path::Path::new(&shared_data.path).join(path);
    info!("access path: {:?}", p);
    if !p.exists() {
        info!("file not found: {:?}", p);
        (
            StatusCode::NOT_FOUND,
            format!("File {:?} Not Found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(f) => {
                info!("read bytes length: {:?}", f.len());
                (StatusCode::OK, f)
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Read file error: {:?}", e),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::process::http_serve::file_handler;
    use axum::extract::State;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_file_handler() {
        // todo
        let state = super::HttpServeState {
            path: std::path::PathBuf::from("./"),
        };
        let path = axum::extract::Path("Cargo.toml".to_string());
        let (status_code, content) = file_handler(State(Arc::new(state)), path).await;
        assert_eq!(status_code, 200);
        println!("{:?}", content);
    }
}
