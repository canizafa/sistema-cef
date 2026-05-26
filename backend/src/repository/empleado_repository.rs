use sqlx::SqlitePool;

use crate::domain::{Empleado, Rol};
use crate::errors::ApiError;
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
                rol: crate::domain::Rol::from(row.rol),
            })
            .collect();

        Ok(empleados)
    }

    // el *as* fuerza a sqlx a tratar el campo como NO opcional (NOT NULL)
    // y a inferir el tipo exacto.
    // Esto evita que sqlx lo interprete como Option<T> y genere
    // errores de mismatched types
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
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(Empleado {
                dni_empleado: row.dni_empleado,
                nombre_apellido: row.nombre_apellido,
                mail: row.mail,
                password_hash: row.password,
                genero: row.genero,
                estado: row.estado,
                rol: crate::domain::Rol::from(row.rol),
            }),
            None => Err(ApiError::NotFound),
        }
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
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(Empleado {
                dni_empleado: row.dni_empleado,
                nombre_apellido: row.nombre_apellido,
                mail: row.mail,
                password_hash: row.password,
                genero: row.genero,
                estado: row.estado,
                rol: crate::domain::Rol::from(row.rol),
            }),
            None => Err(ApiError::NotFound),
        }
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
                    password = ?,
                    genero = ?,
                    estado = ?,
                    rol = ?
                WHERE dni_empleado = ?
                "#,
            empleado.nombre_apellido,
            empleado.mail,
            empleado.password_hash,
            empleado.genero,
            empleado.estado,
            rol,
            dni
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

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
        .map_err(ApiError::DatabaseError)?;

        Ok(empleado)
    }
}
