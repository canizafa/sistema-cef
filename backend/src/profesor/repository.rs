use crate::{
    app::{
        errors::DbError,
        rol::{Estado, Genero},
    },
    profesor::domain::Profesor,
};
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct ProfesorRow {
    dni_profesor: i64,
    nombre: String,
    genero: Genero,
    estado: Estado,
}
impl From<ProfesorRow> for Profesor {
    fn from(row: ProfesorRow) -> Self {
        Profesor::new(row.dni_profesor, row.nombre, row.genero, row.estado)
    }
}

pub struct ProfesorRepository;
impl ProfesorRepository {
    pub async fn create(pool: &SqlitePool, profesor: &Profesor) -> Result<Profesor, DbError> {
        let row = sqlx::query_as::<_, ProfesorRow>(
            r#"
                    INSERT INTO profesor (
                        dni_profesor,
                        nombre,
                        genero,
                        estado
                    )
                    VALUES (?, ?, ?, ?)
                    RETURNING dni_profesor, nombre, genero, estado
                    "#,
        )
        .bind(profesor.get_dni())
        .bind(profesor.get_nombre_completo())
        .bind(profesor.get_genero())
        .bind(profesor.get_estado())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Profesor, DbError> {
        let row = sqlx::query_as::<_, ProfesorRow>(
            r#"
                    SELECT
                        dni_profesor as "dni_profesor!",
                        nombre as "nombre!",
                        genero as "genero!",
                        estado as "estado!"
                    FROM profesor
                    WHERE dni_profesor = ?
                    "#,
        )
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Profesor>, DbError> {
        let rows = sqlx::query_as::<_, ProfesorRow>(
            r#"
                    SELECT
                        dni_profesor as "dni_profesor!",
                        nombre as "nombre!",
                        genero as "genero!",
                        estado as "estado!"
                    FROM profesor
                    "#,
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Profesor::from).collect())
    }
    pub async fn update(
        pool: &SqlitePool,
        dni: i64,
        profesor: &Profesor,
    ) -> Result<Profesor, DbError> {
        let row = sqlx::query_as::<_, ProfesorRow>(
            r#"
                    UPDATE profesor
                    SET
                        nombre = ?,
                        genero = ?,
                        estado = ?
                    WHERE dni_profesor = ?
                    RETURNING dni_profesor, nombre, genero, estado
                    "#,
        )
        .bind(profesor.get_nombre_completo())
        .bind(profesor.get_genero())
        .bind(profesor.get_estado())
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn delete(
        pool: &SqlitePool,
        dni: i64,
        motivo_eliminacion: &str,
    ) -> Result<(), DbError> {
        let estado = Estado::Eliminado.to_string();
        sqlx::query!(
            r#"
                UPDATE profesor
                SET
                    estado = ?,
                    motivo_eliminacion = ?
                WHERE dni_profesor = ?
                "#,
            estado,
            motivo_eliminacion,
            dni
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(())
    }
}
