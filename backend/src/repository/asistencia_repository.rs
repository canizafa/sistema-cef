use sqlx::SqlitePool;

use crate::{domain::asistencia::Asistencia, errors::ApiError};

pub struct AsistenciaRepository;

impl AsistenciaRepository {
    pub async fn create_asistencia(
        pool: &SqlitePool,
        asistencia: &Asistencia,
    ) -> Result<Asistencia, ApiError> {
        todo!()
    }
    pub async fn get_asistencia_by_id(pool: &SqlitePool, id: &str) -> Result<Asistencia, ApiError> {
        todo!()
    }
    pub async fn get_all_asistencias(pool: &SqlitePool) -> Result<Vec<Asistencia>, ApiError> {
        todo!()
    }
    pub async fn update_asistencia(
        pool: &SqlitePool,
        id: &str,
        asistencia: &Asistencia,
    ) -> Result<Asistencia, ApiError> {
        todo!()
    }
    pub async fn delete_asistencia(pool: &SqlitePool, id: &str) -> Result<Asistencia, ApiError> {
        todo!()
    }
}
