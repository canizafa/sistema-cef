use crate::{
    app::errors::{AppError, FieldError},
    clase,
    reserva::{domain::Reserva, dto::CreateReservaRequest, repository::ReservaRepository},
};
use sqlx::SqlitePool;

pub async fn create(db: &SqlitePool, request: CreateReservaRequest) -> Result<Reserva, AppError> {
    //Validar si no existe una reserva para la misma actividad para ese mismo cliente
    let reserva = Reserva::from(request);
    let errors: Vec<FieldError> = reserva
        .validate_reserva()
        .into_iter()
        .map(FieldError::from)
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    let reservas_existentes = ReservaRepository::get_all(db)
        .await
        .map_err(AppError::from)?;
    if reservas_existentes.iter().any(|r| {
        r.get_id_clase() == reserva.get_id_clase()
            && r.get_dni_cliente() == reserva.get_dni_cliente()
    }) {
        return Err(AppError::Conflict(
            "Ya existe una reserva para esta actividad y cliente".to_string(),
        ));
    }
    //Descontar cupo de la clase
    clase::service::aumentar_inscripciones(db, &reserva.get_id_clase()).await?;
    //Guardar la reserva
    ReservaRepository::create(db, &reserva)
        .await
        .map_err(AppError::from)?;
    Ok(reserva)
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<Reserva, AppError> {
    ReservaRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Reserva>, AppError> {
    ReservaRepository::get_all(db).await.map_err(AppError::from)
}

pub async fn update(
    db: &SqlitePool,
    id: &str,
    request: CreateReservaRequest,
) -> Result<Reserva, AppError> {
    let reserva = Reserva::from(request);
    let errors: Vec<FieldError> = reserva
        .validate_reserva()
        .into_iter()
        .map(FieldError::from)
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    ReservaRepository::update(db, id, &reserva)
        .await
        .map_err(AppError::from)
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    ReservaRepository::delete(db, id)
        .await
        .map_err(AppError::from)?;
    clase::service::decrementar_inscripciones(db, id).await?;
    Ok(())
}
pub async fn delete_all_by_client(db: &SqlitePool, id: i64) -> Result<(), AppError> {
    let mut reservas = get_all(db).await.map_err(AppError::from)?;

    let reservas_cliente = reservas.iter_mut().filter(|r| r.get_dni_cliente() == id);

    for reserva in reservas_cliente {
        delete(db, reserva.get_id()).await?;
    }
    Ok(())
}
