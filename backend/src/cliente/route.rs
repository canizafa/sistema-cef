use super::*;
use crate::app::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn cliente_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_cliente_handler))
        .route("/get-cliente/{id}", get(get_cliente_handler))
        .route("/update-cliente/{id}", put(update_cliente_handler))
        .route("/delete-cliente/{id}", delete(delete_cliente_handler))
        .route("/get-all", get(get_clientes_handler))
}
