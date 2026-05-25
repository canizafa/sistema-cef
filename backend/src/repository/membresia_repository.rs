use sqlx::SqlitePool;

use crate::domain::Membresia;
use crate::errors::ApiError;

pub struct MembresiaRepository;

impl MembresiaRepository {
    pub async fn create_membresia(
        pool: &SqlitePool,
        membresia: &Membresia,
    ) -> Result<Membresia, ApiError> {
        todo!()
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Option<Vec<Membresia>>, ApiError> {
        todo!()
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Membresia>, ApiError> {
        todo!()
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Option<Membresia>, ApiError> {
        todo!()
    }
    pub async fn update_membresia(
        pool: &SqlitePool,
        id: &str,
        membresia: &Membresia,
    ) -> Result<Membresia, ApiError> {
        todo!()
    }
    pub async fn delete_membresia(pool: &SqlitePool, id: &str) -> Result<Membresia, ApiError> {
        todo!()
    }
}
