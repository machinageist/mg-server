// All route definitions owned by this file

//use axum::{Router, routing::get, response::Html};
use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use crate::handlers::pages;

pub fn build() -> Router {
    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/portfolio", get(pages::portfolio))
        // Blog routes goes here
        .nest_service("/static", ServeDir::new("static"))
}
 
//async fn hello() -> Html<&'static str> {
//    Html(r#"
//    
//        "#)    
//}
