use crate::{
    app::errors::AppError,
    sala::{domain::Sala, dto::CreateSalaRequest, repository::SalaRepository},
};
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, request: CreateSalaRequest) -> Result<Sala, AppError> {
    // Convertir el request a un modelo Sala
    let sala = Sala::from(request);
    // Verificar si existe una sala con el mismo id
    let existing_sala = SalaRepository::get_by_id(pool, sala.get_id()).await.is_ok();
    if existing_sala {
        return Err(AppError::Conflict(
            "Sala con el mismo id ya existe".to_string(),
        ));
    }
    // Guardar la sala en la base de datos
    SalaRepository::create(pool, &sala)
        .await
        .map_err(AppError::from)
}

pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Sala, AppError> {
    SalaRepository::get_by_id(pool, id)
        .await
        .map_err(AppError::from)
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Sala>, AppError> {
    SalaRepository::get_all(pool).await.map_err(AppError::from)
}
