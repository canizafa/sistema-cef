use sqlx::SqlitePool;

use crate::domain::pago::Pago;
use crate::errors::ApiError;

pub struct PagoRepository;

impl PagoRepository {
    pub async fn create_pago(pool: &SqlitePool, pago: &Pago) -> Result<Pago, ApiError> {
        let id_pago = pago.get_id_pago();
        let monto = pago.get_monto();
        let fecha = pago.get_fecha().format("%Y-%m-%d").to_string();
        let hora = pago.get_hora();
        let sena = pago.get_sena();

        let id_membresia = pago.get_id_membresia().cloned();
        let reserva_paga = pago.get_reserva_paga().cloned();

        sqlx::query!(
            r#"
                   INSERT INTO pagos (
                       id_pago,
                       monto,
                       fecha,
                       hora,
                       sena,
                       id_membresia,
                       reserva_paga
                   )
                   VALUES (?, ?, ?, ?, ?, ?, ?)
                   "#,
            id_pago,
            monto,
            fecha,
            hora,
            sena,
            id_membresia,
            reserva_paga
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(pago.clone())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Pago>, ApiError> {
        let rows = sqlx::query!(
            r#"
                    SELECT
                        id_pago as "id_pago!",
                        monto as "monto!",
                        fecha as "fecha!",
                        hora as "hora!",
                        sena as "sena!",
                        id_membresia,
                        reserva_paga
                    FROM pagos
                    "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Pago::new(
                    row.id_pago,
                    row.monto,
                    chrono::NaiveDate::parse_from_str(&row.fecha, "%Y-%m-%d").unwrap(),
                    row.hora,
                    row.sena,
                    row.id_membresia,
                    row.reserva_paga,
                )
            })
            .collect())
    }
    pub async fn get_pago(pool: &SqlitePool, id: &str) -> Result<Pago, ApiError> {
        let row = sqlx::query!(
            r#"
                   SELECT
                       id_pago as "id_pago!",
                       monto as "monto!",
                       fecha as "fecha!",
                       hora as "hora!",
                       sena as "sena!",
                       id_membresia,
                       reserva_paga
                   FROM pagos
                   WHERE id_pago = ?
                   "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(Pago::new(
                row.id_pago,
                row.monto,
                chrono::NaiveDate::parse_from_str(&row.fecha, "%Y-%m-%d").unwrap(),
                row.hora,
                row.sena,
                row.id_membresia,
                row.reserva_paga,
            )),
            None => Err(ApiError::NotFound),
        }
    }
    pub async fn delete_pago(pool: &SqlitePool, id: &str) -> Result<Pago, ApiError> {
        let pago = Self::get_pago(pool, id).await?;

        sqlx::query!(
            r#"
                   DELETE FROM pagos
                   WHERE id_pago = ?
                   "#,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(pago)
    }
}
