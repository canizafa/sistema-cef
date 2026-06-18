use crate::app::ApiError;
use crate::lista_espera::*;
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
    pub async fn add_cliente(
        pool: &SqlitePool,
        id_espera: &str,
        dni_cliente: i64,
        fecha_ingreso: NaiveDate,
    ) -> Result<ClienteListaEspera, ApiError> {
        sqlx::query!(
            r#"
                INSERT INTO cliente_lista_espera
                (id_espera, dni_cliente, fecha_ingreso)
                VALUES (?, ?, ?)
                "#,
            id_espera,
            dni_cliente,
            fecha_ingreso
        )
        .execute(pool)
        .await?;
        //para este retorno se puede usar una query_as pero lo veo innecesario porque YA CONOZCO
        // los datos del cliente que voy a insertar en el new
        Ok(ClienteListaEspera::new(
            id_espera.to_string(),
            dni_cliente,
            fecha_ingreso,
        ))
    }

    pub async fn get_all(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<Vec<ClienteListaEspera>, ApiError> {
        let rows = sqlx::query_as::<_, ClienteListaEsperaRow>(
            r#"
               SELECT
                   id_espera,
                   dni_cliente,
                   fecha_ingreso
               FROM cliente_lista_espera
               WHERE id_espera = ?
               ORDER BY fecha_ingreso
               "#,
        )
        .bind(id_espera)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
    pub async fn get_first(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<ClienteListaEspera, ApiError> {
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
        .fetch_one(pool)
        .await?;

        Ok(row.into())
    }
    pub async fn delete_cliente(
        pool: &SqlitePool,
        id_espera: &str,
        dni_cliente: i64,
    ) -> Result<ClienteListaEspera, ApiError> {
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
        .await?;

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
        .await?;

        Ok(row.into())
    }
}
