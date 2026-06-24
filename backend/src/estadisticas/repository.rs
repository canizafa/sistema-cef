use super::domain::{ClaseMasCancelada, ClaseMasConcurrida, Recaudacion};
use crate::app::errors::DbError;
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct ClaseRow {
    id_clase: String,
    descripcion: String,
    cantidad: i64,
}

#[derive(Debug, sqlx::FromRow)]
struct RecaudacionRow {
    total: f64,
}

pub struct EstadisticaRepository;

impl EstadisticaRepository {
    pub async fn clase_mas_concurrida(
        pool: &SqlitePool,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<ClaseMasConcurrida, DbError> {
        let row = sqlx::query_as::<_, ClaseRow>(
            r#"
            SELECT
                c.id_clase,
                c.descripcion,
                COUNT(*) as cantidad
            FROM reserva r
            INNER JOIN clase c
                ON c.id_clase = r.id_clase
            WHERE
                r.estado = 'confirmada'
                AND r.fecha_reserva BETWEEN ? AND ?
            GROUP BY
                c.id_clase,
                c.descripcion
            ORDER BY cantidad DESC
            LIMIT 1
            "#,
        )
        .bind(desde)
        .bind(hasta)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)?;

        Ok(match row {
            Some(row) => ClaseMasConcurrida::new(row.id_clase, row.descripcion, row.cantidad),
            None => ClaseMasConcurrida::new("".to_string(), "Sin datos".to_string(), 0), //sin clase
        })
    }

    pub async fn clase_mas_cancelada(
        pool: &SqlitePool,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<ClaseMasCancelada, DbError> {
        let row = sqlx::query_as::<_, ClaseRow>(
            r#"
            SELECT
                c.id_clase,
                c.descripcion,
                COUNT(*) as cantidad
            FROM reserva r
            INNER JOIN clase c
                ON c.id_clase = r.id_clase
            WHERE
                r.estado = 'cancelada'
                AND r.fecha_reserva BETWEEN ? AND ?
            GROUP BY
                c.id_clase,
                c.descripcion
            ORDER BY cantidad DESC
            LIMIT 1
            "#,
        )
        .bind(desde)
        .bind(hasta)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)?;

        Ok(match row {
            Some(row) => ClaseMasCancelada::new(row.id_clase, row.descripcion, row.cantidad),
            None => ClaseMasCancelada::new("".to_string(), "Sin datos".to_string(), 0), //sin clase
        })
    }

    pub async fn recaudacion(
        pool: &SqlitePool,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<Recaudacion, DbError> {
        let row = sqlx::query_as::<_, RecaudacionRow>(
            r#"
            SELECT COALESCE(SUM(monto), 0) as total
            FROM pagos
            WHERE fecha BETWEEN ? AND ?
            "#,
        )
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(Recaudacion::new(row.total))
    }
}
