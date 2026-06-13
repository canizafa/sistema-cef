use crate::app::ApiError;
use crate::cliente::*;
use crate::lista_espera::*;
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub struct ClienteListaEsperaRepository;
impl ClienteListaEsperaRepository {
    pub async fn create_lista_espera(
        pool: &SqlitePool,
        lista: ListaEspera,
    ) -> Result<ListaEspera, ApiError> {
        let id_espera = lista.get_id_lista();
        let tipo = lista.get_tipo();
        let fecha_ingreso = lista.get_fecha_ingreso();
        let id_clase = lista.get_id_clase();

        sqlx::query!(
            r#"
                INSERT INTO lista_de_espera
                (id_espera, tipo, fecha_ingreso, id_clase)
                VALUES (?, ?, ?, ?)
                "#,
            id_espera,
            tipo,
            fecha_ingreso,
            id_clase
        )
        .execute(pool)
        .await?;

        Ok(lista)
    }
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
        Ok(ClienteListaEspera::new(
            id_espera.to_string(),
            dni_cliente,
            fecha_ingreso,
        ))
    }
    pub async fn get_by_id(pool: &SqlitePool, id_espera: &str) -> Result<ListaEspera, ApiError> {
        let lista = sqlx::query!(
            r#"
            SELECT
                id_espera,
                tipo,
                fecha_ingreso as "fecha_ingreso: NaiveDate",
                id_clase
            FROM lista_de_espera
            WHERE id_espera = ?
            "#,
            id_espera
        )
        .fetch_optional(pool)
        .await?;

        let lista = lista.ok_or(ApiError::NotFound)?;

        let clientes = Self::obtener_clientes(pool, id_espera).await?;

        Ok(ListaEspera::new(
            lista.id_espera,
            lista.tipo,
            lista.fecha_ingreso,
            lista.id_clase,
            clientes,
        ))
    }
    pub async fn get_all(pool: &SqlitePool, id_espera: &str) -> Result<Vec<Cliente>, ApiError> {
        let registros = sqlx::query!(
            r#"
                SELECT dni_cliente
                FROM cliente_lista_espera
                WHERE id_espera = ?
                ORDER BY fecha_ingreso
                "#,
            id_espera
        )
        .fetch_all(pool)
        .await?;

        let mut clientes = Vec::new();
        // recorro cada registro obtenido de cliente_lista_espera para recuperar
        // el Cliente completo asociado a cada dni_cliente. como get_by_dni devuelve
        // un unico Cliente por vez, es necesario iterar sobre todos los registros
        // para construir el Vec<Cliente> que retorna la funcion
        for registro in registros {
            let cliente = ClienteRepository::get_by_dni(pool, registro.dni_cliente).await?;

            clientes.push(cliente);
        }

        Ok(clientes)
    }
    pub async fn get_first(pool: &SqlitePool, id_espera: &str) -> Result<Cliente, ApiError> {
        let registro = sqlx::query!(
            r#"
                SELECT dni_cliente
                FROM cliente_lista_espera
                WHERE id_espera = ?
                ORDER BY fecha_ingreso ASC
                LIMIT 1
                "#,
            id_espera
        )
        .fetch_optional(pool)
        .await?;

        let registro = registro.ok_or(ApiError::NotFound)?;

        ClienteRepository::get_by_dni(pool, registro.dni_cliente).await
    }
    pub async fn delete_cliente(
        pool: &SqlitePool,
        id_espera: &str,
        dni_cliente: i64,
    ) -> Result<ClienteListaEspera, ApiError> {
        let registro = sqlx::query!(
            r#"
                SELECT
                    fecha_ingreso as "fecha_ingreso: NaiveDate"
                FROM cliente_lista_espera
                WHERE id_espera = ?
                AND dni_cliente = ?
                "#,
            id_espera,
            dni_cliente
        )
        .fetch_optional(pool)
        .await?;

        let registro = registro.ok_or(ApiError::NotFound)?;

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

        Ok(ClienteListaEspera::new(
            id_espera.to_string(),
            dni_cliente,
            registro.fecha_ingreso,
        ))
    }
    pub async fn delete_lista_espera(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<ListaEspera, ApiError> {
        let lista = Self::get_by_id(pool, id_espera).await?;

        sqlx::query!(
            r#"
            DELETE FROM cliente_lista_espera
            WHERE id_espera = ?
            "#,
            id_espera
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM lista_de_espera
            WHERE id_espera = ?
            "#,
            id_espera
        )
        .execute(pool)
        .await?;

        Ok(lista)
    }
}
