use axum::response::IntoResponse;

pub async fn health_checker() -> impl IntoResponse {
    String::from("response is 200: OK")
}
