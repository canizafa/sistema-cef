use sqlx::SqlitePool;

use crate::domain::Cliente;
use crate::errors::ApiError;

pub struct ClienteRepository;

impl ClienteRepository {
    pub async fn create_cliente(
        pool: &SqlitePool,
        cliente: Cliente,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn list_clientes(pool: &SqlitePool) -> Result<Option<Vec<Cliente>>, ApiError> {
        todo!()
    }
    pub async fn find_by_dni(pool: &SqlitePool, id: i32) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn find_by_email(
        pool: &SqlitePool,
        email: String,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn update_cliente(
        pool: &SqlitePool,
        cliente: Cliente,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
    pub async fn delete_cliete(
        pool: &SqlitePool,
        cliente: Cliente,
    ) -> Result<Option<Cliente>, ApiError> {
        todo!()
    }
}
