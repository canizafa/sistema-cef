use crate::domain::{Clase, Estado};
use crate::errors::ApiError;
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub struct ClaseRepository;

impl ClaseRepository {
    pub async fn create_clase(pool: &SqlitePool, clase: &Clase) -> Result<Clase, ApiError> {
        let id = clase.get_id();
        let dia = clase.get_dia();
        let horario = clase.get_horario();
        let cupo_profe = clase.get_cupo_profe();
        let cupo_maximo = clase.get_cupo_maximo();
        let estado = clase.get_estado();
        let descripcion = clase.get_descripcion();
        let id_actividad = clase.get_id_actividad();
        let id_sala = clase.get_id_sala();
        let dni_profesor = clase.get_dni_profesor();

        sqlx::query!(
            "INSERT INTO clase
               (
                   id_clase,
                   dia,
                   horario,
                   cupo_profe,
                   cupo_maximo,
                   estado,
                   descripcion,
                   id_actividad,
                   id_sala,
                   dni_profesor
               )
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            dia,
            horario,
            cupo_profe,
            cupo_maximo,
            estado,
            descripcion,
            id_actividad,
            id_sala,
            dni_profesor
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(clase.clone())
    }
    pub async fn list_clases(pool: &SqlitePool) -> Result<Vec<Clase>, ApiError> {
        let rows = sqlx::query!(
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_actividad,
                id_sala,
                dni_profesor
            FROM clase"
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let clases: Result<Vec<Clase>, ApiError> = rows
            .into_iter()
            .map(|row| {
                Ok(Clase::new(
                    row.id_clase.unwrap(),
                    row.dia.parse::<NaiveDate>().unwrap(),
                    row.horario,
                    row.descripcion.unwrap(),
                    row.cupo_profe.unwrap(),
                    row.cupo_maximo,
                    Estado::from(row.estado),
                    row.id_sala,
                    row.dni_profesor.unwrap(),
                    row.id_actividad,
                ))
            })
            .collect();

        clases
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Clase>, ApiError> {
        let row = sqlx::query!(
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_actividad,
                id_sala,
                dni_profesor
            FROM clase
            WHERE id_clase = ?",
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(Some(Clase::new(
                row.id_clase.unwrap(),
                row.dia.parse::<NaiveDate>().unwrap(),
                row.horario,
                row.descripcion.unwrap(),
                row.cupo_profe.unwrap(),
                row.cupo_maximo,
                Estado::from(row.estado),
                row.id_sala,
                row.dni_profesor.unwrap(),
                row.id_actividad,
            ))),
            None => Ok(None),
        }
    }

    pub async fn update_clase(
        pool: &SqlitePool,
        id: &str,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
        let dia = clase.get_dia();
        let horario = clase.get_horario();
        let cupo_profe = clase.get_cupo_profe();
        let cupo_maximo = clase.get_cupo_maximo();
        let estado = clase.get_estado();
        let descripcion = clase.get_descripcion();
        let id_sala = clase.get_id_sala();
        let dni_profesor = clase.get_dni_profesor();

        sqlx::query!(
            "UPDATE clase
            SET
                dia = ?,
                horario = ?,
                cupo_profe = ?,
                cupo_maximo = ?,
                estado = ?,
                descripcion = ?,
                id_sala = ?,
                dni_profesor = ?
            WHERE id_clase = ?",
            dia,
            horario,
            cupo_profe,
            cupo_maximo,
            estado,
            descripcion,
            id_sala,
            dni_profesor,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_by_id(pool, id).await
    }

    pub async fn delete_clase(pool: &SqlitePool, id: &str) -> Result<Option<Clase>, ApiError> {
        let clase = Self::get_by_id(pool, id).await?;

        sqlx::query!("DELETE FROM clase WHERE id_clase = ?", id)
            .execute(pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        Ok(clase)
    }
}
