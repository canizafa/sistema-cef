use sqlx::SqlitePool;

use crate::domain::FichaMedica;
use crate::errors::ApiError;

pub struct FichaMedicaRepository;

impl FichaMedicaRepository {
    pub async fn create_ficha_medica(
        pool: &SqlitePool,
        ficha_medica: FichaMedica,
    ) -> Result<FichaMedica, ApiError> {
        todo!()
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: &str) -> Result<FichaMedica, ApiError> {
        todo!()
    }
    pub async fn update_ficha_medica(
        pool: &SqlitePool,
        dni: &str,
        ficha_medica: FichaMedica,
    ) -> Result<FichaMedica, ApiError> {
        todo!()
    }
    pub async fn delete_ficha_medica(
        pool: &SqlitePool,
        dni: &str,
    ) -> Result<FichaMedica, ApiError> {
        todo!()
    }
}
