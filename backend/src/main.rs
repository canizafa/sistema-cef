mod app_state;
mod auth;
mod domain;
mod dtos;
mod errors;
mod handlers;
mod repository;
mod routes;

use crate::routes::root;
use crate::{app_state::AppState, errors::AppError};
use sqlx::SqlitePool;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();

    let port = 8081;
    let dir = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(dir)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let db = SqlitePool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL no encontrada"))
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    let app_state = AppState { db };

    sqlx::migrate!("./migrations")
        .run(&app_state.db)
        .await
        .map_err(|e| AppError::MigrationError(e))?;

    let app = root::router().with_state(app_state.db);

    axum::serve(listener, app)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(())
}
