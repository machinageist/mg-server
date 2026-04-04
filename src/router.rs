// All route definitions owned by this file

use axum::{Router, routing::get, response::Html};
use tower_http::services::ServeDir;
// use crate::handlers::pages;

pub fn build() -> Router {
    Router::new()
        .route("/", get(hello))
        // .route("/", get(pages::home))
        // .route("/about", get(pages::about))
        // .route("/portfolio", get(pages::portfolio))
        // Blog routes goes here
        // Static file serving goes here
        .nest_service("/static", ServeDir::new("static"))
}

async fn hello() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>mg-server</title>
    <link  rel="stylesheet" href="/static/css/style.css">
</head>
<body>
    <h1>mg-server</h1>
    <p>Step 2: static files loading.</p>
    <script src="/static/js/main.js"></script>
</body>
</html>
    "#)
}
