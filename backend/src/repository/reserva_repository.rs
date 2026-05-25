use sqlx::SqlitePool;

use crate::domain::Reserva;
use crate::errors::ApiError;

pub struct ReservaRepository;

impl ReservaRepository {
    pub async fn create_reserva(pool: &SqlitePool, reserva: Reserva) -> Result<Reserva, ApiError> {
        todo!()
    }
    pub async fn list_reservas(pool: &SqlitePool) -> Result<Vec<Reserva>, ApiError> {
        todo!()
    }
    pub async fn get_reserva(pool: &SqlitePool, id: &str) -> Result<Reserva, ApiError> {
        todo!()
    }
    pub async fn update_reserva(pool: &SqlitePool, reserva: Reserva) -> Result<Reserva, ApiError> {
        todo!()
    }
    pub async fn delete_reserva(pool: &SqlitePool, id: &str) -> Result<Option<Reserva>, ApiError> {
        todo!()
    }
}
