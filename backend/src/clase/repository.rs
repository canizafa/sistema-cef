use super::domain::Clase;
use crate::app::{errors::DbError, rol::Estado};
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct ClaseRow {
    id_clase: String,
    dia: NaiveDate,
    horario: String,
    descripcion: String,
    cupo_base: i64,
    inscripciones: i64,
    estado: Estado,
    id_sala: String,
    dni_profesor: i64,
    id_actividad: String,
}
impl From<ClaseRow> for Clase {
    fn from(row: ClaseRow) -> Self {
        Clase::new(
            row.id_clase,
            row.dia,
            row.horario,
            row.descripcion,
            row.cupo_base,
            row.inscripciones,
            row.estado,
            row.id_sala,
            row.dni_profesor,
            row.id_actividad,
        )
    }
}

pub struct ClaseRepository;
impl ClaseRepository {
    pub async fn create(pool: &SqlitePool, clase: &Clase) -> Result<Clase, DbError> {
        let row = sqlx::query_as::<_, ClaseRow>(
            "INSERT INTO clase
               (
                   id_clase,
                   dia,
                   horario,
                   cupo_base,
                   inscripciones,
                   estado,
                   descripcion,
                   id_actividad,
                   id_sala,
                   dni_profesor
               )
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&clase.get_id())
        .bind(&clase.get_dia())
        .bind(&clase.get_horario())
        .bind(&clase.get_cupo_base())
        .bind(0)
        .bind(&clase.get_estado())
        .bind(&clase.get_descripcion())
        .bind(&clase.get_id_actividad())
        .bind(&clase.get_id_sala())
        .bind(&clase.get_dni_profesor())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Clase>, DbError> {
        let rows = sqlx::query_as::<_, ClaseRow>(
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_base,
                estado,
                descripcion,
                id_actividad,
                id_sala,
                dni_profesor
            FROM clase",
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Clase, DbError> {
        let row = sqlx::query_as::<_, ClaseRow>(
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_base,
                inscripciones,
                estado,
                descripcion,
                id_actividad,
                id_sala,
                dni_profesor
            FROM clase
            WHERE id_clase = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn update(pool: &SqlitePool, id: &str, clase: &Clase) -> Result<Clase, DbError> {
        let row = sqlx::query_as::<_, ClaseRow>(
            "UPDATE clase
            SET
                estado = ?,
                dni_profesor = ?
            WHERE id_clase = ?
            RETURNING
                id_clase,
                dia,
                horario,
                cupo_base,
                inscripciones,
                estado,
                descripcion,
                id_actividad,
                id_sala,
                dni_profesor",
        )
        .bind(clase.get_estado())
        .bind(clase.get_dni_profesor())
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
        sqlx::query!("DELETE FROM clase WHERE id_clase = ?", id)
            .execute(pool)
            .await
            .map_err(DbError::from)?;

        Ok(())
    }
}
