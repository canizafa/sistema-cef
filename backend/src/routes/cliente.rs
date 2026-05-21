use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use sqlx::SqlitePool;

use crate::{models::cliente::Cliente, repository::cliente::crear_cliente};

pub fn clientes_router() -> Router<SqlitePool> {
    Router::new()
        .route("/{id}", get(leer_cliente))
        .route("/", post(registrar_cliente))
}
pub async fn leer_cliente(
    State(pool): State<SqlitePool>,
    Path(cliente_id): Path<i32>,
) -> impl IntoResponse {
}
pub async fn registrar_cliente(
    State(pool): State<SqlitePool>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    StatusCode::ACCEPTED.into_response()
}
