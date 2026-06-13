use super::*;
use crate::app::AppState;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn reserva_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_reserva_handler))
        .route("/get-reserva/{id}", get(get_reserva_handler))
        .route("/delete-reserva/{id}", delete(delete_reserva_handler))
        .route("/update-reserva/{id}", put(update_reserva_handler))
        .route("/get-all", get(get_reservas_handler))
}
