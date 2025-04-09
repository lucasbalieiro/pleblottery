use axum::{response::Html, Router};
use crate::config::PleblotteryConfig;

pub async fn serve_config_htmx() -> Html<String> {
    match PleblotteryConfig::from_file("./config.toml") {
        Ok(config) => {
            let rows = vec![
                // Mining Server Config
                format!(
                    r#"<tr class="hover:bg-gray-100">
                        <td class="border px-4 py-2 font-bold">Listening Port</td>
                        <td class="border px-4 py-2">{}</td>
                        <td class="border px-4 py-2">Port the server listens on</td>
                    </tr>"#,
                    config.mining_server_config.listening_port
                ),
                format!(
                    r#"<tr class="hover:bg-gray-100">
                        <td class="border px-4 py-2 font-bold">Public Key</td>
                        <td class="border px-4 py-2">{}</td>
                        <td class="border px-4 py-2">Public key for the server</td>
                    </tr>"#,
                    config.mining_server_config.pub_key
                ),
                format!(
                    r#"<tr class="hover:bg-gray-100">
                        <td class="border px-4 py-2 font-bold">Private Key</td>
                        <td class="border px-4 py-2">[REDACTED]</td>
                        <td class="border px-4 py-2">Private key for the server (hidden for security)</td>
                    </tr>"#
                ),
                format!(
                    r#"<tr class="hover:bg-gray-100">
                        <td class="border px-4 py-2 font-bold">Certificate Validity</td>
                        <td class="border px-4 py-2">{}</td>
                        <td class="border px-4 py-2">Certificate validity in seconds</td>
                    </tr>"#,
                    config.mining_server_config.cert_validity
                ),
                format!(
                    r#"<tr class="hover:bg-gray-100">
                        <td class="border px-4 py-2 font-bold">Inactivity Limit</td>
                        <td class="border px-4 py-2">{}</td>
                        <td class="border px-4 py-2">Inactivity timeout in seconds</td>
                    </tr>"#,
                    config.mining_server_config.inactivity_limit
                ),
                // Template Distribution Config
                format!(
                    r#"<tr class="hover:bg-gray-100">
                        <td class="border px-4 py-2 font-bold">Server Address</td>
                        <td class="border px-4 py-2">{}</td>
                        <td class="border px-4 py-2">Address of the template distribution server</td>
                    </tr>"#,
                    config.template_distribution_config.server_addr
                ),
                format!(
                    r#"<tr class="hover:bg-gray-100">
                        <td class="border px-4 py-2 font-bold">Auth Public Key</td>
                        <td class="border px-4 py-2">{}</td>
                        <td class="border px-4 py-2">Authentication public key (optional)</td>
                    </tr>"#,
                    config.template_distribution_config
                        .auth_pk
                        .as_ref()
                        .map(|key| key.to_string())
                        .unwrap_or_else(|| "None".to_string())
                ),
            ];

            // Return the rows as the response
            Html(rows.join(""))
        }
        Err(_) => Html(
            r#"<tr class="bg-red-100">
                <td colspan="3" class="border px-4 py-2 text-red-600">Error loading configuration</td>
            </tr>"#
            .to_string(),
        ),
    }
}

// Define the router for API routes
pub fn api_routes() -> Router {
    Router::new()
        .route("/api/config", axum::routing::get(serve_config_htmx))
}