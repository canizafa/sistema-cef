use crate::{
    app::errors::DbError, membresia::domain::Membresia, membresia::estado::EstadoMembresia,
};
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct MembresiaRow {
    pub id_membresia: String,
    pub id_actividad: String,
    pub tipo: String,
    pub dni_cliente: i64,
    pub estado: EstadoMembresia,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub horario: String,
}
impl From<MembresiaRow> for Membresia {
    fn from(value: MembresiaRow) -> Self {
        Self::new(
            value.id_membresia,
            value.tipo,
            value.dni_cliente,
            value.estado,
            value.id_actividad,
            value.fecha_inicio.parse::<NaiveDate>().unwrap(),
            value.fecha_fin.parse::<NaiveDate>().unwrap(),
            value.horario,
        )
    }
}

pub struct MembresiaRepository;
impl MembresiaRepository {
    pub async fn create(pool: &SqlitePool, membresia: &Membresia) -> Result<Membresia, DbError> {
        let id = membresia.get_id_membresia();
        let id_actividad = membresia.get_id_actividad();
        let tipo = membresia.get_tipo_actividad();
        let dni_cliente = membresia.get_dni_cliente();
        let estado = membresia.get_estado();

        let fecha_inicio = membresia.get_fecha_inicio().format("%Y-%m-%d").to_string();
        let fecha_fin = membresia.get_fecha_fin().format("%Y-%m-%d").to_string();

        sqlx::query!(
            r#"
                   INSERT INTO membresias (
                       id_membresia,
                       tipo,
                       dni_cliente,
                       estado,
                       id_actividad,
                       fecha_inicio,
                       fecha_fin
                   )
                   VALUES (?, ?, ?, ?, ?, ?, ?)
                   "#,
            id,
            tipo,
            dni_cliente,
            estado,
            id_actividad,
            fecha_inicio,
            fecha_fin
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(membresia.clone())
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Vec<Membresia>, DbError> {
        let rows = sqlx::query_as::<_, MembresiaRow>(
            r#"
                    SELECT
                        id_membresia,
                        tipo,
                        dni_cliente,
                        estado,
                        id_actividad,
                        fecha_inicio,
                        fecha_fin
                    FROM membresias
                    WHERE dni_cliente = ?
                    "#,
        )
        .bind(dni)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;
        let membresias: Vec<Membresia> = rows.into_iter().map(Membresia::from).collect();
        Ok(membresias)
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Membresia>, DbError> {
        let rows = sqlx::query!(
            r#"
                    SELECT
                        id_membresia as "id_membresia!",
                        tipo as "tipo!",
                        dni_cliente as "dni_cliente!",
                        estado as "estado!",
                        id_actividad as "id_actividad!",
                        fecha_inicio as "fecha_inicio!",
                        fecha_fin as "fecha_fin!",
                        horario as "horario!"
                    FROM membresias
                    "#
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Membresia::new(
                    row.id_membresia,
                    row.tipo,
                    row.dni_cliente,
                    EstadoMembresia::from(row.estado),
                    row.id_actividad,
                    NaiveDate::parse_from_str(&row.fecha_inicio, "%Y-%m-%d").unwrap_or_default(),
                    NaiveDate::parse_from_str(&row.fecha_fin, "%Y-%m-%d").unwrap_or_default(),
                    row.horario,
                )
            })
            .collect())
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Membresia, DbError> {
        let row = sqlx::query!(
            r#"
                    SELECT
                        id_membresia as "id_membresia!",
                        tipo as "tipo!",
                        dni_cliente as "dni_cliente!",
                        estado as "estado!",
                        id_actividad as "id_actividad!",
                        fecha_inicio as "fecha_inicio!",
                        fecha_fin as "fecha_fin!",
                        horario as "horario!"
                    FROM membresias
                    WHERE id_membresia = ?
                    "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(Membresia::new(
            row.id_membresia,
            row.tipo,
            row.dni_cliente,
            EstadoMembresia::from(row.estado),
            row.id_actividad,
            NaiveDate::parse_from_str(&row.fecha_inicio, "%Y-%m-%d").unwrap_or_default(),
            NaiveDate::parse_from_str(&row.fecha_fin, "%Y-%m-%d").unwrap_or_default(),
            row.horario,
        ))
    }
    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        membresia: &Membresia,
    ) -> Result<Membresia, DbError> {
        let tipo = membresia.get_tipo_actividad();
        let estado = membresia.get_estado().to_string();
        let dni_cliente = membresia.get_dni_cliente();
        let id_actividad = membresia.get_id_actividad();

        let fecha_inicio = membresia.get_fecha_inicio().format("%Y-%m-%d").to_string();

        let fecha_fin = membresia.get_fecha_fin().format("%Y-%m-%d").to_string();

        sqlx::query!(
            r#"
                    UPDATE membresias
                    SET
                        tipo = ?,
                        dni_cliente = ?,
                        estado = ?,
                        id_actividad = ?,
                        fecha_inicio = ?,
                        fecha_fin = ?
                    WHERE id_membresia = ?
                    "#,
            tipo,
            dni_cliente,
            estado,
            id_actividad,
            fecha_inicio,
            fecha_fin,
            id
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Self::get_by_id(pool, id).await
    }
    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
        sqlx::query!(
            r#"
                    DELETE FROM membresias
                    WHERE id_membresia = ?
                    "#,
            id
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(())
    }
}
