use crate::{
    app::errors::AppError,
    usuarios::profesor::{
        domain::Profesor,
        dto::{CreateProfesorRequest, EliminarProfesorRequest},
        repository::ProfesorRepository,
    },
};
use sqlx::SqlitePool;
use tracing::instrument;

#[instrument(skip_all, err)]
pub async fn create(
    pool: &SqlitePool,
    request: CreateProfesorRequest,
) -> Result<Profesor, AppError> {
    // Verificar si ya existe un profesor con el mismo DNI
    let profesor = ProfesorRepository::get_by_dni(pool, request.dni)
        .await
        .is_ok();
    if profesor {
        return Err(AppError::Conflict(
            "Error al registrar el profesor. Revisá los datos".to_string(),
        ));
    }
    // Crear el profesor
    let profesor = Profesor::new(
        request.dni,
        request.nombre_completo,
        request.genero,
        request.estado,
        None,
    );
    ProfesorRepository::create(pool, &profesor).await?;
    Ok(profesor)
}

#[instrument(skip_all, err)]
pub async fn update(
    pool: &SqlitePool,
    request: CreateProfesorRequest,
) -> Result<Profesor, AppError> {
    // Armamos el profesor con los datos NUEVOS que llegaron del form
    let profesor = Profesor::from(request);

    // Solo verificamos que exista en la base (sin pisar los datos nuevos)
    ProfesorRepository::get_by_dni(pool, profesor.get_dni()).await?;

    // Persistimos los datos nuevos
    let profesor = ProfesorRepository::update(pool, profesor.get_dni(), &profesor)
        .await
        .map_err(AppError::from)?;
    Ok(profesor)
}

#[instrument(skip_all, err)]
pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Profesor, AppError> {
    let profesor = ProfesorRepository::get_by_dni(pool, dni).await?;
    Ok(profesor)
}

#[instrument(skip_all, err)]
pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Profesor>, AppError> {
    let profesores = ProfesorRepository::get_all(pool).await?;
    Ok(profesores)
}

#[instrument(skip_all, err)]
pub async fn delete(pool: &SqlitePool, request: EliminarProfesorRequest) -> Result<(), AppError> {
    ProfesorRepository::delete(
        pool,
        request.profesor_dni,
        request.estado,
        &request.motivo_eliminacion,
    )
    .await
    .map_err(AppError::from)
}