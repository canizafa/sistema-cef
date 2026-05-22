use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Membresia;
use crate::errors::ApiError;

pub struct MembresiaRepository;

impl MembresiaRepository {
    pub async fn create_membresia(
        State(pool): State<SqlitePool>,
        membresia: Membresia,
    ) -> Result<Membresia, ApiError> {
        todo!()
    }
    pub async fn list_membresias(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Membresia>>, ApiError> {
        todo!()
    }
    pub async fn get_membresia(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Membresia>, ApiError> {
        todo!()
    }
    pub async fn update_membresia(
        State(pool): State<SqlitePool>,
        id: i32,
        membresia: Membresia,
    ) -> Result<Membresia, ApiError> {
        todo!()
    }
    pub async fn delete_membresia(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Membresia, ApiError> {
        todo!()
    }
}
