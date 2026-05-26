use sqlx::SqlitePool;

use crate::{domain::actividad::Actividad, errors::ApiError};

pub struct ActividadRepository;

impl ActividadRepository {
    pub async fn create_actividad(
        pool: &SqlitePool,
        actividad: &Actividad,
    ) -> Result<Actividad, ApiError> {
        todo!()
    }
    pub async fn get_actividad_by_id(pool: &SqlitePool, id: &str) -> Result<Actividad, ApiError> {
        todo!()
    }
    pub async fn get_all_actividades(pool: &SqlitePool) -> Result<Vec<Actividad>, ApiError> {
        todo!()
    }
    pub async fn update_actividad(
        pool: &SqlitePool,
        id: &str,
        actividad: &Actividad,
    ) -> Result<Actividad, ApiError> {
        todo!()
    }
    pub async fn delete_actividad(pool: &SqlitePool, id: &str) -> Result<Actividad, ApiError> {
        todo!()
    }
}
