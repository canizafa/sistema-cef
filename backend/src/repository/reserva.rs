use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize, Serialize)]
pub struct NuevaReserva {
    pub fecha: String,
    pub estado: String,
    pub dniCliente: i32,
    pub idClase: i32,
}

pub async fn crear_reserva(
    State(pool): State<SqlitePool>,
    Json(reserva): Json<NuevaReserva>,
) -> String {
    sqlx::query(
        "INSERT INTO Reserva
        (fecha, estado, dniCliente, idClase)
        VALUES (?,?, ?, ?, ?)",
    )
    .bind(&reserva.fecha)
    .bind(&reserva.estado)
    .bind(reserva.dniCliente)
    .bind(reserva.idClase)
    .execute(&pool)
    .await
    .unwrap();
    "reserva creada".to_string()
}
