use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app_state::AppState,
    handlers::{
        create_profesor_handler, delete_profesor_handler, get_profesor_by_dni_handler,
        get_profesores_handler, update_profesor_handler,
    },
};

pub fn profesor_router() -> Router<AppState> {
    Router::new()
        .route("/create-profesor", post(create_profesor_handler))
        .route("/get-profesor/{dni}", get(get_profesor_by_dni_handler))
        .route("/get-all", get(get_profesores_handler))
        .route("/update-profesor/{dni}", post(update_profesor_handler))
        .route("/delete-profesor/{dni}", delete(delete_profesor_handler))
}
