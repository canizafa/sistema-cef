use sqlx::SqlitePool;

use crate::{
    app::errors::{AppError, FieldError},
    pago::{domain::Pago, dto::CreatePagoRequest, repository::PagoRepository},
};

pub async fn create(db: &SqlitePool, request: CreatePagoRequest) -> Result<Pago, AppError> {
    //Crear pago de request
    let pago = Pago::from(request);
    let errors: Vec<FieldError> = pago
        .validate_pago()
        .into_iter()
        .map(|e| FieldError::from(e))
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    //Guardar pago
    PagoRepository::create(db, &pago)
        .await
        .map_err(AppError::from)?;
    Ok(pago)
}
