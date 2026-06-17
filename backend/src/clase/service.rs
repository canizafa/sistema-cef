use sqlx::SqlitePool;

use crate::{
    app::errors::{AppError, FieldError},
    clase::{domain::Clase, dto::CreateClaseRequest, repository::ClaseRepository},
    sala,
};

pub async fn create(
    db: &SqlitePool,
    request: CreateClaseRequest,
    id: &str,
) -> Result<Clase, AppError> {
    //Verificar si existe la sala
    let sala = sala::service::get_by_id(db, &request.id_sala).await?;

    //Verificar si existe por id
    let existing_clase = ClaseRepository::get_by_id(db, id).await;
    if existing_clase.is_ok() {
        return Err(AppError::Conflict("Clase ya existe".to_string()));
    }

    //Verificar si existe la sala
    let clase = Clase::from(request);
    let errors: Vec<FieldError> = clase
        .validate_clase(sala.get_capacidad_maxima())
        .into_iter()
        .map(FieldError::from)
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }

    //Verificar si no existe otra clase en el mismo dia y horario
    let clases = ClaseRepository::get_all(db).await?;
    let errors: Vec<FieldError> = clase
        .profesor_libre(&clases)
        .into_iter()
        .map(FieldError::from)
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }

    ClaseRepository::create(db, &clase).await?;
    Ok(clase)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Clase>, AppError> {
    let clases = ClaseRepository::get_all(db).await?;
    Ok(clases)
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<Clase, AppError> {
    let clase = ClaseRepository::get_by_id(db, id).await?;
    Ok(clase)
}

pub async fn update(
    db: &SqlitePool,
    id: &str,
    request: CreateClaseRequest,
) -> Result<Clase, AppError> {
    let _ = ClaseRepository::get_by_id(db, id).await?;
    let updated_clase = Clase::from(request);
    ClaseRepository::update(db, id, &updated_clase).await?;
    Ok(updated_clase)
}
pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    ClaseRepository::delete(db, id).await?;
    Ok(())
}
pub async fn descontar_cupo(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    let mut clase = ClaseRepository::get_by_id(db, id).await?;
    clase.descontar_cupo();
    ClaseRepository::update(db, id, &clase).await?;
    Ok(())
}
