use crate::app::errors::{AppError, FieldError};
use crate::lista_espera::{
    cliente_lista_espera::domain::ClienteListaEspera,
    cliente_lista_espera::dto::CreateClienteListaEsperaRequest,
    cliente_lista_espera::repository::ClienteListaEsperaRepository,
};
use crate::{cliente, lista_espera};
use sqlx::SqlitePool;

pub async fn create(
    db: &SqlitePool,
    request: CreateClienteListaEsperaRequest,
) -> Result<ClienteListaEspera, AppError> {
    let cliente = ClienteListaEspera::from(request);
    let mut errors: Vec<FieldError> = vec![];

    if cliente.get_dni_cliente() <= 0 {
        errors.push(FieldError::new("dni_cliente", "DNI invalido"));
    }

    if cliente.get_id_espera().trim().is_empty() {
        errors.push(FieldError::new("id_espera", "Lista de espera invalida"));
    }

    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }

    // valida la existencia de FK
    lista_espera_service::get_by_id(db, cliente.get_id_espera()).await?;

    cliente_service::get_by_dni(db, cliente.get_dni_cliente()).await?;
    ClienteListaEsperaRepository::create(db, &cliente).await
}

pub async fn get_all(
    db: &SqlitePool,
    id_espera: &str,
) -> Result<Vec<ClienteListaEspera>, AppError> {
    ClienteListaEsperaRepository::get_all(db, id_espera).await
}

pub async fn get_next(
    db: &SqlitePool,
    id_espera: &str,
) -> Result<Option<ClienteListaEspera>, AppError> {
    ClienteListaEsperaRepository::get_next(db, id_espera).await
}

pub async fn delete(
    db: &SqlitePool,
    id_espera: &str,
    dni_cliente: i64,
) -> Result<ClienteListaEspera, AppError> {
    ClienteListaEsperaRepository::delete(db, id_espera, dni_cliente).await
}
