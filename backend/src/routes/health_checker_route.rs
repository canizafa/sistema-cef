use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

#[instrument(name = "http.health", skip_all)]
pub async fn health_checker() -> impl IntoResponse {
    (StatusCode::OK, "response is 200: OK")
}
