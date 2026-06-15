use sqlx::SqlitePool;

use crate::{
    app::errors::{AppError, FieldError},
    cliente,
    membresia::{domain::Membresia, dto::CreateMembresiaRequest, repository::MembresiaRepository},
};

pub async fn create(
    db: &SqlitePool,
    request: CreateMembresiaRequest,
) -> Result<Membresia, AppError> {
    //Transformar de la request
    let membresia = Membresia::from(request);
    let errors: Vec<FieldError> = membresia
        .validate_membresia()
        .into_iter()
        .map(FieldError::from)
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    //Verificar si el cliente existe
    let _ = cliente::service::get_by_dni(db, membresia.get_dni_cliente())
        .await
        .map_err(AppError::from)?;
    //Verificar si ya tiene una membresia de esa actividad/tipo
    let membresias_cliente = MembresiaRepository::get_by_dni(db, membresia.get_dni_cliente())
        .await
        .map_err(AppError::from)?;

    if membresias_cliente
        .iter()
        .any(|m| m.get_tipo_actividad().eq(&membresia.get_tipo_actividad()))
    {
        return Err(AppError::Conflict(
            "Ya existe una membresia para este cliente".to_string(),
        ));
    }
    // Si no creamos la membresia
    MembresiaRepository::create(db, &membresia)
        .await
        .map_err(AppError::from)?;
    Ok(membresia)
}

pub async fn get_by_all_by_dni(db: &SqlitePool, dni: i64) -> Result<Vec<Membresia>, AppError> {
    let membresias = MembresiaRepository::get_by_dni(db, dni)
        .await
        .map_err(AppError::from)?;
    Ok(membresias)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Membresia>, AppError> {
    let membresias = MembresiaRepository::get_all(db)
        .await
        .map_err(AppError::from)?;
    Ok(membresias)
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<Membresia, AppError> {
    let membresia = MembresiaRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)?;
    Ok(membresia)
}

pub async fn update(
    db: &SqlitePool,
    id: &str,
    request: CreateMembresiaRequest,
) -> Result<Membresia, AppError> {
    let membresia = Membresia::from(request);
    MembresiaRepository::update(db, id, &membresia)
        .await
        .map_err(AppError::from)?;
    Ok(membresia)
}

pub async fn delete_by_id(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    MembresiaRepository::delete(db, id)
        .await
        .map_err(AppError::from)?;
    Ok(())
}
