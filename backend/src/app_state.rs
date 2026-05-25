use std::sync::Arc;

use sqlx::SqlitePool;

use crate::mailer::Mailer;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
    pub mailer: Arc<Mailer>,
}
