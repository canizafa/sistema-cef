use super::{
    domain::FichaMedica, dto::CreateFichaMedicaRequest, repository::FichaMedicaRepository,
};
use crate::app::errors::AppError;
use sqlx::SqlitePool;
use tracing::instrument;

#[instrument(skip_all, err)]
pub async fn create(
    db: &SqlitePool,
    request: CreateFichaMedicaRequest,
    id_ficha: &str,
) -> Result<FichaMedica, AppError> {
    let ficha_medica = FichaMedica::from(request);
    let existe = FichaMedicaRepository::get_by_id(db, id_ficha).await.is_ok();
    if existe {
        return Err(AppError::Conflict("Ficha medica ya existe".to_string()));
    }
    FichaMedicaRepository::create(db, &ficha_medica, id_ficha).await?;
    Ok(ficha_medica)
}
#[instrument(skip_all, err)]
pub async fn get_by_id(db: &SqlitePool, id_ficha: &str) -> Result<FichaMedica, AppError> {
    let ficha_medica = FichaMedicaRepository::get_by_id(db, id_ficha).await?;
    Ok(ficha_medica)
}
#[instrument(skip_all, err)]
pub async fn update(
    db: &SqlitePool,
    id_ficha: &str,
    request: CreateFichaMedicaRequest,
) -> Result<FichaMedica, AppError> {
    let ficha_medica = FichaMedica::from(request);
    FichaMedicaRepository::update(db, id_ficha, &ficha_medica).await?;
    Ok(ficha_medica)
}
#[instrument(skip_all, err)]
pub async fn delete(db: &SqlitePool, id_ficha: &str) -> Result<(), AppError> {
    let exists = FichaMedicaRepository::get_by_id(db, id_ficha).await.is_ok();
    if !exists {
        return Err(AppError::NotFound("Ficha medica no encontrada".to_string()));
    }
    FichaMedicaRepository::delete(db, id_ficha).await?;
    Ok(())
}
