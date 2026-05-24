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
        sqlx::query_as!(
            Clase,
            "INSERT INTO clase
            (dia, horario, cupo_profe, cupo_maximo, estado, descripcion, id_sala, dni_profesor)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
                dni_profesor",
            clase.get_dia(),
            clase.get_horario(),
            clase.get_cupo_profe(),
            clase.get_cupo_maximo(),
            clase.get_estado(),
            clase.get_descripcion(),
            clase.get_id_sala(),
            clase.get_dni_profesor()
        )
        .fetch_optional(&pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    pub async fn list_clases(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Clase>>, ApiError> {
        sqlx::query_as!(
            Clase,
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
                dni_profesor
            FROM clase"
        )
        .fetch_all(&pool)
        .await
        .map_err(ApiError::DatabaseError)
        .map(Some)
    }

    pub async fn get_by_id(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(
            Clase,
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
                dni_profesor
            FROM clase
            WHERE id_clase = ?",
            id
        )
        .fetch_optional(&pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    pub async fn update_clase(
        State(pool): State<SqlitePool>,
        id: i32,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(
            Clase,
            "UPDATE clase
            SET
                dia = ?,
                horario = ?,
                cupo_profe = ?,
                cupo_maximo = ?,
                estado = ?,
                descripcion = ?,
                id_sala = ?,
                dni_profesor = ?
            WHERE id_clase = ?
            RETURNING
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
                dni_profesor",
            clase.get_dia(),
            clase.get_horario(),
            clase.get_cupo_profe(),
            clase.get_cupo_maximo(),
            clase.get_estado(),
            clase.get_descripcion(),
            clase.get_id_sala(),
            clase.get_dni_profesor(),
            id
        )
        .fetch_optional(&pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    pub async fn delete_clase(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(
            Clase,
            "DELETE FROM clase
            WHERE id_clase = ?
            RETURNING
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
                dni_profesor",
            id
        )
        .fetch_optional(&pool)
        .await
        .map_err(ApiError::DatabaseError)
    }
}
