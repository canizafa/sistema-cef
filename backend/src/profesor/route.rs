use super::handler::*;
use crate::app::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn profesor_router() -> Router<AppState> {
    Router::new()
        .route("/create-profesor", post(create_profesor_handler))
        .route("/get-profesor/{dni}", get(get_profesor_by_dni_handler))
        .route("/get-all", get(get_profesores_handler))
        .route("/update-profesor/{dni}", put(update_profesor_handler))
        .route("/delete-profesor/{dni}", delete(delete_profesor_handler))
}
