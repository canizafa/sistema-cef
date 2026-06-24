use sqlx::SqlitePool;

use crate::{
    app::errors::{AppError, FieldError},
    clase,
    lista_espera::{
        cliente_espera::{self, dto::CreateClienteListaEsperaRequest},
        domain::ListaEspera,
        dto::CreateListaEsperaRequest,
        repository::ListaDeEsperaRepository,
    },
};

pub async fn create(
    db: &SqlitePool,
    request: CreateListaEsperaRequest,
) -> Result<ListaEspera, AppError> {
    //verifica que exista la clase
    clase::service::get_by_id(db, &request.id_clase).await?;
    let lista = ListaEspera::from(request);
    let errors: Vec<FieldError> = lista.validate().into_iter().map(|e| e.into()).collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    ListaDeEsperaRepository::create(db, &lista).await?;
    Ok(lista)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<ListaEspera>, AppError> {
    ListaDeEsperaRepository::get_all(db)
        .await
        .map_err(|e| AppError::from(e))
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<ListaEspera, AppError> {
    ListaDeEsperaRepository::get_by_id(db, id)
        .await
        .map_err(|e| AppError::from(e))
}

pub async fn insert_user(
    db: &SqlitePool,
    request: CreateClienteListaEsperaRequest,
    id_clase: &str,
) -> Result<(), AppError> {
    //verifica que exista la clase
    clase::service::get_by_id(db, id_clase).await?;
    let errors: Vec<FieldError> = cliente_espera::domain::ClienteListaEspera::from(request.clone())
        .validate()
        .into_iter()
        .map(|e| e.into())
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    cliente_espera::service::create(db, request).await?;
    Ok(())
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    ListaDeEsperaRepository::delete(db, id)
        .await
        .map_err(|e| AppError::from(e))
}
