use crate::{actividad::Actividad, app::errors::DbError};
use sqlx::SqlitePool;

pub struct ActividadRepository;

impl ActividadRepository {
    pub async fn create(pool: &SqlitePool, actividad: &Actividad) -> Result<Actividad, DbError> {
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
        .map_err(DbError::from)?;

        Ok(actividad.clone())
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Actividad, DbError> {
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
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(Actividad::new(
            row.id_actividad,
            row.nombre,
            row.descripcion,
        ))
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Actividad>, DbError> {
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
        .map_err(DbError::from)?;

        Ok(rows
            .into_iter()
            .map(|row| Actividad::new(row.id_actividad, row.nombre, row.descripcion))
            .collect())
    }
    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        actividad: &Actividad,
    ) -> Result<Actividad, DbError> {
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
        .map_err(DbError::from)?;

        Self::get_by_id(pool, id).await
    }
    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
        sqlx::query!(
            r#"
                DELETE FROM actividad
                WHERE id_actividad = ?
                "#,
            id
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(())
    }
}
