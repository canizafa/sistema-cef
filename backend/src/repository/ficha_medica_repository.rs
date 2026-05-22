use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::FichaMedica;
use crate::errors::ApiError;

pub struct FichaMedicaRepository;

impl FichaMedicaRepository {
    pub async fn create_ficha_medica(
        State(pool): State<SqlitePool>,
        ficha_medica: FichaMedica,
    ) -> Result<Option<FichaMedica>, ApiError> {
        todo!()
    }
    pub async fn get_by_dni(
        State(pool): State<SqlitePool>,
        dni: i32,
    ) -> Result<Option<FichaMedica>, ApiError> {
        todo!()
    }
    pub async fn update_ficha_medica(
        State(pool): State<SqlitePool>,
        dni: i32,
        ficha_medica: FichaMedica,
    ) -> Result<Option<FichaMedica>, ApiError> {
        todo!()
    }
    pub async fn delete_ficha_medica(
        State(pool): State<SqlitePool>,
        dni: i32,
    ) -> Result<Option<FichaMedica>, ApiError> {
        todo!()
    }
}
