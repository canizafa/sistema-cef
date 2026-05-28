use backend::feed_database;
use backend::mailer::Mailer;
use backend::routes::root;
use backend::{app_state::AppState, errors::AppError, telemetry};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    telemetry::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let config = backend::config::Config::from_env()?;
    let dir = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(dir)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let db = SqlitePool::connect(&config.database_url)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&db)
        .await?;

    let mailer = Mailer::new().map_err(|e| AppError::Api(e))?;

    let app_state = AppState {
        db,
        jwt_secret: config.jwt_secret,
        mailer: Arc::new(mailer),
    };

    sqlx::migrate!("./migrations")
        .run(&app_state.db)
        .await
        .map_err(|e| AppError::MigrationError(e))?;

    tracing::info!(port = config.port, "Servidor iniciado");
    feed_database::seed_database(&app_state.db).await?;

    let app = root::router()
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(())
}
