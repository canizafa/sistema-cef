use axum::{Router, response::IntoResponse, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(traer))
}

async fn traer() -> impl IntoResponse {
    String::from("holamundo")
}
