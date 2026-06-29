use super::domain::FichaMedica;
use crate::app::errors::DbError;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct FichaMedicaRow {
    pub id_ficha: String,
    pub enfermedades: bool,
    pub operaciones_quirurgicas: bool,
    pub detalles: String,
}
impl From<FichaMedicaRow> for FichaMedica {
    fn from(row: FichaMedicaRow) -> Self {
        FichaMedica::new(
            row.id_ficha,
            row.enfermedades,
            row.operaciones_quirurgicas,
            row.detalles,
        )
    }
}

pub struct FichaMedicaRepository;
impl FichaMedicaRepository {
    pub async fn create(
        pool: &SqlitePool,
        ficha_medica: &FichaMedica,
        id_ficha: &str,
    ) -> Result<FichaMedica, DbError> {
        let row = sqlx::query_as::<_, FichaMedicaRow>(
            r#"
               INSERT INTO ficha_medica (
                   id_ficha,
                   enfermedades,
                   operaciones_quirurgicas,
                   detalles
               )
               VALUES (?, ?, ?, ?)
               RETURNING id_ficha, enfermedades, operaciones_quirurgicas, detalles
               "#,
        )
        .bind(id_ficha)
        .bind(ficha_medica.get_enfermedades())
        .bind(ficha_medica.get_operaciones_quirurgicas())
        .bind(ficha_medica.get_detalles())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn get_by_id(pool: &SqlitePool, id_ficha: &str) -> Result<FichaMedica, DbError> {
        let row = sqlx::query_as::<_, FichaMedicaRow>(
            r#"
                   SELECT
                       id_ficha,
                       enfermedades,
                       operaciones_quirurgicas,
                       detalles
                   FROM ficha_medica
                   WHERE id_ficha = ?
                   "#,
        )
        .bind(id_ficha)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn update(
        pool: &SqlitePool,
        id_ficha: &str,
        ficha_medica: &FichaMedica,
    ) -> Result<FichaMedica, DbError> {
        let row = sqlx::query_as::<_, FichaMedicaRow>(
            r#"
                    UPDATE ficha_medica
                    SET
                        enfermedades = ?,
                        operaciones_quirurgicas = ?,
                        detalles = ?
                    WHERE id_ficha = ?
                    RETURNING id_ficha, enfermedades, operaciones_quirurgicas, detalles
                    "#,
        )
        .bind(ficha_medica.get_enfermedades())
        .bind(ficha_medica.get_operaciones_quirurgicas())
        .bind(ficha_medica.get_detalles())
        .bind(id_ficha)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;
        Ok(row.into())
    }
    pub async fn delete(pool: &SqlitePool, id_ficha: &str) -> Result<(), DbError> {
        sqlx::query!(
            r#"
                    DELETE FROM ficha_medica
                    WHERE id_ficha = ?
                    "#,
            id_ficha
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(())
    }
}
