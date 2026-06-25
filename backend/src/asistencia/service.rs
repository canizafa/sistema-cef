use sqlx::SqlitePool;

use crate::{
    app::errors::AppError,
    asistencia::{
        domain::Asistencia, dto::CreateAsistenciaRequest, repository::AsistenciaRepository,
    },
    reserva,
};

pub async fn create(
    db: &SqlitePool,
    request: CreateAsistenciaRequest,
) -> Result<Asistencia, AppError> {
    let asistencia = Asistencia::from(request);
    let mut reserva =
        reserva::repository::ReservaRepository::get_by_id(db, asistencia.get_id_reserva()).await?;
    //Verificar si ya existe una asistencia con el mismo id
    let existing_asistencia = AsistenciaRepository::get_by_id(db, &asistencia.id_asistencia)
        .await
        .ok();
    if existing_asistencia.is_some() {
        return Err(AppError::Conflict("Asistencia ya existe".to_string()));
    }
    AsistenciaRepository::create(db, &asistencia).await?;
    reserva.confirmar_reserva(); //marca como reserva confirmada
    reserva::repository::ReservaRepository::update(db, reserva.get_id(), &reserva).await?; //persistir el cambio
    Ok(asistencia)
}

pub async fn get_by_id(db: &SqlitePool, id_asistencia: &str) -> Result<Asistencia, AppError> {
    let asistencia = AsistenciaRepository::get_by_id(db, id_asistencia).await?;
    Ok(asistencia)
}

pub async fn update(
    db: &SqlitePool,
    id_asistencia: &str,
    request: CreateAsistenciaRequest,
) -> Result<Asistencia, AppError> {
    let existing_asistencia = AsistenciaRepository::get_by_id(db, id_asistencia)
        .await
        .ok();
    if existing_asistencia.is_none() {
        return Err(AppError::NotFound("Asistencia no encontrada".to_string()));
    }
    let asistencia = Asistencia::from(request);
    AsistenciaRepository::update(db, id_asistencia, &asistencia).await?;
    Ok(asistencia)
}

pub async fn delete(db: &SqlitePool, id_asistencia: &str) -> Result<(), AppError> {
    let existing_asistencia = AsistenciaRepository::get_by_id(db, id_asistencia)
        .await
        .ok();
    if existing_asistencia.is_none() {
        return Err(AppError::NotFound("Asistencia no encontrada".to_string()));
    }
    AsistenciaRepository::delete(db, id_asistencia).await?;
    Ok(())
}
