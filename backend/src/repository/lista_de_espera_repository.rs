use crate::{domain::ListaEspera, errors::ApiError};
use axum::extract::State;
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub struct ListaDeEsperaRepository;

impl ListaDeEsperaRepository {
    pub async fn delete_lista_espera(
        State(pool): State<SqlitePool>,
        id_espera: i32,
    ) -> Result<Option<ListaEspera>, ApiError> {
        todo!()
    }
    pub async fn add_lista_espera(
        State(pool): State<SqlitePool>,
        id_espera: i32,
        dni_cliente: i32,
        id_clase: i32,
        fecha: NaiveDate,
    ) -> Result<Option<ListaEspera>, ApiError> {
        todo!()
    }
    pub async fn get_lista_espera(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<ListaEspera>, ApiError> {
        todo!()
    }
}
