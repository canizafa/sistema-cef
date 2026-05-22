use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Cliente;
use crate::errors::ApiError;

pub struct ClienteRepository;

impl ClienteRepository {
    pub async fn create_cliente(
        State(pool): State<SqlitePool>,
        cliente: Cliente,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn list_clientes(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Cliente>>, ApiError> {
        todo!()
    }
    pub async fn find_by_dni(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn find_by_email(
        State(pool): State<SqlitePool>,
        email: String,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn update_cliente(
        State(pool): State<SqlitePool>,
        cliente: Cliente,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn delete_cliete(
        State(pool): State<SqlitePool>,
        cliente: Cliente,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
}
