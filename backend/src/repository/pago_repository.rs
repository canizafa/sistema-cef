use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::pago::Pago;

pub struct PagoRepository;

impl PagoRepository {
    pub async fn create_pago(State(pool): State<SqlitePool>, pago: Pago) {}
    pub async fn list_pagos(State(pool): State<SqlitePool>) {}
    pub async fn get_pago(State(pool): State<SqlitePool>, id: i32) {}
    pub async fn update_pago(State(pool): State<SqlitePool>, id: i32, pago: Pago) {}
    pub async fn delete_pago(State(pool): State<SqlitePool>, id: i32) {}
}
