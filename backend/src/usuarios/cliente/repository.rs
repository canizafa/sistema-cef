use super::domain::Cliente;
use crate::app::errors::DbError;
use crate::usuarios::{estado::EstadoUsuario, rol::RolUsuario};
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
    estado: EstadoUsuario,
    motivo_eliminacion: Option<String>,
    id_ficha: String,
    rol: RolUsuario,
    creditos: i64,
    contador_cancelaciones: i64,
}
impl From<ClienteRow> for Cliente {
    fn from(row: ClienteRow) -> Self {
        Self::new(
            row.dni,
            row.nombre,
            row.password,
            row.email,
            row.telefono,
            row.fecha_nacimiento,
            row.estado,
            row.motivo_eliminacion,
            row.id_ficha,
            row.rol,
            row.creditos,
            row.contador_cancelaciones,
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
                motivo_eliminacion,
                password,
                id_ficha
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING
                dni_cliente AS dni,
                nombre_completo AS nombre,
                email,
                telefono,
                fecha_nacimiento,
                estado,
                motivo_eliminacion,
                password,
                id_ficha,
                creditos,
                contador_cancelaciones,
                'cliente' AS rol
            "#,
        )
        .bind(cliente.get_dni())
        .bind(cliente.get_nombre_apellido())
        .bind(cliente.get_mail())
        .bind(cliente.get_telefono())
        .bind(cliente.get_fecha_nacimiento())
        .bind(cliente.get_estado())
        .bind(cliente.get_motivo_eliminacion())
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
                c.dni_cliente AS dni,
                c.nombre_completo AS nombre,
                c.email,
                c.telefono,
                c.fecha_nacimiento,
                c.estado,
                c.motivo_eliminacion,
                c.password,
                c.id_ficha,
                c.creditos,
                c.contador_cancelaciones,
                c.fecha_notificacion,
                'cliente' AS rol
            FROM cliente c
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
                c.dni_cliente AS dni,
                c.nombre_completo AS nombre,
                c.email,
                c.telefono,
                c.fecha_nacimiento,
                c.estado,
                c.motivo_eliminacion,
                c.password,
                c.id_ficha,
                c.creditos,
                c.contador_cancelaciones,
                c.fecha_notificacion,
                'cliente' AS rol
            FROM cliente c
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
                c.dni_cliente AS dni,
                c.nombre_completo AS nombre,
                c.email,
                c.telefono,
                c.fecha_nacimiento,
                c.estado,
                c.motivo_eliminacion,
                c.password,
                c.id_ficha,
                c.creditos,
                c.contador_cancelaciones,
                c.fecha_notificacion,
                'cliente' AS rol
            FROM cliente c
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
                SET nombre_completo = ?
                WHERE dni_cliente = ?
                RETURNING
                    dni_cliente AS dni,
                    nombre_completo AS nombre,
                    email,
                    telefono,
                    fecha_nacimiento,
                    estado,
                    motivo_eliminacion,
                    password,
                    id_ficha,
                    creditos,
                    contador_cancelaciones,
                    fecha_notificacion,
                    'cliente' AS rol
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
        estado: EstadoUsuario,
        motivo_eliminacion: Option<String>,
    ) -> Result<Cliente, DbError> {
        let row = sqlx::query_as::<_, ClienteRow>(
            r#"
                UPDATE cliente
                SET estado = ?,
                motivo_eliminacion = ?
                WHERE dni_cliente = ?
                RETURNING
                    dni_cliente AS dni,
                    nombre_completo AS nombre,
                    email,
                    telefono,
                    fecha_nacimiento,
                    estado,
                    motivo_eliminacion,
                    password,
                    id_ficha,
                    creditos,
                    contador_cancelaciones,
                    fecha_notificacion,
                    'cliente' AS rol
                "#,
        )
        .bind(estado)
        .bind(motivo_eliminacion)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn delete(
        pool: &SqlitePool,
        dni: i64,
        estado: EstadoUsuario,
        motivo_eliminacion: Option<String>,
    ) -> Result<(), DbError> {
        sqlx::query("UPDATE cliente SET estado = ?, motivo_eliminacion = ? WHERE dni_cliente = ?")
            .bind(estado)
            .bind(motivo_eliminacion)
            .bind(dni)
            .execute(pool)
            .await
            .map_err(DbError::from)?;

        Ok(())
    }
    pub async fn update_creditos_y_cancelaciones(
        pool: &SqlitePool,
        dni: i64,
        creditos: i64,
        contador_cancelaciones: i64,
    ) -> Result<Cliente, DbError> {
        let row = sqlx::query_as::<_, ClienteRow>(
            r#"
            UPDATE cliente
            SET
                creditos = ?,
                contador_cancelaciones = ?
            WHERE dni_cliente = ?
            RETURNING
                dni_cliente AS dni,
                nombre_completo AS nombre,
                email,
                telefono,
                fecha_nacimiento,
                estado,
                motivo_eliminacion,
                password,
                id_ficha,
                creditos,
                contador_cancelaciones,
                fecha_notificacion,
                'cliente' AS rol
            "#,
        )
        .bind(creditos)
        .bind(contador_cancelaciones)
        .bind(dni)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn update_notify_date(
        db: &SqlitePool,
        email: &str,
        date: NaiveDate,
    ) -> Result<(), DbError> {
        let _ = sqlx::query!(
            "
            UPDATE cliente
            SET fecha_notificacion = ?
            WHERE email = ?
            ",
            date,
            email
        )
        .fetch_one(db)
        .await
        .map_err(DbError::from)?;
        Ok(())
    }
}
