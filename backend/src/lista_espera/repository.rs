use super::*;
use crate::app::*;
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub struct ListaDeEsperaRepository;

impl ListaDeEsperaRepository {
    pub async fn delete_lista_espera(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<ListaEspera, ApiError> {
        let lista = Self::get_lista_espera_by_id(pool, id_espera).await?;

        sqlx::query!(
            r#"
                    DELETE FROM lista_de_espera
                    WHERE id_espera = ?
                    "#,
            id_espera
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(lista)
    }
    pub async fn add_lista_espera(
        pool: &SqlitePool,
        id_espera: &str,
        dni_cliente: &str,
        id_clase: &str,
        fecha: NaiveDate,
    ) -> Result<ListaEspera, ApiError> {
        let tipo = "general";

        sqlx::query!(
            r#"
                    INSERT INTO lista_de_espera (
                        id_espera,
                        tipo,
                        fecha_ingreso,
                        id_clase
                    )
                    VALUES (?, ?, ?, ?)
                    "#,
            id_espera,
            tipo,
            fecha,
            id_clase
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_lista_espera_by_id(pool, id_espera).await
    }
    pub async fn get_lista_espera(pool: &SqlitePool) -> Result<Vec<ListaEspera>, ApiError> {
        let rows = sqlx::query!(
            r#"
                    SELECT
                    id_espera,
                            tipo,
                            fecha_ingreso,
                            id_clase
                    FROM lista_de_espera
                    "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                ListaEspera::new(
                    row.id_espera,
                    row.tipo,
                    row.fecha_ingreso.parse::<NaiveDate>().unwrap(),
                    row.id_clase,
                    Vec::new(),
                )
            })
            .collect())
    }
    pub async fn get_lista_espera_by_id(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<ListaEspera, ApiError> {
        let row = sqlx::query!(
            r#"
             SELECT
             id_espera,
                     tipo,
                     fecha_ingreso,
                     id_clase
             FROM lista_de_espera
             WHERE id_espera = ?
             "#,
            id_espera
        )
        .fetch_optional(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        match row {
            Some(row) => Ok(ListaEspera::new(
                row.id_espera,
                row.tipo,
                row.fecha_ingreso.parse::<NaiveDate>().unwrap(),
                row.id_clase,
                Vec::new(),
            )),
            None => Err(ApiError::NotFound),
        }
    }
}
