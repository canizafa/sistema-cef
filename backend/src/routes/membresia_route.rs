use crate::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::handlers::{
    create_membresia_handler, delete_membresia_handler, get_membresia_handler,
    get_membresias_handler, update_membresia_handler,
};

pub fn membresia_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_membresia_handler))
        .route("/get-membresia", get(get_membresia_handler))
        .route("/update-membresia", post(update_membresia_handler))
        .route("/delete-membresia", delete(delete_membresia_handler))
        .route("/get-all", get(get_membresias_handler))
}
