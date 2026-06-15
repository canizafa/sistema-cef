use crate::app::errors::DbError;
use crate::empleado::domain::Empleado;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct EmpleadoRow {
    dni_empleado: i64,
    nombre_apellido: String,
    password: String,
    mail: String,
    genero: String,
    estado: String,
    rol: String,
}
impl From<EmpleadoRow> for Empleado {
    fn from(row: EmpleadoRow) -> Self {
        Empleado {
            dni_empleado: row.dni_empleado,
            nombre_apellido: row.nombre_apellido,
            password_hash: row.password,
            mail: row.mail,
            genero: row.genero,
            estado: row.estado,
            rol: row.rol.into(),
        }
    }
}

pub struct EmpleadoRepository;
impl EmpleadoRepository {
    pub async fn create(pool: &SqlitePool, empleado: &Empleado) -> Result<Empleado, DbError> {
        let row = sqlx::query_as::<_, EmpleadoRow>(
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
               RETURNING dni_empleado, nombre_apellido, mail, password, genero, estado, rol
               "#,
        )
        .bind(empleado.get_dni())
        .bind(empleado.get_nombre_apellido())
        .bind(empleado.get_mail())
        .bind(empleado.get_password_hash())
        .bind(empleado.get_genero())
        .bind(empleado.get_estado())
        .bind(empleado.get_rol())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Empleado>, DbError> {
        let rows = sqlx::query_as::<_, EmpleadoRow>(
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
              "#,
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Empleado::from).collect())
    }

    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<Empleado, DbError> {
        let row = sqlx::query_as::<_, EmpleadoRow>(
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
                WHERE mail = ?
                "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Empleado, DbError> {
        let row = sqlx::query_as::<_, EmpleadoRow>(
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
                WHERE dni_empleado = ?
                "#,
        )
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn update_password_by_email(
        pool: &SqlitePool,
        email: &str,
        password_hash: &str,
    ) -> Result<(), DbError> {
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
        .map_err(DbError::from)?;

        Ok(())
    }

    pub async fn update_password_by_dni(
        pool: &SqlitePool,
        dni: i64,
        password_hash: &str,
    ) -> Result<(), DbError> {
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
        .map_err(DbError::from)?;
        Ok(())
    }

    pub async fn update(
        pool: &SqlitePool,
        dni: i64,
        empleado: &Empleado,
    ) -> Result<Empleado, DbError> {
        let row = sqlx::query_as::<_, EmpleadoRow>(
            r#"
                UPDATE empleado
                SET
                    nombre_apellido = ?,
                    mail = ?,
                    genero = ?,
                    estado = ?,
                    rol = ?
                WHERE dni_empleado = ?
                RETURNING *
                "#,
        )
        .bind(empleado.nombre_apellido.clone())
        .bind(empleado.mail.clone())
        .bind(empleado.genero.clone())
        .bind(empleado.estado.clone())
        .bind(empleado.rol.to_string())
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn delete(pool: &SqlitePool, dni: i64) -> Result<(), DbError> {
        sqlx::query!(
            r#"
                DELETE FROM empleado
                WHERE dni_empleado = ?
                "#,
            dni
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(())
    }
}
