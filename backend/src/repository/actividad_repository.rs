use sqlx::SqlitePool;

use crate::{domain::actividad::Actividad, errors::ApiError};

pub struct ActividadRepository;

impl ActividadRepository {
    pub async fn create_actividad(
        pool: &SqlitePool,
        actividad: &Actividad,
    ) -> Result<Actividad, ApiError> {
        let id = actividad.get_id();
        let nombre = actividad.get_nombre();
        let descripcion = actividad.get_descripcion();

        sqlx::query!(
            r#"
               INSERT INTO actividad (
                   id_actividad,
                   nombre,
                   descripcion
               )
               VALUES (?, ?, ?)
               "#,
            id,
            nombre,
            descripcion
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(actividad.clone())
    }
    pub async fn get_actividad_by_id(pool: &SqlitePool, id: &str) -> Result<Actividad, ApiError> {
        let row = sqlx::query!(
            r#"
               SELECT
                   id_actividad as "id_actividad!",
                   nombre,
                   descripcion
               FROM actividad
               WHERE id_actividad = ?
               "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(Actividad::new(
                row.id_actividad,
                row.nombre,
                row.descripcion,
            )),
            None => Err(ApiError::NotFound),
        }
    }
    pub async fn get_all_actividades(pool: &SqlitePool) -> Result<Vec<Actividad>, ApiError> {
        let rows = sqlx::query!(
            r#"
                SELECT
                    id_actividad as "id_actividad!",
                    nombre,
                    descripcion
                FROM actividad
                "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| Actividad::new(row.id_actividad, row.nombre, row.descripcion))
            .collect())
    }
    pub async fn update_actividad(
        pool: &SqlitePool,
        id: &str,
        actividad: &Actividad,
    ) -> Result<Actividad, ApiError> {
        let nombre = actividad.get_nombre();
        let descripcion = actividad.get_descripcion();

        sqlx::query!(
            r#"
               UPDATE actividad
               SET
                   nombre = ?,
                   descripcion = ?
               WHERE id_actividad = ?
               "#,
            nombre,
            descripcion,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_actividad_by_id(pool, id).await
    }
    pub async fn delete_actividad(pool: &SqlitePool, id: &str) -> Result<Actividad, ApiError> {
        let actividad = Self::get_actividad_by_id(pool, id).await?;

        sqlx::query!(
            r#"
                DELETE FROM actividad
                WHERE id_actividad = ?
                "#,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(actividad)
    }
}
