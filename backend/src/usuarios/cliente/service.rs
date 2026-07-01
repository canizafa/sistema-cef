use super::{domain::Cliente, dto::CreateClienteRequest, repository::ClienteRepository};
use crate::{
    app::{
        errors::AppError,
        mailer::{self, Mailer},
    },
    auth,
    ficha_medica::{self, domain::FichaMedica},
    reserva,
    usuarios::{
        cliente::dto::{ClienteRequest, EliminarClienteRequest, UpdatePasswordRequest},
        empleado,
    },
};
use sqlx::SqlitePool;

pub async fn update_cliente(db: &SqlitePool, request: ClienteRequest) -> Result<Cliente, AppError> {
    let cliente = ClienteRepository::get_by_dni(db, request.dni)
        .await
        .map_err(AppError::from)?;

    ClienteRepository::update_estado(
        db,
        cliente.get_dni(),
        request.estado,
        request.motivo_eliminacion,
    )
    .await
    .map_err(AppError::from)
}

pub async fn create(db: &SqlitePool, request: CreateClienteRequest) -> Result<Cliente, AppError> {
    //Verificamos si ya existe
    let existe = ClienteRepository::get_by_email(db, &request.email)
        .await
        .is_ok();
    if existe {
        return Err(AppError::Conflict(
            "El email ya está registrado".to_string(),
        ));
    }
    //Verificamos que no exista un empleado con el mismo email
    let existe_empleado = empleado::service::get_by_email(db, &request.email)
        .await
        .is_ok();
    if existe_empleado {
        return Err(AppError::Conflict(
            "El email ya está registrado".to_string(),
        ));
    }
    //Hasheamos la contraseña
    let password_hash = auth::password::hash_password(&request.password)?;
    // Creamos id ficha medica
    let id_ficha = uuid::Uuid::new_v4().to_string();
    //Validamos el cliente
    let cliente = Cliente::try_from((request.clone(), password_hash, id_ficha.clone()))?;
    let errors = cliente.validate_cliente();
    if !errors.is_empty() {
        return Err(AppError::from(errors));
    }

    //Creamos la ficha medica
    let _ = ficha_medica::service::create(db, request.ficha_medica, &id_ficha).await?;

    ClienteRepository::create(db, &cliente, &id_ficha).await?;

    Ok(cliente)
}

pub async fn get_by_dni(db: &SqlitePool, dni: i64) -> Result<(Cliente, FichaMedica), AppError> {
    let cliente = ClienteRepository::get_by_dni(db, dni)
        .await
        .map_err(AppError::from)?;
    let ficha = ficha_medica::service::get_by_id(db, &cliente.get_id_ficha())
        .await
        .map_err(AppError::from)?;
    Ok((cliente, ficha))
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
    ClienteRepository::update_nombre(db, request.dni, &request.nombre_apellido, &request.telefono)
        .await
        .map_err(AppError::from)
}

pub async fn change_password(
    db: &SqlitePool,
    dni: i64,
    old_password: &str,
    new_password: &str,
) -> Result<(), AppError> {
    let mut cliente = ClienteRepository::get_by_dni(db, dni)
        .await
        .map_err(AppError::from)?;
    if !auth::password::verify_password(&old_password, &cliente.get_password_hash()).is_ok() {
        return Err(AppError::Unauthorized(
            "Clave actual incorrecta, intente nuevamente".to_string(),
        ));
    }
    let new_password_hash = auth::password::hash_password(new_password)?;
    let errors = cliente.update_password(true, &new_password_hash);
    if !errors.is_empty() {
        return Err(AppError::from(errors));
    }
    ClienteRepository::update_password_by_dni(db, dni, &new_password_hash)
        .await
        .map_err(AppError::from)
}

pub async fn update_password(
    db: &SqlitePool,
    request: UpdatePasswordRequest,
) -> Result<(), AppError> {
    //Verificamos la password actual
    let mut cliente = ClienteRepository::get_by_email(db, &request.email)
        .await
        .map_err(AppError::from)?;
    if !auth::password::verify_password(&request.old_password, &cliente.get_password_hash()).is_ok()
    {
        return Err(AppError::Unauthorized(
            "Clave actual incorrecta, intente nuevamente".to_string(),
        ));
    }
    //Hasheamos la nueva password
    let new_hashed_password = auth::password::hash_password(&request.new_password)?;
    //Actualizamos la password en el cliente
    let errors = cliente.update_password(true, &new_hashed_password);
    if !errors.is_empty() {
        return Err(AppError::from(errors));
    }
    //Actualizamos la password en la base de datos
    ClienteRepository::update_password(db, &request.email, &new_hashed_password)
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
    ClienteRepository::update_estado(
        db,
        cliente.get_dni(),
        cliente.get_estado(),
        cliente.get_motivo_eliminacion(),
    )
    .await
    .map_err(AppError::from)
}
pub async fn update_creditos_y_cancelaciones(
    db: &SqlitePool,
    cliente: &Cliente,
) -> Result<Cliente, AppError> {
    ClienteRepository::update_creditos_y_cancelaciones(
        db,
        cliente.get_dni(),
        cliente.get_creditos(),
        cliente.get_contador_cancelaciones(),
    )
    .await
    .map_err(AppError::from)
}

pub async fn delete(db: &SqlitePool, request: EliminarClienteRequest) -> Result<(), AppError> {
    reserva::service::delete_all_by_client(db, request.dni).await?;
    ClienteRepository::delete(db, request.dni, request.estado, request.motivo_eliminacion)
        .await
        .map_err(AppError::from)
}

pub async fn login_cliente(
    db: &SqlitePool,
    email: &str,
    password: &str,
) -> Result<Cliente, AppError> {
    let cliente = ClienteRepository::get_by_email(db, email)
        .await
        .map_err(AppError::from)?;
    if auth::password::verify_password(password, &cliente.get_password_hash()).is_err() {
        return Err(AppError::InvalidCredentials);
    }
    Ok(cliente)
}
