use crate::errors::ApiError;
use axum::extract::State;
use sqlx::SqlitePool;

pub struct AuthRepository;

impl AuthRepository {
    pub async fn login(
        State(pool): State<SqlitePool>,
        email: String,
        password: String,
    ) -> Result<Option, ApiError> {
        todo!()
    }
    pub async fn register(
        State(pool): State<SqlitePool>,
        email: String,
        password: String,
    ) -> Result<(), ApiError> {
        todo!()
    }
    //pub async fn logout(State(pool): State<SqlitePool>) {}
}
