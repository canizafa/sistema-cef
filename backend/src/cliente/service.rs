use super::{domain::Cliente, dto::CreateClienteRequest, repository::ClienteRepository};
use crate::{
    app::{
        errors::AppError,
        mailer::{self, Mailer},
    },
    auth,
    cliente::dto::{ClienteRequest, UpdatePasswordRequest},
    ficha_medica,
};
use sqlx::SqlitePool;

pub async fn create(db: &SqlitePool, request: CreateClienteRequest) -> Result<Cliente, AppError> {
    let CreateClienteRequest {
        dni,
        nombre_apellido,
        password,
        email,
        telefono,
        fecha_nacimiento,
        estado,
        ficha_medica,
    } = request.clone();

    //Verificamos si ya existe
    let existe = ClienteRepository::get_by_email(db, &email).await.is_ok();
    if existe {
        return Err(AppError::Conflict(
            "El email ya está registrado".to_string(),
        ));
    }
    //Hasheamos la contraseña
    let password_hash = auth::password::hash_password(&password)?;
    // Creamos id ficha medica
    let id_ficha = uuid::Uuid::new_v4().to_string();
    //Validamos el cliente
    let cliente = Cliente::try_from((request, password_hash, id_ficha.clone()))?;
    let errors = cliente.validate_cliente();
    if !errors.is_empty() {
        return Err(AppError::from(errors));
    }

    //Creamos la ficha medica
    let ficha_medica = ficha_medica::service::create(db, ficha_medica, &id_ficha).await?;

    ClienteRepository::create(db, &cliente, &id_ficha).await?;

    Ok(cliente)
}

pub async fn get_by_dni(db: &SqlitePool, dni: i64) -> Result<Cliente, AppError> {
    ClienteRepository::get_by_dni(db, dni)
        .await
        .map_err(AppError::from)
}

pub async fn get_by_email(db: &SqlitePool, email: &str) -> Result<Cliente, AppError> {
    ClienteRepository::get_by_email(db, email)
        .await
        .map_err(AppError::from)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Cliente>, AppError> {
    ClienteRepository::get_all(db).await.map_err(AppError::from)
}

pub async fn update_nombre(db: &SqlitePool, request: ClienteRequest) -> Result<Cliente, AppError> {
    let ClienteRequest {
        dni,
        nombre_apellido,
        email,
        telefono,
        fecha_nacimiento,
        estado,
        id_ficha,
    } = request;
    ClienteRepository::update_nombre(db, dni, &nombre_apellido)
        .await
        .map_err(AppError::from)
}

pub async fn update_password(
    db: &SqlitePool,
    request: UpdatePasswordRequest,
) -> Result<(), AppError> {
    let UpdatePasswordRequest {
        email,
        old_password,
        new_password,
    } = request;
    //Verificamos la password actual
    let mut cliente = ClienteRepository::get_by_email(db, &email)
        .await
        .map_err(AppError::from)?;
    if !auth::password::verify_password(&old_password, &cliente.get_password_hash()).is_ok() {
        return Err(AppError::Unauthorized(
            "Clave actual incorrecta, intente nuevamente".to_string(),
        ));
    }
    //Hasheamos la nueva password
    let new_hashed_password = auth::password::hash_password(&new_password)?;
    //Actualizamos la password en el cliente
    let errors = cliente.update_password(true, &new_hashed_password);
    if !errors.is_empty() {
        return Err(AppError::from(errors));
    }
    //Actualizamos la password en la base de datos
    ClienteRepository::update_password(db, &email, &new_hashed_password)
        .await
        .map_err(AppError::from)
}

pub async fn reset_password(db: &SqlitePool, email: &str, mailer: &Mailer) -> Result<(), AppError> {
    let random_password = auth::password::generate_random_password();
    let new_hashed_password = auth::password::hash_password(&random_password)?;
    ClienteRepository::update_password(db, &email, &new_hashed_password)
        .await
        .map_err(AppError::from)?;
    mailer::Mailer::send_new_password(mailer, email, &random_password)
        .await
        .map_err(AppError::from)
}

pub async fn update_estado(db: &SqlitePool, request: ClienteRequest) -> Result<Cliente, AppError> {
    let cliente = Cliente::try_from(request)?;
    ClienteRepository::update_estado(db, cliente.get_dni(), cliente.get_estado())
        .await
        .map_err(AppError::from)
}

pub async fn delete(db: &SqlitePool, dni: i64) -> Result<(), AppError> {
    ClienteRepository::delete(db, dni)
        .await
        .map_err(AppError::from)
}
