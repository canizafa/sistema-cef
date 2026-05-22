use crate::errors::ApiError;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Clase;

pub struct ClaseRepository;

impl ClaseRepository {
    pub async fn create_clase(
        State(pool): State<SqlitePool>,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
        todo!()
    }
    pub async fn list_clases(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Clase>>, ApiError> {
        todo!()
    }
    pub async fn get_by_id(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        todo!()
    }
    pub async fn update_clase(
        State(pool): State<SqlitePool>,
        id: i32,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
        todo!()
    }
    pub async fn delete_clase(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        todo!()
    }
}
