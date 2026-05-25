use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app_state::AppState,
    handlers::{
        create_membresia_handler, delete_membresia_handler, get_membresia_by_dni_handler,
        get_membresia_by_id_handler, get_membresias_handler, update_membresia_handler,
    },
};

pub fn membresia_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_membresia_handler))
        .route("/get-membresia-dni/{id}", get(get_membresia_by_dni_handler))
        .route("/get-membresia-id/{id}", get(get_membresia_by_id_handler))
        .route("/update-membresia/{id}", post(update_membresia_handler))
        .route("/delete-membresia/{id}", delete(delete_membresia_handler))
        .route("/get-all", get(get_membresias_handler))
}
