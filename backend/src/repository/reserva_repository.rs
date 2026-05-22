use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::models::reserva::CrearReserva;

pub async fn crear_reserva(
    State(pool): State<SqlitePool>,
    Json(reserva): Json<CrearReserva>,
) -> String {
    sqlx::query(
        "INSERT INTO Reserva
        (fecha, estado, dniCliente, idClase)
        VALUES (?,?, ?, ?, ?)",
    )
    .bind(&reserva.fecha)
    .bind(&reserva.estado)
    .bind(reserva.dni_cliente)
    .bind(reserva.id_clase)
    .execute(&pool)
    .await
    .unwrap();
    "reserva creada".to_string()
}
