use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::AppState;
use crate::handlers::{
    create_reserva_handler, delete_reserva_handler, get_reserva_handler, get_reservas_handler,
    update_reserva_handler,
};

pub fn reserva_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_reserva_handler))
        .route("/get-reserva", get(get_reserva_handler))
        .route("/delete-reserva", delete(delete_reserva_handler))
        .route("/update-reserva", put(update_reserva_handler))
        .route("/get-all", get(get_reservas_handler))
}
