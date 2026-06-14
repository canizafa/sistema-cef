use sqlx::SqlitePool;

use crate::{
    app::errors::AppError,
    clase::{domain::Clase, dto::CreateClaseRequest, repository::ClaseRepository},
};

pub async fn create(
    db: &SqlitePool,
    request: CreateClaseRequest,
    id: &str,
) -> Result<Clase, AppError> {
    //Verificar si existe por id
    let existing_clase = ClaseRepository::get_by_id(db, id).await;
    if existing_clase.is_ok() {
        return Err(AppError::Conflict("Clase ya existe".to_string()));
    }

    //Verificar si existe la sala

    let clase = Clase::from(request);
    let errors = clase.validate_clase();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }

    ClaseRepository::create(db, &clase).await?;
    Ok(clase)
}
