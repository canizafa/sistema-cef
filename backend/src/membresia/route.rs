use super::handler::*;
use crate::app::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn membresia_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_membresia_handler))
        .route("/get-membresia-dni/{id}", get(get_membresia_by_dni_handler))
        .route("/get-membresia-id/{id}", get(get_membresia_by_id_handler))
        .route("/update-membresia/{id}", put(update_membresia_handler))
        .route("/delete-membresia/{id}", delete(delete_membresia_handler))
        .route("/get-all", get(get_membresias_handler))
}
