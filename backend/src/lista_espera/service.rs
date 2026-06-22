use sqlx::SqlitePool;

use crate::{
    app::errors::{AppError, FieldError},
    clase,
    lista_espera::{
        domain::ListaEspera, dto::CreateListaEsperaRequest, repository::ListaEsperaRepository,
    },
};

pub async fn create(
    db: &SqlitePool,
    request: CreateListaEsperaRequest,
) -> Result<ListaEspera, AppError> {
    //verifica que exista la clase
    clase::service::get_by_id(db, &request.id_clase).await?;
    let lista = ListaEspera::from(request);
    let errors: Vec<FieldError> = lista.validate();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    ListaEsperaRepository::create(db, &lista).await?;
    Ok(lista)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<ListaEspera>, AppError> {
    Ok(ListaEsperaRepository::get_all(db).await?)
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<ListaEspera, AppError> {
    Ok(ListaEsperaRepository::get_by_id(db, id).await?)
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    ListaEsperaRepository::delete(db, id).await?;
    Ok(())
}
