use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{
    create_clase_handler, delete_clase_handler, get_clase_handler, get_clases_handler,
    update_clase_handler,
};

pub fn clase_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_clase_handler))
        .route("/get-clase", get(get_clase_handler))
        .route("/update-clase", post(update_clase_handler))
        .route("/delete-clase", post(delete_clase_handler))
        .route("/get-all", get(get_clases_handler))
}
