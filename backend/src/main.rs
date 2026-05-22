mod app_state;
mod auth;
mod domain;
mod dtos;
mod errores;
mod handlers;
mod repository;
mod routes;

use crate::app_state::AppState;
use crate::routes::root;
use sqlx::SqlitePool;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = 8081;
    let dir = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(dir)
        .await
        .unwrap_or_else(|e| {
            panic!("fallo la conetsion de la dir {}:{}", dir, e);
        });

    let db = SqlitePool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL no encontrada"))
        .await
        .expect("No se pudo conectar a la base de datos");

    let app_state = AppState { db };

    sqlx::migrate!("./migrations")
        .run(&app_state.db)
        .await
        .unwrap();

    let app = root::router().with_state(app_state.db);

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        panic!("falló la conexión con el servidor {}", e);
    });
}
