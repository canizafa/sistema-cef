use super::{dto::CreateActividadRequest, repository::ActividadRepository};
use crate::actividad::Actividad;
use crate::app::errors::AppError;
use sqlx::SqlitePool;

pub async fn create(
    db: &SqlitePool,
    request: CreateActividadRequest,
) -> Result<Actividad, AppError> {
    let actividad = Actividad::from(request);
    let errors = actividad.validate_actividad();
    if !errors.is_empty() {
        return Err(AppError::from(errors));
    }
    let existe = ActividadRepository::get_by_id(db, &actividad.id)
        .await
        .is_ok();
    if existe {
        return Err(AppError::Conflict("Actividad ya existe".into()));
    }
    ActividadRepository::create(db, &actividad)
        .await
        .map_err(AppError::from)?;
    Ok(actividad)
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<Actividad, AppError> {
    ActividadRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Actividad>, AppError> {
    ActividadRepository::get_all(db)
        .await
        .map_err(AppError::from)
}

pub async fn update(
    db: &SqlitePool,
    id: &str,
    request: CreateActividadRequest,
) -> Result<Actividad, AppError> {
    let nueva_actividad = Actividad::from(request);
    let actividad = ActividadRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)?;
    actividad.update_actividad(nueva_actividad);
    let errors = actividad.validate_actividad();
    if !errors.is_empty() {
        return Err(AppError::from(errors));
    }
    ActividadRepository::update(db, id, &actividad)
        .await
        .map_err(AppError::from)?;
    Ok(actividad)
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    ActividadRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)?;
    ActividadRepository::delete(db, id)
        .await
        .map_err(AppError::from)
}
