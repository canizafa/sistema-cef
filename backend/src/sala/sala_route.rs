use super::*;
use crate::app::AppState;
use axum::Router;
use axum::routing::{delete, get, post, put};

/*
* | POST | `/api/salas/` |
| GET | `/api/salas/` |
| GET | `/api/salas/:id` |
| PUT | `/api/salas/:id` |
| DELETE | `/api/salas/:id` |
*/

pub fn sala_router() -> Router<AppState> {
    Router::new()
        .route("/create/", post(create_sala_handler))
        .route("/get-all/", get(get_salas_handler))
        .route("/get-sala/{id}", get(get_sala_handler))
        .route("/update-sala/{id}", put(update_sala_handler))
        .route("/delete-sala/{id}", delete(delete_sala_handler))
}
