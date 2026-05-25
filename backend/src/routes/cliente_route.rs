use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    app_state::AppState,
    handlers::{
        create_cliente_handler, delete_cliente_handler, get_cliente_handler, get_clientes_handler,
        update_cliente_handler,
    },
};

pub fn cliente_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_cliente_handler))
        .route("/get-cliente/{id}", get(get_cliente_handler))
        .route("/update-cliente/{id}", post(update_cliente_handler))
        .route("/delete-cliente/{id}", post(delete_cliente_handler))
        .route("/get-all", get(get_clientes_handler))
}
