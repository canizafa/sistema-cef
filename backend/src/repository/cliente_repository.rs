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
    pub async fn find_by_dni(pool: &SqlitePool, dni: &str) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn reset_password(
        pool: &SqlitePool,
        dni: &str,
        password: &str,
    ) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn update_cliente(pool: &SqlitePool, cliente: &Cliente) -> Result<Cliente, ApiError> {
        todo!()
    }
    pub async fn delete_cliente(pool: &SqlitePool, cliente: &Cliente) -> Result<Cliente, ApiError> {
        todo!()
    }
}
