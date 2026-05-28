use crate::{
    domain::{Cliente, Estado, FichaMedica, Rol},
    errors::ApiError,
};
use chrono::NaiveDate;
use sqlx::SqlitePool;
pub struct ClienteRepository;

impl ClienteRepository {
    pub async fn create_cliente(pool: &SqlitePool, cliente: &Cliente) -> Result<Cliente, ApiError> {
        let dni = cliente.get_dni();
        let nombre = cliente.get_nombre_apellido();
        let email = cliente.get_email();
        let telefono = cliente.get_telefono();
        let fecha_nacimiento = cliente.get_fecha_nacimiento().to_string();
        let estado = cliente.get_estado();
        let password = cliente.get_password_hash();
        let ficha = cliente.get_ficha_medica();
        let id_ficha = ficha.get_id_ficha();
        let enfermedades = ficha.get_enfermedades();
        let operaciones = ficha.get_operaciones_quirurgicas();
        let detalles = ficha.get_detalles();

        // primero crear ficha médica
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

        // después crear cliente
        sqlx::query!(
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
            dni,
            nombre,
            email,
            telefono,
            fecha_nacimiento,
            estado,
            password,
            id_ficha
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(cliente.clone())
    }

    pub async fn list_clientes(pool: &SqlitePool) -> Result<Vec<Cliente>, ApiError> {
        let rows = sqlx::query!(
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
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        let clientes = rows
            .into_iter()
            .map(|row| {
                let ficha = FichaMedica::new(
                    row.id_ficha,
                    row.enfermedades,
                    row.operaciones_quirurgicas,
                    row.detalles,
                );

                Cliente::new(
                    row.dni_cliente,
                    row.nombre_completo,
                    row.password,
                    row.email,
                    row.telefono,
                    row.fecha_nacimiento.parse::<NaiveDate>().unwrap(),
                    Estado::from(row.estado),
                    ficha,
                    Rol::Cliente,
                )
            })
            .collect();

        Ok(clientes)
    }

    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Cliente, ApiError> {
        let row = sqlx::query!(
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
            dni
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        match row {
            Some(row) => {
                let ficha = FichaMedica::new(
                    row.id_ficha,
                    row.enfermedades,
                    row.operaciones_quirurgicas,
                    row.detalles,
                );

                let cliente = Cliente::new(
                    row.dni_cliente,
                    row.nombre_completo,
                    row.password,
                    row.email,
                    row.telefono,
                    row.fecha_nacimiento.parse::<NaiveDate>().unwrap(),
                    Estado::from(row.estado),
                    ficha,
                    Rol::Cliente,
                );

                Ok(cliente)
            }

            None => Err(ApiError::NotFound),
        }
    }

    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<Cliente, ApiError> {
        let row = sqlx::query!(
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
            email
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        match row {
            Some(row) => {
                let ficha = FichaMedica::new(
                    row.id_ficha,
                    row.enfermedades,
                    row.operaciones_quirurgicas,
                    row.detalles,
                );

                let cliente = Cliente::new(
                    row.dni_cliente,
                    row.nombre_completo,
                    row.password,
                    row.email,
                    row.telefono,
                    row.fecha_nacimiento.parse::<NaiveDate>().unwrap(),
                    Estado::from(row.estado),
                    ficha,
                    Rol::Cliente,
                );

                Ok(cliente)
            }
            None => Err(ApiError::NotFound),
        }
    }
    pub async fn update_password(
        pool: &SqlitePool,
        email: &str,
        password_hash: &str,
    ) -> Result<(), ApiError> {
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
        .map_err(|e| ApiError::DatabaseError(e))?;
        Ok(())
    }
    pub async fn update_cliente(
        pool: &SqlitePool,
        id: i64,
        cliente: &Cliente,
    ) -> Result<Cliente, ApiError> {
        let nombre = cliente.get_nombre_apellido();
        let email = cliente.get_email();
        let telefono = cliente.get_telefono();
        let fecha_nacimiento = cliente.get_fecha_nacimiento().to_string();
        let estado = cliente.get_estado().to_string();

        let ficha = cliente.get_ficha_medica();
        let id_ficha = ficha.get_id_ficha();
        let enfermedades = ficha.get_enfermedades();
        let operaciones = ficha.get_operaciones_quirurgicas();
        let detalles = ficha.get_detalles();

        // 1) actualizar ficha médica
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
        .map_err(ApiError::DatabaseError)?;

        // 2) actualizar cliente
        sqlx::query!(
            r#"
                UPDATE cliente
                SET
                    nombre_completo = ?,
                    email = ?,
                    telefono = ?,
                    fecha_nacimiento = ?,
                    estado = ?
                WHERE dni_cliente = ?
                "#,
            nombre,
            email,
            telefono,
            fecha_nacimiento,
            estado,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        // 3) devolver cliente actualizado
        Self::get_by_dni(pool, id).await
    }
    pub async fn delete_cliente(pool: &SqlitePool, dni: i64) -> Result<Cliente, ApiError> {
        let cliente = Self::get_by_dni(pool, dni).await?;

        sqlx::query("DELETE FROM cliente WHERE dni_cliente = ?")
            .bind(dni)
            .execute(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(cliente)
    }
}
