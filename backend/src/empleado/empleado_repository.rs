use super::*;
use crate::app::{ApiError, Rol};
use sqlx::SqlitePool;

pub struct EmpleadoRepository;

impl EmpleadoRepository {
    pub async fn create_empleado(
        pool: &SqlitePool,
        empleado: &Empleado,
    ) -> Result<Empleado, ApiError> {
        let rol = empleado.rol.to_string();
        sqlx::query!(
            r#"
               INSERT INTO empleado (
                   dni_empleado,
                   nombre_apellido,
                   mail,
                   password,
                   genero,
                   estado,
                   rol
               )
               VALUES (?, ?, ?, ?, ?, ?, ?)
               "#,
            empleado.dni_empleado,
            empleado.nombre_apellido,
            empleado.mail,
            empleado.password_hash,
            empleado.genero,
            empleado.estado,
            rol
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(empleado.clone())
    }

    pub async fn get_empleados(pool: &SqlitePool) -> Result<Vec<Empleado>, ApiError> {
        let rows = sqlx::query!(
            r#"
              SELECT
                  dni_empleado,
                  nombre_apellido,
                  mail,
                  password,
                  genero as "genero!: String",
                  estado,
                  rol
              FROM empleado
              "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let empleados = rows
            .into_iter()
            .map(|row| Empleado {
                dni_empleado: row.dni_empleado,
                nombre_apellido: row.nombre_apellido,
                mail: row.mail,
                password_hash: row.password,
                genero: row.genero,
                estado: row.estado,
                rol: Rol::from(row.rol),
            })
            .collect();

        Ok(empleados)
    }

    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<Empleado, ApiError> {
        let row = sqlx::query!(
            r#"
                SELECT
                    dni_empleado as "dni_empleado!: i64",
                    nombre_apellido,
                    mail,
                    password,
                    genero as "genero!: String",
                    estado,
                    rol
                FROM empleado
                WHERE mail = ?
                "#,
            email
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(Empleado {
            dni_empleado: row.dni_empleado,
            nombre_apellido: row.nombre_apellido,
            mail: row.mail,
            password_hash: row.password,
            genero: row.genero,
            estado: row.estado,
            rol: Rol::from(row.rol),
        })
    }

    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Empleado, ApiError> {
        let row = sqlx::query!(
            r#"
                SELECT
                    dni_empleado,
                    nombre_apellido,
                    mail,
                    password,
                    genero as "genero!: String",
                    estado,
                    rol
                FROM empleado
                WHERE dni_empleado = ?
                "#,
            dni
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(Empleado {
            dni_empleado: row.dni_empleado,
            nombre_apellido: row.nombre_apellido,
            mail: row.mail,
            password_hash: row.password,
            genero: row.genero,
            estado: row.estado,
            rol: Rol::from(row.rol),
        })
    }

    pub async fn update_password_by_email(
        pool: &SqlitePool,
        email: &str,
        password_hash: &str,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            r#"
                UPDATE empleado
                SET password = ?
                WHERE mail = ?
                "#,
            password_hash,
            email,
        )
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;
        Ok(())
    }

    pub async fn update_password_by_dni(
        pool: &SqlitePool,
        dni: i64,
        password_hash: &str,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            r#"
                UPDATE empleado
                SET password = ?
                WHERE dni_empleado = ?
                "#,
            password_hash,
            dni,
        )
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;
        Ok(())
    }

    pub async fn update_empleado(
        pool: &SqlitePool,
        dni: i64,
        empleado: &Empleado,
    ) -> Result<Empleado, ApiError> {
        let rol = empleado.rol.to_string();
        sqlx::query!(
            r#"
                UPDATE empleado
                SET
                    nombre_apellido = ?,
                    mail = ?,
                    genero = ?,
                    estado = ?,
                    rol = ?
                WHERE dni_empleado = ?
                "#,
            empleado.nombre_apellido,
            empleado.mail,
            empleado.genero,
            empleado.estado,
            rol,
            dni
        )
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Self::get_by_dni(pool, dni).await
    }

    pub async fn delete_empleado(pool: &SqlitePool, dni: i64) -> Result<Empleado, ApiError> {
        let empleado = Self::get_by_dni(pool, dni).await?;

        sqlx::query!(
            r#"
                DELETE FROM empleado
                WHERE dni_empleado = ?
                "#,
            dni
        )
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(empleado)
    }
}
