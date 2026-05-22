use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Reserva;
use crate::errors::ApiError;

pub struct ReservaRepository;

impl ReservaRepository {
    pub async fn create_reserva(
        State(pool): State<SqlitePool>,
        reserva: Reserva,
    ) -> Result<Option<Reserva>, ApiError> {
        todo!()
    }
    pub async fn list_reservas(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Reserva>>, ApiError> {
        todo!()
    }
    pub async fn get_reserva(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Reserva>, ApiError> {
        todo!()
    }
    pub async fn update_reserva(
        State(pool): State<SqlitePool>,
        reserva: Reserva,
    ) -> Result<Option<Reserva>, ApiError> {
        todo!()
    }
    pub async fn delete_reserva(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Reserva>, ApiError> {
        todo!()
    }
}
