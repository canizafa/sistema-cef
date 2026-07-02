use super::mailer::Mailer;
use sqlx::SqlitePool;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
    pub mailer: Arc<Mailer>,
    pub dias_gracia: Arc<Mutex<i64>>,
}