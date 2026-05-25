use sqlx::SqlitePool;

use crate::domain::Empleado;
use crate::errors::ApiError;

pub struct EmpleadoRepository;

impl EmpleadoRepository {
    pub async fn create_empleado(
        pool: &SqlitePool,
        empleado: &Empleado,
    ) -> Result<Empleado, ApiError> {
        sqlx::query(
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
        )
        .bind(empleado.get_dni())
        .bind(empleado.get_nombre_apellido())
        .bind(empleado.get_mail())
        .bind(empleado.get_password_hash())
        .bind(empleado.get_genero())
        .bind(empleado.get_estado())
        .bind("EMPLEADO")
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(empleado.clone())
    }
    pub async fn list_empleados(pool: &SqlitePool) -> Result<Vec<Empleado>, ApiError> {
        let rows = sqlx::query!(
            r#"
                SELECT
                    dni_empleado,
                    nombre_apellido,
                    mail,
                    password,
                    genero,
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
                password_hash: row.password.clone(),
                password: row.password,
                mail: row.mail,
                genero: row.genero.unwrap_or_default(),
                estado: row.estado,
                rol: Rol::from(row.rol),
            })
            .collect();

        Ok(empleados)
    }
    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<Empleado, ApiError> {
        let row = sqlx::query(
            r#"
                    SELECT
                        dni_empleado,
                        nombre_apellido,
                        mail,
                        password,
                        genero,
                        estado
                    FROM empleado
                    WHERE dni_empleado = ?
                    "#,
        )
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(|_| ApiError::InvalidEmail)?;

        Ok(Empleado {
            dni_empleado: row.get("dni_empleado"),
            nombre_apellido: row.get("nombre_apellido"),
            mail: row.get("mail"),
            password_hash: row.get("password"),
            password: String::new(),
            genero: row.get("genero"),
            estado: row.get("estado"),
            rol: Rol::Empleado,
        })
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: &str) -> Result<Empleado, ApiError> {
        let row = sqlx::query(
            r#"
                   SELECT
                       dni_empleado,
                       nombre_apellido,
                       mail,
                       password,
                       genero,
                       estado
                   FROM empleado
                   WHERE dni_empleado = ?
                   "#,
        )
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(|_| ApiError::Forbidden)?;

        Ok(Empleado {
            dni_empleado: row.get("dni_empleado"),
            nombre_apellido: row.get("nombre_apellido"),
            mail: row.get("mail"),
            password_hash: row.get("password"),
            password: String::new(),
            genero: row.get("genero"),
            estado: row.get("estado"),
            rol: Rol::Empleado,
        })
    }
    pub async fn update_empleado(
        pool: &SqlitePool,
        dni: i64,
        empleado: &Empleado,
    ) -> Result<Empleado, ApiError> {
        sqlx::query(
            r#"
                    UPDATE empleado
                    SET
                        nombre_apellido = ?,
                        mail = ?,
                        password = ?,
                        genero = ?,
                        estado = ?
                    WHERE dni_empleado = ?
                    "#,
        )
        .bind(empleado.get_nombre_apellido())
        .bind(empleado.get_mail())
        .bind(empleado.get_password_hash())
        .bind(empleado.get_genero())
        .bind(empleado.get_estado())
        .bind(dni)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError)?;

        Ok(empleado)
    }
    pub async fn delete_empleado(pool: &SqlitePool, dni: &str) -> Result<Empleado, ApiError> {
        let empleado = Self::get_by_dni(pool, dni).await?;

        sqlx::query(
            r#"
                    DELETE FROM empleado
                    WHERE dni_empleado = ?
                    "#,
        )
        .bind(dni)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError)?;

        Ok(empleado)
    }
    pub async fn update_password(
        pool: &SqlitePool,
        dni: &str,
        password_hash: &str,
    ) -> Result<Empleado, ApiError> {
        sqlx::query(
            r#"
                    UPDATE empleado
                    SET password = ?
                    WHERE dni_empleado = ?
                    "#,
        )
        .bind(password_hash)
        .bind(dni)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError)?;

        let empleado = Self::get_by_dni(pool, dni).await?;

        Ok(empleado)
    }
}
