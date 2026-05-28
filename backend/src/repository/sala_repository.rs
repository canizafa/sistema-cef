use sqlx::SqlitePool;

use crate::{domain::sala::Sala, errors::ApiError};

pub struct SalaRepository;

impl SalaRepository {
    pub async fn create_sala(pool: &SqlitePool, sala: &Sala) -> Result<Sala, ApiError> {
        let id = sala.get_id();
        let numero = sala.get_numero();
        let capacidad_maxima = sala.get_capacidad_maxima();

        sqlx::query!(
            r#"
                    INSERT INTO sala (
                        id_sala,
                        numero,
                        capacidad_maxima
                    )
                    VALUES (?, ?, ?)
                    "#,
            id,
            numero,
            capacidad_maxima
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(sala.clone())
    }
    pub async fn get_sala_by_id(pool: &SqlitePool, id: &str) -> Result<Sala, ApiError> {
        let row = sqlx::query!(
            r#"
                    SELECT
                        id_sala as "id_sala!",
                        numero as "numero!",
                        capacidad_maxima as "capacidad_maxima!"
                    FROM sala
                    WHERE id_sala = ?
                    "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(Sala::new(row.id_sala, row.numero, row.capacidad_maxima))
    }
    pub async fn get_all_salas(pool: &SqlitePool) -> Result<Vec<Sala>, ApiError> {
        let rows = sqlx::query!(
            r#"
                    SELECT
                        id_sala as "id_sala!",
                        numero as "numero!",
                        capacidad_maxima as "capacidad_maxima!"
                    FROM sala
                    "#
        )
        .fetch_all(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| Sala::new(row.id_sala, row.numero, row.capacidad_maxima))
            .collect())
    }
    pub async fn update_sala(pool: &SqlitePool, id: &str, sala: &Sala) -> Result<Sala, ApiError> {
        let numero = sala.get_numero();
        let capacidad_maxima = sala.get_capacidad_maxima();

        sqlx::query!(
            r#"
                    UPDATE sala
                    SET
                        numero = ?,
                        capacidad_maxima = ?
                    WHERE id_sala = ?
                    "#,
            numero,
            capacidad_maxima,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Self::get_sala_by_id(pool, id).await
    }
    pub async fn delete_sala(pool: &SqlitePool, id: &str) -> Result<Sala, ApiError> {
        let sala = Self::get_sala_by_id(pool, id).await?;

        sqlx::query!(
            r#"
                    DELETE FROM sala
                    WHERE id_sala = ?
                    "#,
            id
        )
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(sala)
    }
}
