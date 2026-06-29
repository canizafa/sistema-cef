use super::handler::*;
use crate::app::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn cliente_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_cliente_handler))
        .route("/get-cliente/{id}", get(get_cliente_handler))
        .route("/update-cliente", put(update_cliente_handler))
        .route("/update-password/", put(update_password_handler))
        .route("/reset-password/{email}", put(reset_password_handler))
        .route("/delete-cliente", delete(delete_cliente_handler))
        .route("/get-all", get(get_clientes_handler))
}
