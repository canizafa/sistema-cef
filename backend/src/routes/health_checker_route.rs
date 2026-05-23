use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_checker() -> impl IntoResponse {
    (StatusCode::OK, "response is 200: OK")
}
