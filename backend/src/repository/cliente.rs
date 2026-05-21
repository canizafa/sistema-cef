use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::models::cliente::Cliente;

pub async fn crear_cliente(pool: &SqlitePool, cliente: Cliente) -> String {
    sqlx::query(
        "INSERT INTO Cliente
        (dni, nombre, email, telefono, fecha_nacimiento, estado, ficha)
        VALUES (?,?,?,?,?, ?,?)",
    )
    .bind(cliente.dni)
    .bind(&cliente.nombre)
    .bind(&cliente.email)
    .bind(&cliente.telefono)
    .bind(&cliente.fecha_nacimiento)
    .bind(&cliente.estado)
    .bind(&cliente.ficha)
    .execute(pool)
    .await
    .unwrap();
    "cliente creado".to_string()
}
