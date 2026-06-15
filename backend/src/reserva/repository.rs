use crate::{
    app::{errors::DbError, rol::Estado},
    reserva::domain::Reserva,
};
use sqlx::SqlitePool;

pub struct ReservaRepository;
impl ReservaRepository {
    pub async fn create(pool: &SqlitePool, reserva: &Reserva) -> Result<Reserva, DbError> {
        let id = reserva.get_id();
        let estado = reserva.get_estado().to_string();
        let tipo = reserva.get_tipo();
        let fecha = reserva.get_fecha_reserva().format("%Y-%m-%d").to_string();
        let dni_cliente = reserva.get_dni_cliente();
        let id_clase = reserva.get_id_clase();
        sqlx::query!(
            r#"
                INSERT INTO reserva (
                    id_reserva,
                    estado,
                    tipo,
                    fecha_reserva,
                    dni_cliente,
                    id_clase
                )
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            id,
            estado,
            tipo,
            fecha,
            dni_cliente,
            id_clase
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(reserva.clone())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Reserva>, DbError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                id_reserva as "id_reserva!",
                estado as "estado!",
                tipo as "tipo!",
                fecha_reserva as "fecha_reserva!",
                dni_cliente as "dni_cliente!",
                id_clase as "id_clase!"
            FROM reserva
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        let reservas = rows
            .into_iter()
            .map(|row| {
                Reserva::new(
                    row.id_reserva,
                    Estado::from(row.estado),
                    row.tipo,
                    chrono::NaiveDate::parse_from_str(&row.fecha_reserva, "%Y-%m-%d")
                        .unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
                    row.dni_cliente,
                    row.id_clase,
                )
            })
            .collect();

        Ok(reservas)
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Reserva, DbError> {
        let row = sqlx::query!(
            r#"
            SELECT
                id_reserva as "id_reserva!",
                estado as "estado!",
                tipo as "tipo!",
                fecha_reserva as "fecha_reserva!",
                dni_cliente as "dni_cliente!",
                id_clase as "id_clase!"
            FROM reserva
            WHERE id_reserva = ?
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(Reserva::new(
            row.id_reserva,
            Estado::from(row.estado),
            row.tipo,
            chrono::NaiveDate::parse_from_str(&row.fecha_reserva, "%Y-%m-%d")
                .unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            row.dni_cliente,
            row.id_clase,
        ))
    }
    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        reserva: &Reserva,
    ) -> Result<Reserva, DbError> {
        let estado = reserva.get_estado().to_string();
        let tipo = reserva.get_tipo();
        let fecha = reserva.get_fecha_reserva().format("%Y-%m-%d").to_string();
        let dni_cliente = reserva.get_dni_cliente();
        let id_clase = reserva.get_id_clase();

        sqlx::query!(
            r#"
            UPDATE reserva
            SET
                estado = ?,
                tipo = ?,
                fecha_reserva = ?,
                dni_cliente = ?,
                id_clase = ?
            WHERE id_reserva = ?
            "#,
            estado,
            tipo,
            fecha,
            dni_cliente,
            id_clase,
            id
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(reserva.clone())
    }
    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
        sqlx::query!(
            r#"
                DELETE FROM reserva
                WHERE id_reserva = ?
                "#,
            id
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;
        Ok(())
    }
}
