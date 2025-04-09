use axum::{Router, response::Html};

// Serve the HTML page for /
pub async fn serve_index() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PlebLottery</title>
        <script src="https://cdn.tailwindcss.com/3.4.16"></script>
    </head>
    <body class="bg-gray-100 text-gray-800">
        <header class="bg-blue-600 text-white py-4">
            <div class="container mx-auto flex justify-between items-center">
                <h1 class="text-2xl font-bold">PlebLottery</h1>
            </div>
            <nav class="bg-blue-700 text-white py-2">
                <div class="container mx-auto flex space-x-4">
                    <a href="/" class="hover:underline">Home</a>
                    <a href="/config" class="hover:underline">Configuration</a>
                </div>
            </nav>
        </header>
        <main>
            <h2 class="text-xl font-bold mb-4">Welcome to PlebLottery</h2>
            <p>This is a dynamic HTML page rendered using htmx!</p>
            <img src="/static/images/pleblottery.png" alt="Pleblottery Logo" class="mt-4">
        </main>
        <footer>
            <p>&copy; 2025 PlebLottery</p>
        </footer>
    </body>
    </html>
    "#)
}

// Serve the HTML page for /config
pub async fn serve_config_html() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PlebLottery - Configuration</title>
        <script src="https://cdn.tailwindcss.com/3.4.16"></script>
        <script src="https://unpkg.com/htmx.org"></script>
    </head>
    <body class="bg-gray-100 text-gray-800">
        <header class="bg-blue-600 text-white py-4">
            <div class="container mx-auto flex justify-between items-center">
                <h1 class="text-2xl font-bold">PlebLottery</h1>
            </div>
            <nav class="bg-blue-700 text-white py-2">
                <div class="container mx-auto flex space-x-4">
                    <a href="/" class="hover:underline">Home</a>
                    <a href="/config" class="hover:underline">Configuration</a>
                </div>
            </nav>
        </header>
        <main>
            <h2 class="text-xl font-bold mb-4">Configuration</h2>
            <p class="mb-4">Below is the current configuration:</p>
            <div id="config-container" class="mt-4 bg-white p-4 rounded shadow">
                <table class="table-auto w-full border-collapse border border-gray-300">
                    <thead>
                        <tr class="bg-gray-100">
                            <th class="border border-gray-300 px-4 py-2">Key</th>
                            <th class="border border-gray-300 px-4 py-2">Value</th>
                            <th class="border border-gray-300 px-4 py-2">Description</th>
                        </tr>
                    </thead>
                    <tbody hx-get="/api/config" hx-trigger="load" hx-target="this">
                        <!-- Rows will be dynamically loaded here -->
                    </tbody>
                </table>
            </div>
        </main>
        <footer>
            <p>&copy; 2025 PlebLottery</p>
        </footer>
    </body>
    </html>
    "#)
}

// Define the router for HTML routes
pub fn html_routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(serve_index))
        .route("/config", axum::routing::get(serve_config_html))
}