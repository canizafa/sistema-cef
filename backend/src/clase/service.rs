use chrono::{Duration, Utc, Weekday};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    app::errors::{AppError, FieldError},
    clase::{
        domain::Clase,
        dto::{CreateClaseRequest, UpdateClaseRequest},
        repository::ClaseRepository,
    },
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
    let mut clase = Clase::from(request);
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

    for _ in 0..3 {
        clase.cambiar_fecha(clase.get_dia() + Duration::days(7));
        clase.cambiar_id(Uuid::new_v4().to_string());
        ClaseRepository::create(db, &clase).await?;
    }

    Ok(clase)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Clase>, AppError> {
    let clases = ClaseRepository::get_all(db).await?;
    let yesterday = Utc::now().date_naive() - Duration::days(1);

    let clases = clases
        .into_iter()
        .filter(|c| c.get_dia() >= yesterday)
        .collect();
    Ok(clases)
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<Clase, AppError> {
    let clase = ClaseRepository::get_by_id(db, id).await?;
    Ok(clase)
}

pub async fn update(
    db: &SqlitePool,
    id: &str,
    request: UpdateClaseRequest,
) -> Result<Clase, AppError> {
    //Verificar si existe la clase antes de actualizar
    let mut clase = ClaseRepository::get_by_id(db, id).await?;
    //Actualizar los campos de la clase
    clase.update_clase(request.estado, request.dni_profesor);
    ClaseRepository::update(db, id, &clase).await?;
    Ok(clase)
}
pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    ClaseRepository::delete(db, id).await?;
    Ok(())
}
pub async fn aumentar_inscripciones(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    let mut clase = ClaseRepository::get_by_id(db, id).await?;
    let sala = sala::service::get_by_id(db, clase.get_id_sala()).await?;
    if clase.is_lleno() {
        return Err(AppError::Conflict(
            "La clase no tiene cupo disponible".to_string(),
        ));
    }
    clase.aumentar_inscripciones(sala.get_capacidad_maxima());
    ClaseRepository::update_inscripciones(db, id, clase.get_inscripciones(), clase.get_estado())
        .await?;
    Ok(())
}

pub async fn decrementar_inscripciones(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    let mut clase = ClaseRepository::get_by_id(db, id).await?;
    clase.decrementar_inscripciones();
    ClaseRepository::update_inscripciones(db, id, clase.get_inscripciones(), clase.get_estado())
        .await?;
    Ok(())
}

pub async fn get_all_by_actividad_horario(
    db: &SqlitePool,
    id_actividad: &str,
    horario: &str,
    dia: Weekday,
) -> Result<Vec<Clase>, AppError> {
    let clases = ClaseRepository::get_by_actividad_horario_dia(db, id_actividad, horario, dia)
        .await
        .map_err(AppError::from)?;

    let clases: Vec<Clase> = clases.into_iter().take(4).collect();
    Ok(clases)
}
