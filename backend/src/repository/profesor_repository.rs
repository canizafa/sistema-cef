use sqlx::SqlitePool;

use crate::{domain::Profesor, errors::ApiError};

pub struct ProfesorRepository;

impl ProfesorRepository {
    pub async fn create_profesor(
        pool: &SqlitePool,
        profesor: &Profesor,
    ) -> Result<Profesor, ApiError> {
        let dni = profesor.get_dni();
        let nombre = profesor.get_nombre_completo();
        let genero = profesor.get_genero().to_string();
        let estado = profesor.get_estado().to_string();

        sqlx::query!(
            r#"
                    INSERT INTO profesor (
                        dni_profesor,
                        nombre,
                        genero,
                        estado
                    )
                    VALUES (?, ?, ?, ?)
                    "#,
            dni,
            nombre,
            genero,
            estado
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(profesor.clone())
    }
    pub async fn get_profesor_by_dni(pool: &SqlitePool, dni: i64) -> Result<Profesor, ApiError> {
        let row = sqlx::query!(
            r#"
                    SELECT
                        dni_profesor as "dni_profesor!",
                        nombre as "nombre!",
                        genero as "genero!",
                        estado as "estado!"
                    FROM profesor
                    WHERE dni_profesor = ?
                    "#,
            dni
        )
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(Profesor::new(
                row.dni_profesor,
                row.nombre,
                crate::domain::Genero::from(row.genero),
                crate::domain::Estado::from(row.estado),
            )),
            None => Err(ApiError::NotFound),
        }
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Profesor>, ApiError> {
        let rows = sqlx::query!(
            r#"
                    SELECT
                        dni_profesor as "dni_profesor!",
                        nombre as "nombre!",
                        genero as "genero!",
                        estado as "estado!"
                    FROM profesor
                    "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Profesor::new(
                    row.dni_profesor,
                    row.nombre,
                    crate::domain::Genero::from(row.genero),
                    crate::domain::Estado::from(row.estado),
                )
            })
            .collect())
    }
    pub async fn update_profesor(
        pool: &SqlitePool,
        dni: i64,
        profesor: &Profesor,
    ) -> Result<Profesor, ApiError> {
        let nombre = profesor.get_nombre_completo();
        let genero = profesor.get_genero().to_string();
        let estado = profesor.get_estado().to_string();

        sqlx::query!(
            r#"
                    UPDATE profesor
                    SET
                        nombre = ?,
                        genero = ?,
                        estado = ?
                    WHERE dni_profesor = ?
                    "#,
            nombre,
            genero,
            estado,
            dni
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_profesor_by_dni(pool, dni).await
    }
    pub async fn delete_profesor(pool: &SqlitePool, dni: i64) -> Result<Profesor, ApiError> {
        let profesor = Self::get_profesor_by_dni(pool, dni).await?;

        sqlx::query!(
            r#"
                    DELETE FROM profesor
                    WHERE dni_profesor = ?
                    "#,
            dni
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;
        Ok(profesor)
    }
}
