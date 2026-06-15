use crate::{
    app::{
        errors::AppError,
        mailer::{self, Mailer},
    },
    auth, cliente,
    empleado::{
        domain::Empleado,
        dto::{CreateEmpleadoRequest, UpdateEmpleadoRequest},
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
    //Verificamos que no exista un cliente con el mismo email
    let existe_cliente = cliente::service::get_by_email(db, &request.mail)
        .await
        .is_ok();
    if existe_cliente {
        return Err(AppError::Conflict(
            "El email ya está registrado".to_string(),
        ));
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

pub async fn reset_password(db: &SqlitePool, email: &str, mailer: &Mailer) -> Result<(), AppError> {
    //Generar contraseña
    let password = auth::password::generate_random_password();
    //Hash password
    let password_hash = auth::password::hash_password(&password)?;
    EmpleadoRepository::update_password_by_email(db, email, &password_hash)
        .await
        .map_err(AppError::from)?;
    //Avisar al usuario
    mailer::Mailer::send_new_password(mailer, email, &password)
        .await
        .map_err(AppError::from)?;
    Ok(())
}

pub async fn change_password(
    db: &SqlitePool,
    dni: i64,
    old_password: &str,
    new_password: &str,
) -> Result<(), AppError> {
    //verificar si la contraseña actual es correcta
    let empleado = EmpleadoRepository::get_by_dni(db, dni)
        .await
        .map_err(AppError::from)?;
    if auth::password::verify_password(old_password, &empleado.get_password_hash()).is_err() {
        return Err(AppError::InvalidCredentials);
    }
    EmpleadoRepository::update_password_by_dni(db, dni, new_password)
        .await
        .map_err(AppError::from)
}

pub async fn login_empleado(
    db: &SqlitePool,
    email: &str,
    password: &str,
) -> Result<Empleado, AppError> {
    let empleado = EmpleadoRepository::get_by_email(db, email)
        .await
        .map_err(AppError::from)?;
    if auth::password::verify_password(&empleado.get_password_hash(), password).is_err() {
        return Err(AppError::InvalidCredentials);
    }
    Ok(empleado)
}
