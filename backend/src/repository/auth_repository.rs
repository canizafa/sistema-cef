use axum::extract::State;
use sqlx::SqlitePool;

pub struct AuthRepository;

impl AuthRepository {
    pub async fn login(State(pool): State<SqlitePool>, email: String, password: String) {}
    pub async fn register(State(pool): State<SqlitePool>, email: String, password: String) {}
    //pub async fn logout(State(pool): State<SqlitePool>) {}
}
