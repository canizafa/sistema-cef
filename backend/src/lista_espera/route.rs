use crate::app::state::AppState;
use crate::lista_espera::cliente_espera::handler::{
    delete_cliente_lista_espera_handler, get_clientes_lista_espera_handler,
    get_next_cliente_handler,
};
use crate::lista_espera::handler::*;
use axum::Router;
use axum::routing::{delete, get, post};

pub fn lista_espera_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_lista_espera_handler))
        .route("/delete/{id}", delete(delete_lista_espera_handler))
        .route("/get-by-id/{id}", get(get_lista_espera_handler))
        .route("/get-all", get(get_all_lista_espera_handler))
        .route("/insert-user/{id_clase}", post(insert_user))
        .route("/get-clientes", get(get_clientes_lista_espera_handler))
        .route("/get-next", get(get_next_cliente_handler))
        .route("/delete", post(delete_cliente_lista_espera_handler))
}
