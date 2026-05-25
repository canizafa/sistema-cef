use crate::app_state::AppState;
use crate::handlers::{
    create_pago_handler, delete_pago_handler, get_pago_handler, get_pagos_handler,
    update_pago_handler,
};
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn pago_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_pago_handler))
        .route("/get-pago", get(get_pago_handler))
        .route("/delete-pago", delete(delete_pago_handler))
        .route("/update-pago", put(update_pago_handler))
        .route("/get-all", get(get_pagos_handler))
}
