use sqlx::SqlitePool;

use crate::{domain::sala::Sala, errors::ApiError};

pub struct SalaRepository;

impl SalaRepository {
    pub async fn create_sala(pool: &SqlitePool, sala: &Sala) -> Result<Sala, ApiError> {
        todo!()
    }
    pub async fn get_sala_by_id(pool: &SqlitePool, id: &str) -> Result<Sala, ApiError> {
        todo!()
    }
    pub async fn get_all_salas(pool: &SqlitePool) -> Result<Vec<Sala>, ApiError> {
        todo!()
    }
    pub async fn update_sala(pool: &SqlitePool, id: &str, sala: &Sala) -> Result<Sala, ApiError> {
        todo!()
    }
    pub async fn delete_sala(pool: &SqlitePool, id: &str) -> Result<Sala, ApiError> {
        todo!()
    }
}
