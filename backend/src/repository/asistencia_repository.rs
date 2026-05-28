use chrono::NaiveDate;
use sqlx::SqlitePool;

use crate::{domain::asistencia::Asistencia, errors::ApiError};

pub struct AsistenciaRepository;
impl AsistenciaRepository {
    pub async fn create_asistencia(
        pool: &SqlitePool,
        asistencia: &Asistencia,
    ) -> Result<Asistencia, ApiError> {
        let id = asistencia.get_id_asistencia();
        let fecha = asistencia.get_fecha().format("%Y-%m-%d").to_string();
        let metodo = asistencia.get_metodo();
        let id_reserva = asistencia.get_id_clase();

        sqlx::query!(
            r#"
                   INSERT INTO asistencia (
                       id_asistencia,
                       fecha,
                       metodo,
                       id_reserva
                   )
                   VALUES (?, ?, ?, ?)
                   "#,
            id,
            fecha,
            metodo,
            id_reserva
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(asistencia.clone())
    }
    pub async fn get_asistencia_by_id(pool: &SqlitePool, id: &str) -> Result<Asistencia, ApiError> {
        let row = sqlx::query!(
            r#"
                    SELECT
                        id_asistencia as "id_asistencia!",
                        fecha as "fecha!",
                        metodo as "metodo!",
                        id_reserva as "id_reserva!"
                    FROM asistencia
                    WHERE id_asistencia = ?
                    "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(Asistencia::new(
            row.id_asistencia,
            NaiveDate::parse_from_str(&row.fecha, "%Y-%m-%d").unwrap_or_default(),
            row.metodo,
            row.id_reserva,
            Vec::new(),
        ))
    }
    pub async fn get_all_asistencias(pool: &SqlitePool) -> Result<Vec<Asistencia>, ApiError> {
        let rows = sqlx::query!(
            r#"
                    SELECT
                        id_asistencia as "id_asistencia!",
                        fecha as "fecha!",
                        metodo as "metodo!",
                        id_reserva as "id_reserva!"
                    FROM asistencia
                    "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Asistencia::new(
                    row.id_asistencia,
                    NaiveDate::parse_from_str(&row.fecha, "%Y-%m-%d").unwrap_or_default(),
                    row.metodo,
                    row.id_reserva,
                    Vec::new(),
                )
            })
            .collect())
    }
    pub async fn update_asistencia(
        pool: &SqlitePool,
        id: &str,
        asistencia: &Asistencia,
    ) -> Result<Asistencia, ApiError> {
        let fecha = asistencia.get_fecha().format("%Y-%m-%d").to_string();
        let metodo = asistencia.get_metodo();
        let id_reserva = asistencia.get_id_clase();

        sqlx::query!(
            r#"
                    UPDATE asistencia
                    SET
                        fecha = ?,
                        metodo = ?,
                        id_reserva = ?
                    WHERE id_asistencia = ?
                    "#,
            fecha,
            metodo,
            id_reserva,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_asistencia_by_id(pool, id).await
    }
    pub async fn delete_asistencia(pool: &SqlitePool, id: &str) -> Result<Asistencia, ApiError> {
        let asistencia = Self::get_asistencia_by_id(pool, id).await?;

        sqlx::query!(
            r#"
                    DELETE FROM asistencia
                    WHERE id_asistencia = ?
                    "#,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(asistencia)
    }
}
