use super::handler::*;
use crate::app::state::AppState;
use axum::{Router, routing::get};

pub fn estadistica_router() -> Router<AppState> {
    Router::new()
        .route(
            "/get-clase-mas-concurrida",
            get(clase_mas_concurrida_handler),
        )
        .route("/get-clase-mas-cancelada", get(clase_mas_cancelada_handler))
        .route("/get-recaudacion", get(recaudacion_handler))
}
