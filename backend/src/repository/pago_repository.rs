use sqlx::SqlitePool;

use crate::domain::pago::Pago;
use crate::errors::ApiError;

pub struct PagoRepository;

impl PagoRepository {
    pub async fn create_pago(pool: &SqlitePool, pago: Pago) -> Result<Pago, ApiError> {
        todo!()
    }
    pub async fn list_pagos(pool: &SqlitePool) -> Result<Option<Vec<Pago>>, ApiError> {
        todo!()
    }
    pub async fn get_pago(pool: &SqlitePool, id: &str) -> Result<Option<Pago>, ApiError> {
        todo!()
    }
    pub async fn update_pago(
        pool: &SqlitePool,
        id: &str,
        pago: Pago,
    ) -> Result<Option<Pago>, ApiError> {
        todo!()
    }
    pub async fn delete_pago(pool: &SqlitePool, id: &str) -> Result<Option<Pago>, ApiError> {
        todo!()
    }
}
