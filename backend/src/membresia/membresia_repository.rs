use super::*;
use crate::app::ApiError;
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub struct MembresiaRepository;

impl MembresiaRepository {
    pub async fn create_membresia(
        pool: &SqlitePool,
        membresia: &Membresia,
    ) -> Result<Membresia, ApiError> {
        let id = membresia.get_id_membresia();
        let tipo = membresia.get_tipo();
        let estado = membresia.get_estado().to_string();

        let fecha_inicio = membresia.get_fecha_inicio().format("%Y-%m-%d").to_string();

        let fecha_fin = membresia
            .get_fecha_fin()
            .map(|f| f.format("%Y-%m-%d").to_string());

        sqlx::query!(
            r#"
                   INSERT INTO membresias (
                       id_membresia,
                       tipo,
                       estado,
                       fecha_inicio,
                       fecha_fin
                   )
                   VALUES (?, ?, ?, ?, ?)
                   "#,
            id,
            tipo,
            estado,
            fecha_inicio,
            fecha_fin
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(membresia.clone())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Membresia>, ApiError> {
        let rows = sqlx::query!(
            r#"
                    SELECT
                        id_membresia as "id_membresia!",
                        tipo as "tipo!",
                        estado as "estado!",
                        fecha_inicio as "fecha_inicio!",
                        fecha_fin
                    FROM membresias
                    "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Membresia::new(
                    row.id_membresia,
                    row.tipo,
                    crate::domain::Estado::from(row.estado),
                    NaiveDate::parse_from_str(&row.fecha_inicio, "%Y-%m-%d").unwrap_or_default(),
                    row.fecha_fin.map(|f: String| {
                        NaiveDate::parse_from_str(&f, "%Y-%m-%d").unwrap_or_default()
                    }),
                )
            })
            .collect())
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Membresia, ApiError> {
        let row = sqlx::query!(
            r#"
                    SELECT
                        id_membresia as "id_membresia!",
                        tipo as "tipo!",
                        estado as "estado!",
                        fecha_inicio as "fecha_inicio!",
                        fecha_fin
                    FROM membresias
                    WHERE id_membresia = ?
                    "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(Membresia::new(
            row.id_membresia,
            row.tipo,
            crate::domain::Estado::from(row.estado),
            NaiveDate::parse_from_str(&row.fecha_inicio, "%Y-%m-%d").unwrap_or_default(),
            row.fecha_fin
                .map(|f: String| NaiveDate::parse_from_str(&f, "%Y-%m-%d").unwrap_or_default()),
        ))
    }
    pub async fn update_membresia(
        pool: &SqlitePool,
        id: &str,
        membresia: &Membresia,
    ) -> Result<Membresia, ApiError> {
        let tipo = membresia.get_tipo();
        let estado = membresia.get_estado().to_string();

        let fecha_inicio = membresia.get_fecha_inicio().format("%Y-%m-%d").to_string();

        let fecha_fin = membresia
            .get_fecha_fin()
            .map(|f| f.format("%Y-%m-%d").to_string());

        sqlx::query!(
            r#"
                    UPDATE membresias
                    SET
                        tipo = ?,
                        estado = ?,
                        fecha_inicio = ?,
                        fecha_fin = ?
                    WHERE id_membresia = ?
                    "#,
            tipo,
            estado,
            fecha_inicio,
            fecha_fin,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_by_id(pool, id).await
    }
    pub async fn delete_membresia(pool: &SqlitePool, id: &str) -> Result<Membresia, ApiError> {
        let membresia = Self::get_by_id(pool, id).await?;

        sqlx::query!(
            r#"
                    DELETE FROM membresias
                    WHERE id_membresia = ?
                    "#,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(membresia)
    }
}
