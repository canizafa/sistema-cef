use crate::app::errors::{AppError, FieldError};
use crate::lista_espera;
use crate::lista_espera::{
    cliente_espera::domain::ClienteListaEspera,
    cliente_espera::dto::CreateClienteListaEsperaRequest,
    cliente_espera::repository::ClienteListaEsperaRepository,
};
use crate::usuarios::cliente;
use sqlx::SqlitePool;

pub async fn create(
    db: &SqlitePool,
    request: CreateClienteListaEsperaRequest,
) -> Result<ClienteListaEspera, AppError> {
    let cliente = ClienteListaEspera::from(request);
    let errors: Vec<FieldError> = cliente
        .validate()
        .into_iter()
        .map(FieldError::from)
        .collect();

    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }

    // valida la existencia de FK
    lista_espera::service::get_by_id(db, cliente.get_id_espera()).await?;

    cliente::service::get_by_dni(db, cliente.get_dni_cliente()).await?;
    ClienteListaEsperaRepository::create(db, &cliente)
        .await
        .map_err(AppError::from)
}

pub async fn get_all(
    db: &SqlitePool,
    id_espera: &str,
) -> Result<Vec<ClienteListaEspera>, AppError> {
    ClienteListaEsperaRepository::get_all(db, id_espera)
        .await
        .map_err(AppError::from)
}

pub async fn get_next(
    db: &SqlitePool,
    id_espera: &str,
) -> Result<Option<ClienteListaEspera>, AppError> {
    ClienteListaEsperaRepository::get_next(db, id_espera)
        .await
        .map_err(AppError::from)
}

pub async fn delete(
    db: &SqlitePool,
    id_espera: &str,
    dni_cliente: i64,
) -> Result<ClienteListaEspera, AppError> {
    ClienteListaEsperaRepository::delete(db, id_espera, dni_cliente)
        .await
        .map_err(AppError::from)
}
