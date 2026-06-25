use super::domain::ListaEspera;
use crate::app::errors::DbError;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct ListaEsperaRow {
    id_espera: String,
    tipo: String,
    id_clase: String,
}

impl From<ListaEsperaRow> for ListaEspera {
    fn from(row: ListaEsperaRow) -> Self {
        ListaEspera::new(row.id_espera, row.tipo, row.id_clase)
    }
}
pub struct ListaDeEsperaRepository;

impl ListaDeEsperaRepository {
    pub async fn create(pool: &SqlitePool, lista: &ListaEspera) -> Result<ListaEspera, DbError> {
        let row = sqlx::query_as::<_, ListaEsperaRow>(
            r#"
                   INSERT INTO lista_de_espera(
                       id_espera,
                       tipo,
                       id_clase
                   )
                   VALUES (?, ?, ?)
                   RETURNING
                       id_espera,
                       tipo,
                       id_clase
                   "#,
        )
        .bind(lista.get_id_lista())
        .bind(lista.get_tipo())
        .bind(lista.get_id_clase())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_by_clase(pool: &SqlitePool, id_clase: &str) -> Result<ListaEspera, DbError> {
        let row = sqlx::query_as::<_, ListaEsperaRow>(
            r#"
            SELECT
                id_espera,
                tipo,
                id_clase
            FROM lista_de_espera
            WHERE id_clase = ?
            LIMIT 1
            "#,
        )
        .bind(id_clase)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<ListaEspera, DbError> {
        let row = sqlx::query_as::<_, ListaEsperaRow>(
            r#"
               SELECT
                   id_espera,
                   tipo,
                   id_clase
               FROM lista_de_espera
               WHERE id_espera = ?
               "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<ListaEspera>, DbError> {
        let rows = sqlx::query_as::<_, ListaEsperaRow>(
            r#"
                    SELECT
                        id_espera,
                        tipo,
                        id_clase
                    FROM lista_de_espera
                    "#,
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_by_clase_y_tipo(
        pool: &SqlitePool,
        id_clase: &str,
        tipo: &str,
    ) -> Result<ListaEspera, DbError> {
        let row = sqlx::query_as::<_, ListaEsperaRow>(
            r#"
                SELECT
                    id_espera,
                    tipo,
                    id_clase
                FROM lista_de_espera
                WHERE id_clase = ?
                  AND tipo = ?
                "#,
        )
        .bind(id_clase)
        .bind(tipo)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }
    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            DELETE FROM lista_de_espera
            WHERE id_espera = ?
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;

        Ok(())
    }
}
