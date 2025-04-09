use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::web::routes::{html::html_routes, api::api_routes};

pub async fn start_web_server() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("src/web/assets"))
        .merge(html_routes())
        .merge(api_routes());

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    info!("Web server started on http://0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}