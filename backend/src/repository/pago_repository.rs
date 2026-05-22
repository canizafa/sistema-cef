use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::pago::Pago;
use crate::errors::ApiError;

pub struct PagoRepository;

impl PagoRepository {
    pub async fn create_pago(State(pool): State<SqlitePool>, pago: Pago) -> Result<Pago, ApiError> {
        todo!()
    }
    pub async fn list_pagos(State(pool): State<SqlitePool>) -> Result<Option<Vec<Pago>>, ApiError> {
        todo!()
    }
    pub async fn get_pago(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Pago>, ApiError> {
        todo!()
    }
    pub async fn update_pago(
        State(pool): State<SqlitePool>,
        id: i32,
        pago: Pago,
    ) -> Result<Option<Pago>, ApiError> {
        todo!()
    }
    pub async fn delete_pago(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Pago>, ApiError> {
        todo!()
    }
}
