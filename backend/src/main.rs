use backend::errors::ApiError;
use backend::mailer::Mailer;
use backend::routes::root;
use backend::{app_state::AppState, errors::AppError};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    let config = backend::config::Config::from_env()?;
    let port = 8081;
    let dir = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(dir)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let db = SqlitePool::connect(&config.database_url)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    let mailer = Mailer::new()?;

    let app_state = AppState {
        db,
        jwt_secret: config.jwt_secret,
        mailer: Arc::new(mailer),
    };

    sqlx::migrate!("./migrations")
        .run(&app_state.db)
        .await
        .map_err(|e| AppError::MigrationError(e))?;

    let app = root::router().with_state(app_state);

    axum::serve(listener, app)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(())
}
