use axum::{Router, response::Html};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    message: String,
}

#[derive(Template)]
#[template(path = "config.html")]
struct ConfigPageTemplate;

// Serve the HTML page for /
pub async fn serve_index() -> Html<String> {
    let template = IndexTemplate {
        title: "Welcome to PlebLottery".to_string(),
        message: "This is a dynamic HTML page rendered using Askama!".to_string(),
    };
    Html(template.render().unwrap())
}

// Serve the HTML page for /config
pub async fn serve_config_html() -> Html<String> {
    let template = ConfigPageTemplate;
    Html(template.render().unwrap())
}

// Define the router for HTML routes
pub fn html_routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(serve_index))
        .route("/config", axum::routing::get(serve_config_html))
}