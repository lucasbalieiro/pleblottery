use axum::{Json, http::StatusCode, Router};
use crate::config::{PleblotteryConfig, PleblotteryConfigView};


// Serve JSON for /api/config
pub async fn serve_config_json() -> Result<Json<PleblotteryConfigView>, (StatusCode, Json<serde_json::Value>)> {
    match PleblotteryConfig::from_file("./config.toml") {
        Ok(config) => {
            let config_view: PleblotteryConfigView = config.into();
            Ok(Json(config_view))
        }
        Err(err) => {
            let error_message = format!("Failed to load config: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": error_message })),
            ))
        }
    }
}


// Define the router for API routes
pub fn api_routes() -> Router {
    Router::new()
        .route("/api/config", axum::routing::get(serve_config_json))
}