use sqlx::SqlitePool;

use crate::{domain::Profesor, errors::ApiError};

pub struct ProfesorRepository;

impl ProfesorRepository {
    pub async fn create_profesor(
        pool: &SqlitePool,
        profesor: &Profesor,
    ) -> Result<Profesor, ApiError> {
        todo!()
    }
    pub async fn get_profesor_by_dni(pool: &SqlitePool, id: &str) -> Result<Profesor, ApiError> {
        todo!()
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Profesor>, ApiError> {
        todo!()
    }
    pub async fn update_profesor(
        pool: &SqlitePool,
        id: &str,
        profesor: &Profesor,
    ) -> Result<Profesor, ApiError> {
        todo!()
    }
    pub async fn delete_profesor(pool: &SqlitePool, id: &str) -> Result<Profesor, ApiError> {
        todo!()
    }
}
