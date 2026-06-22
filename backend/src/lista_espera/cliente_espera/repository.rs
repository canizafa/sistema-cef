use crate::app::errors::DbError;
use crate::lista_espera::cliente_espera::domain::ClienteListaEspera;
use chrono::NaiveDate;
use sqlx::SqlitePool;
#[derive(Debug, sqlx::FromRow)]
struct ClienteListaEsperaRow {
    id_espera: String,
    dni_cliente: i64,
    fecha_ingreso: NaiveDate,
}

impl From<ClienteListaEsperaRow> for ClienteListaEspera {
    fn from(row: ClienteListaEsperaRow) -> Self {
        ClienteListaEspera::new(row.id_espera, row.dni_cliente, row.fecha_ingreso)
    }
}
pub struct ClienteListaEsperaRepository;
impl ClienteListaEsperaRepository {
    pub async fn create(
        pool: &SqlitePool,
        cliente: &ClienteListaEspera,
    ) -> Result<ClienteListaEspera, DbError> {
        let row = sqlx::query_as::<_, ClienteListaEsperaRow>(
            r#"
                INSERT INTO cliente_lista_espera (
                    id_espera,
                    dni_cliente,
                    fecha_ingreso
                )
                VALUES (?, ?, ?)
                RETURNING
                    id_espera,
                    dni_cliente,
                    fecha_ingreso
                "#,
        )
        .bind(cliente.get_id_espera())
        .bind(cliente.get_dni_cliente())
        .bind(cliente.get_fecha_ingreso())
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.into())
    }

    pub async fn get_all(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<Vec<ClienteListaEspera>, DbError> {
        let rows = sqlx::query_as::<_, ClienteListaEsperaRow>(
            r#"
               SELECT
                   id_espera,
                   dni_cliente,
                   fecha_ingreso
               FROM cliente_lista_espera
               WHERE id_espera = ?
               ORDER BY fecha_ingreso ASC
               "#,
        )
        .bind(id_espera)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
    pub async fn get_next(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<Option<ClienteListaEspera>, DbError> {
        let row = sqlx::query_as::<_, ClienteListaEsperaRow>(
            r#"
                SELECT
                    id_espera,
                    dni_cliente,
                    fecha_ingreso
                FROM cliente_lista_espera
                WHERE id_espera = ?
                ORDER BY fecha_ingreso ASC
                LIMIT 1
                "#,
        )
        .bind(id_espera)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)?;

        Ok(row.map(Into::into))
    }
    pub async fn delete(
        pool: &SqlitePool,
        id_espera: &str,
        dni_cliente: i64,
    ) -> Result<ClienteListaEspera, DbError> {
        let row = sqlx::query_as::<_, ClienteListaEsperaRow>(
            r#"
                SELECT
                    id_espera,
                    dni_cliente,
                    fecha_ingreso
                FROM cliente_lista_espera
                WHERE id_espera = ?
                AND dni_cliente = ?
                "#,
        )
        .bind(id_espera)
        .bind(dni_cliente)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)?;

        sqlx::query!(
            r#"
                DELETE FROM cliente_lista_espera
                WHERE id_espera = ?
                AND dni_cliente = ?
                "#,
            id_espera,
            dni_cliente
        )
        .execute(pool)
        .await
        .map_err(DbError::from)?;
        Ok(row.into())
    }
}
