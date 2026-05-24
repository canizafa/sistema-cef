use crate::{domain::ListaEspera, errors::ApiError};
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub struct ListaDeEsperaRepository;

impl ListaDeEsperaRepository {
    pub async fn delete_lista_espera(
        pool: &SqlitePool,
        id_espera: &str,
    ) -> Result<ListaEspera, ApiError> {
        todo!()
    }
    pub async fn add_lista_espera(
        pool: &SqlitePool,
        id_espera: &str,
        dni_cliente: &str,
        id_clase: &str,
        fecha: NaiveDate,
    ) -> Result<ListaEspera, ApiError> {
        todo!()
    }
    pub async fn get_lista_espera(pool: &SqlitePool) -> Result<ListaEspera, ApiError> {
        todo!()
    }
}
