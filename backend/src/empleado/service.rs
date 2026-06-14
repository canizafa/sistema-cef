use crate::{
    app::errors::AppError,
    auth,
    empleado::{
        domain::Empleado,
        dto::{CreateEmpleadoRequest, UpdateEmpleadoRequest, UpdatePasswordRequest},
        repository::EmpleadoRepository,
    },
};
use sqlx::SqlitePool;

pub async fn create(db: &SqlitePool, request: CreateEmpleadoRequest) -> Result<Empleado, AppError> {
    // Verificar si ya existe
    let existing_empleado = EmpleadoRepository::get_by_email(db, &request.mail)
        .await
        .is_ok();
    if existing_empleado {
        return Err(AppError::Conflict("Empleado ya existe".to_string()));
    }
    //Hash password
    let password_hash = auth::password::hash_password(&request.password)?;
    // Crear desde la request
    let empleado = Empleado::from((request, password_hash));
    // Guardar en la base de datos
    EmpleadoRepository::create(db, &empleado)
        .await
        .map_err(AppError::from)?;
    Ok(empleado)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Empleado>, AppError> {
    EmpleadoRepository::get_all(db)
        .await
        .map_err(AppError::from)
}

pub async fn update(
    db: &SqlitePool,
    dni: i64,
    request: UpdateEmpleadoRequest,
) -> Result<Empleado, AppError> {
    // Crear desde la request
    let empleado = Empleado::from((request, "".to_owned()));
    EmpleadoRepository::update(db, dni, &empleado)
        .await
        .map_err(AppError::from)
}

pub async fn delete(db: &SqlitePool, dni: i64) -> Result<(), AppError> {
    EmpleadoRepository::delete(db, dni)
        .await
        .map_err(AppError::from)
}

pub async fn get_by_dni(db: &SqlitePool, dni: i64) -> Result<Empleado, AppError> {
    EmpleadoRepository::get_by_dni(db, dni)
        .await
        .map_err(AppError::from)
}

pub async fn get_by_email(db: &SqlitePool, email: &str) -> Result<Empleado, AppError> {
    EmpleadoRepository::get_by_email(db, email)
        .await
        .map_err(AppError::from)
}

pub async fn update_password_by_email(
    db: &SqlitePool,
    request: UpdatePasswordRequest,
) -> Result<(), AppError> {
    //Hash password
    let password_hash = auth::password::hash_password(&request.password)?;
    EmpleadoRepository::update_password_by_email(db, &request.email, &password_hash)
        .await
        .map_err(AppError::from)
}
