use crate::{app::errors::DbError, sala::domain::Sala};
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct SalaRow {
    id_sala: String,
    numero: i64,
    capacidad_maxima: i64,
}

impl From<SalaRow> for Sala {
    fn from(row: SalaRow) -> Self {
        Sala::new(row.id_sala, row.numero, row.capacidad_maxima)
    }
}

pub struct SalaRepository;
impl SalaRepository {
    pub async fn create(pool: &SqlitePool, sala: &Sala) -> Result<Sala, DbError> {
        let row = sqlx::query_as::<_, SalaRow>(
            r#"
                INSERT INTO sala (
                    id_sala,
                    numero,
                    capacidad_maxima
                )
                VALUES (?, ?, ?)
                RETURNING id_sala, numero, capacidad_maxima
            "#,
        )
        .bind(sala.get_id())
        .bind(sala.get_numero())
        .bind(sala.get_capacidad_maxima())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Sala, DbError> {
        let row = sqlx::query_as::<_, SalaRow>(
            r#"
                SELECT
                    id_sala as "id_sala!",
                    numero as "numero!",
                    capacidad_maxima as "capacidad_maxima!"
                FROM sala
                WHERE id_sala = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Sala>, DbError> {
        let rows = sqlx::query_as::<_, SalaRow>(
            r#"
                SELECT
                    id_sala as "id_sala!",
                    numero as "numero!",
                    capacidad_maxima as "capacidad_maxima!"
                FROM sala
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Sala::from).collect())
    }
    // pub async fn update(pool: &SqlitePool, id: &str, sala: &Sala) -> Result<Sala, DbError> {
    //     let row = sqlx::query_as::<_, SalaRow>(
    //         r#"
    //             UPDATE sala
    //             SET
    //                 numero = ?,
    //                 capacidad_maxima = ?
    //             WHERE id_sala = ?
    //             RETURNING id_sala, numero, capacidad_maxima
    //         "#,
    //     )
    //     .bind(sala.numero)
    //     .bind(sala.capacidad_maxima)
    //     .bind(id)
    //     .fetch_one(pool)
    //     .await
    //     .map_err(DbError::from)?;

    //     Ok(row.into())
    // }
    // pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
    //     sqlx::query!(
    //         r#"
    //             DELETE FROM sala
    //             WHERE id_sala = ?
    //         "#,
    //         id
    //     )
    //     .execute(pool)
    //     .await
    //     .map_err(DbError::from)?;

    //     Ok(())
    // }
}
