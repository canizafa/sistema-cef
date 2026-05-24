use crate::errors::ApiError;
use crate::domain::{Clase, Estado};
use sqlx::SqlitePool;

pub struct ClaseRepository;

impl ClaseRepository {
    pub async fn create_clase(pool: &SqlitePool, clase: Clase) -> Result<Clase, ApiError> {
        sqlx::query!(
            "INSERT INTO clase
               (
                   id_clase,
                   dia,
                   horario,
                   cupo_profe,
                   cupo_maximo,
                   estado,
                   descripcion,
                   id_actividad,
                   id_sala,
                   dni_profesor
               )
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            clase.get_id(),
            clase.get_dia(),
            clase.get_horario(),
            clase.get_cupo_profe(),
            clase.get_cupo_maximo(),
            clase.get_estado(),
            clase.get_descripcion(),
            clase.get_id_actividad(),
            clase.get_id_sala(),
            clase.get_dni_profesor()
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;
        
        Ok(clase)
    }

    pub async fn list_clases(pool: &SqlitePool) -> Result<Vec<Clase>, ApiError> {
        let rows = sqlx::query!(
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_actividad,
                id_sala,
                dni_profesor
            FROM clase"
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let clases: Result<Vec<Clase>, ApiError> = rows.into_iter().map(|row| {
            Ok(Clase::new(
                row.id_clase,
                row.dia,
                row.horario,
                row.descripcion,
                row.cupo_profe,
                row.cupo_maximo,
                Estado::from(row.estado),
                row.id_sala,
                row.dni_profesor,
                row.id_actividad,
            ))
        }).collect();

        clases
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Clase>, ApiError> {
        let row = sqlx::query!(
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_actividad,
                id_sala,
                dni_profesor
            FROM clase
            WHERE id_clase = ?",
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(Some(Clase::new(
                row.id_clase,
                row.dia,
                row.horario,
                row.descripcion,
                row.cupo_profe,
                row.cupo_maximo,
                Estado::from(row.estado),
                row.id_sala,
                row.dni_profesor,
                row.id_actividad,
            ))),
            None => Ok(None),
        }
    }

    pub async fn update_clase(
        pool: &SqlitePool,
        id: &str,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query!(
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
            WHERE id_clase = ?",
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
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_by_id(pool, id).await
    }

    pub async fn delete_clase(pool: &SqlitePool, id: &str) -> Result<Option<Clase>, ApiError> {
        let clase = Self::get_by_id(pool, id).await?;
        
        sqlx::query!(
            "DELETE FROM clase WHERE id_clase = ?",
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(clase)
    }
}
