use sqlx::SqlitePool;

use crate::domain::Cliente;
use crate::errors::ApiError;

pub struct ClienteRepository;

impl ClienteRepository {
    pub async fn create_cliente(pool: &SqlitePool, cliente: &Cliente) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn list_clientes(pool: &SqlitePool) -> Result<Vec<Cliente>, ApiError> {
        todo!()
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn update_cliente(
        pool: &SqlitePool,
        id: i64,
        cliente: &Cliente,
    ) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn delete_cliente(pool: &SqlitePool, id: i64) -> Result<Cliente, ApiError> {
        todo!()
    }
}
