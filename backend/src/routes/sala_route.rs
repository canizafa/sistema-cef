use axum::Router;

use axum::routing::{delete, get, post, put};

use crate::app_state::AppState;
use crate::handlers::sala_handler::{
    create_sala_handler, delete_sala_handler, get_sala_handler, get_salas_handler,
    update_sala_handler,
};

pub fn sala_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_sala_handler))
        .route("/", get(get_salas_handler))
        .route("/:id", get(get_sala_handler))
        .route("/:id", put(update_sala_handler))
        .route("/:id", delete(delete_sala_handler))
}
