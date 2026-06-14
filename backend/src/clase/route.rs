use super::handler::*;
use crate::app::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn clase_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_clase_handler))
        .route("/get-clase/{id}", get(get_clase_handler))
        .route("/update-clase/{id}", put(update_clase_handler))
        .route("/delete-clase/{id}", delete(delete_clase_handler))
        .route("/get-all", get(get_clases_handler))
}
