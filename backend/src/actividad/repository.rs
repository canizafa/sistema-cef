use crate::{actividad::Actividad, app::errors::DbError};
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct ActividadRow {
    id_actividad: String,
    nombre: String,
    descripcion: String,
}
impl From<ActividadRow> for Actividad {
    fn from(row: ActividadRow) -> Self {
        Self::new(row.id_actividad, row.nombre, row.descripcion)
    }
}

pub struct ActividadRepository;
impl ActividadRepository {
    pub async fn create(pool: &SqlitePool, actividad: &Actividad) -> Result<Actividad, DbError> {
        let row = sqlx::query_as::<_, ActividadRow>(
            r#"
               INSERT INTO actividad (
                   id_actividad,
                   nombre,
                   descripcion
               )
               VALUES (?, ?, ?)
               "#,
        )
        .bind(actividad.get_id())
        .bind(actividad.get_nombre())
        .bind(actividad.get_descripcion())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Actividad, DbError> {
        let row = sqlx::query_as::<_, ActividadRow>(
            r#"
               SELECT
                   id_actividad as "id_actividad!",
                   nombre,
                   descripcion
               FROM actividad
               WHERE id_actividad = ?
               "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Actividad>, DbError> {
        let rows = sqlx::query_as::<_, ActividadRow>(
            r#"
                SELECT
                    id_actividad as "id_actividad!",
                    nombre,
                    descripcion
                FROM actividad
                "#,
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Actividad::from).collect())
    }
    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        actividad: &Actividad,
    ) -> Result<Actividad, DbError> {
        let row = sqlx::query_as::<_, ActividadRow>(
            r#"
               UPDATE actividad
               SET
                   nombre = ?,
                   descripcion = ?
               WHERE id_actividad = ?
               "#,
        )
        .bind(actividad.get_nombre())
        .bind(actividad.get_descripcion())
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
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
