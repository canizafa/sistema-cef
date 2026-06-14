use crate::{app::errors::DbError, asistencia::domain::Asistencia};
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct AsistenciaRow {
    pub id_asistencia: String,
    pub fecha: NaiveDate,
    pub metodo: String,
    pub id_reserva: String,
}

impl From<AsistenciaRow> for Asistencia {
    fn from(row: AsistenciaRow) -> Self {
        Asistencia::new(row.id_asistencia, row.fecha, row.metodo, row.id_reserva)
    }
}

pub struct AsistenciaRepository;
impl AsistenciaRepository {
    pub async fn create(pool: &SqlitePool, asistencia: &Asistencia) -> Result<Asistencia, DbError> {
        let row = sqlx::query_as::<_, AsistenciaRow>(
            r#"
                   INSERT INTO asistencia (
                       id_asistencia,
                       fecha,
                       metodo,
                       id_reserva
                   )
                   VALUES (?, ?, ?, ?)
                   RETURNING id_asistencia, fecha, metodo, id_reserva
                   "#,
        )
        .bind(asistencia.get_id_asistencia())
        .bind(asistencia.get_fecha().format("%Y-%m-%d").to_string())
        .bind(asistencia.get_metodo())
        .bind(asistencia.get_id_reserva())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Asistencia, DbError> {
        let row = sqlx::query_as::<_, AsistenciaRow>(
            r#"
                    SELECT
                        id_asistencia as "id_asistencia!",
                        fecha as "fecha!",
                        metodo as "metodo!",
                        id_reserva as "id_reserva!"
                    FROM asistencia
                    WHERE id_asistencia = ?
                    "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Asistencia>, DbError> {
        let rows = sqlx::query_as::<_, AsistenciaRow>(
            r#"
                    SELECT
                        id_asistencia as "id_asistencia!",
                        fecha as "fecha!",
                        metodo as "metodo!",
                        id_reserva as "id_reserva!"
                    FROM asistencia
                    "#,
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Asistencia::from).collect())
    }
    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        asistencia: &Asistencia,
    ) -> Result<Asistencia, DbError> {
        let row = sqlx::query_as::<_, AsistenciaRow>(
            r#"
                    UPDATE asistencia
                    SET
                        fecha = ?,
                        metodo = ?,
                        id_reserva = ?
                    WHERE id_asistencia = ?
                    RETURNING id_asistencia, fecha, metodo, id_reserva
                    "#,
        )
        .bind(asistencia.get_fecha().format("%Y-%m-%d").to_string())
        .bind(asistencia.get_metodo())
        .bind(asistencia.get_id_reserva())
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
        sqlx::query!(
            r#"
                    DELETE FROM asistencia
                    WHERE id_asistencia = ?
                    "#,
            id
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(())
    }
}
