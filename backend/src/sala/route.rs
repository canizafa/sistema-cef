use super::handler::*;
use crate::app::state::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn sala_router() -> Router<AppState> {
    Router::new()
        .route("/create/", post(create_sala_handler))
        .route("/get-all/", get(get_salas_handler))
        .route("/get-sala/{id}", get(get_sala_handler))
}
