use sqlx::SqlitePool;

use crate::domain::FichaMedica;
use crate::errors::ApiError;

pub struct FichaMedicaRepository;

impl FichaMedicaRepository {
    pub async fn create_ficha_medica(
        pool: &SqlitePool,
        ficha_medica: FichaMedica,
    ) -> Result<FichaMedica, ApiError> {
        let id_ficha = ficha_medica.get_id_ficha();
        let enfermedades = ficha_medica.get_enfermedades();
        let operaciones = ficha_medica.get_operaciones_quirurgicas();
        let detalles = ficha_medica.get_detalles();

        sqlx::query!(
            r#"
                   INSERT INTO ficha_medica (
                       id_ficha,
                       enfermedades,
                       operaciones_quirurgicas,
                       detalles
                   )
                   VALUES (?, ?, ?, ?)
                   "#,
            id_ficha,
            enfermedades,
            operaciones,
            detalles
        )
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(ficha_medica)
    }
    pub async fn get_by_id(pool: &SqlitePool, id_ficha: &str) -> Result<FichaMedica, ApiError> {
        let row = sqlx::query!(
            r#"
                   SELECT
                       id_ficha as "id_ficha!",
                       enfermedades as "enfermedades!",
                       operaciones_quirurgicas as "operaciones_quirurgicas!",
                       detalles as "detalles!"
                   FROM ficha_medica
                   WHERE id_ficha = ?
                   "#,
            id_ficha
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(FichaMedica::new(
            row.id_ficha,
            row.enfermedades,
            row.operaciones_quirurgicas,
            row.detalles,
        ))
    }
    pub async fn update_ficha_medica(
        pool: &SqlitePool,
        id_ficha: &str,
        ficha_medica: FichaMedica,
    ) -> Result<FichaMedica, ApiError> {
        let enfermedades = ficha_medica.get_enfermedades();
        let operaciones = ficha_medica.get_operaciones_quirurgicas();
        let detalles = ficha_medica.get_detalles();

        sqlx::query!(
            r#"
                    UPDATE ficha_medica
                    SET
                        enfermedades = ?,
                        operaciones_quirurgicas = ?,
                        detalles = ?
                    WHERE id_ficha = ?
                    "#,
            enfermedades,
            operaciones,
            detalles,
            id_ficha
        )
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Self::get_by_id(pool, id_ficha).await
    }
    pub async fn delete_ficha_medica(
        pool: &SqlitePool,
        id_ficha: &str,
    ) -> Result<FichaMedica, ApiError> {
        let ficha = Self::get_by_id(pool, id_ficha).await?;

        sqlx::query!(
            r#"
                    DELETE FROM ficha_medica
                    WHERE id_ficha = ?
                    "#,
            id_ficha
        )
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(ficha)
    }
}
