use crate::{
    app::errors::AppError,
    pago::{domain::Pago, dto::CreatePagoRequest},
};

pub async fn create(db: &SqlitePool, request: CreatePagoRequest) -> Result<Pago, AppError> {}
