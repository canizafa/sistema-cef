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
        // sqlx::query_as!(Clase, "INSERT INTO clase (id_clase, dia, horario, cupo_profe, cupo_maximo, estado, descripcion, id_sala, dni_profesor) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", clase.get_id_clase(), clase.get_dia(), clase.get_horario(), clase.get_cupo_profe(), clase.get_cupo_maximo(), clase.get_estado(), clase.get_descripcion(), clase.get_id_sala(), clase.get_dni_profesor())
        //     .execute(&pool)
        //     .await
        //     .map_err(|e| ApiError::DatabaseError(e))
        //     .map(|_| None)
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
