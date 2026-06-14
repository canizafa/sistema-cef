use super::domain::Cliente;
use crate::app::{
    errors::DbError,
    rol::{Estado, Rol},
};
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct ClienteRow {
    dni: i64,
    nombre: String,
    email: String,
    telefono: String,
    password: String,
    fecha_nacimiento: NaiveDate,
    estado: Estado,
    id_ficha: String,
    rol: Rol,
}
impl From<ClienteRow> for Cliente {
    fn from(row: ClienteRow) -> Self {
        Self::new(
            row.dni,
            row.nombre,
            row.email,
            row.telefono,
            row.password,
            row.fecha_nacimiento,
            row.estado,
            row.id_ficha,
            row.rol,
        )
    }
}

pub struct ClienteRepository;
impl ClienteRepository {
    pub async fn create(
        pool: &SqlitePool,
        cliente: &Cliente,
        id_ficha: &str,
    ) -> Result<Cliente, DbError> {
        let cliente_row = sqlx::query_as::<_, ClienteRow>(
            r#"
            INSERT INTO cliente (
                dni_cliente,
                nombre_completo,
                email,
                telefono,
                fecha_nacimiento,
                estado,
                password,
                id_ficha
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(cliente.get_dni())
        .bind(cliente.get_nombre_apellido())
        .bind(cliente.get_mail())
        .bind(cliente.get_telefono())
        .bind(cliente.get_fecha_nacimiento())
        .bind(cliente.get_estado())
        .bind(cliente.get_password_hash())
        .bind(id_ficha)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(cliente_row.into())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Cliente>, DbError> {
        let rows = sqlx::query_as::<_, ClienteRow>(
            r#"
            SELECT
                c.dni_cliente as "dni_cliente!",
                c.nombre_completo as "nombre_completo!",
                c.email as "email!",
                c.telefono as "telefono!",
                c.fecha_nacimiento as "fecha_nacimiento!",
                c.estado as "estado!",
                c.password as "password!",

                f.id_ficha as "id_ficha!",
                f.enfermedades as "enfermedades!",
                f.operaciones_quirurgicas as "operaciones_quirurgicas!",
                f.detalles as "detalles!"

            FROM cliente c
            INNER JOIN ficha_medica f
                ON c.id_ficha = f.id_ficha
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Cliente::from).collect())
    }

    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Cliente, DbError> {
        let row = sqlx::query_as::<_, ClienteRow>(
            r#"
            SELECT
                c.dni_cliente as "dni_cliente!",
                c.nombre_completo as "nombre_completo!",
                c.email as "email!",
                c.telefono as "telefono!",
                c.fecha_nacimiento as "fecha_nacimiento!",
                c.estado as "estado!",
                c.password as "password!",

                f.id_ficha as "id_ficha!",
                f.enfermedades as "enfermedades!",
                f.operaciones_quirurgicas as "operaciones_quirurgicas!",
                f.detalles as "detalles!"

            FROM cliente c
            INNER JOIN ficha_medica f
                ON c.id_ficha = f.id_ficha
            WHERE c.dni_cliente = ?
            "#,
        )
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<Cliente, DbError> {
        let row = sqlx::query_as::<_, ClienteRow>(
            r#"
            SELECT
                c.dni_cliente as "dni_cliente!",
                c.nombre_completo as "nombre_completo!",
                c.email as "email!",
                c.telefono as "telefono!",
                c.fecha_nacimiento as "fecha_nacimiento!",
                c.estado as "estado!",
                c.password as "password!",

                f.id_ficha as "id_ficha!",
                f.enfermedades as "enfermedades!",
                f.operaciones_quirurgicas as "operaciones_quirurgicas!",
                f.detalles as "detalles!"

            FROM cliente c
            INNER JOIN ficha_medica f
                ON c.id_ficha = f.id_ficha
            WHERE c.email = ?
            "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn update_password_by_dni(
        pool: &SqlitePool,
        dni: i64,
        password_hash: &str,
    ) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE cliente
            SET password = ?
            WHERE dni_cliente = ?
            "#,
            password_hash,
            dni,
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;
        Ok(())
    }

    pub async fn update_password(
        pool: &SqlitePool,
        email: &str,
        password_hash: &str,
    ) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE cliente
            SET password = ?
            WHERE email = ?
            "#,
            password_hash,
            email,
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;
        Ok(())
    }
    pub async fn update_nombre(
        pool: &SqlitePool,
        id: i64,
        nombre_apellido: &str,
    ) -> Result<Cliente, DbError> {
        let row = sqlx::query_as::<_, ClienteRow>(
            r#"
                UPDATE cliente
                SET
                    nombre_completo = ?
                WHERE dni_cliente = ?
                "#,
        )
        .bind(nombre_apellido)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn update_estado(
        pool: &SqlitePool,
        id: i64,
        estado: Estado,
    ) -> Result<Cliente, DbError> {
        let row = sqlx::query_as::<_, ClienteRow>(
            r#"
                UPDATE cliente
                SET estado = ?
                WHERE dni_cliente = ?
                "#,
        )
        .bind(estado)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn delete(pool: &SqlitePool, dni: i64) -> Result<(), DbError> {
        sqlx::query("DELETE FROM cliente WHERE dni_cliente = ?")
            .bind(dni)
            .execute(pool)
            .await
            .map_err(DbError::from)?;

        Ok(())
    }
}
