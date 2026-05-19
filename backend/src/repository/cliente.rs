use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
#[derive(Deserialize, Serialize)]
pub struct NuevoCliente {
    pub dni: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_Nacimiento: String,
    pub estado: String,
    pub ficha: String,
}

pub async fn crear_cliente(
    State(pool): State<SqlitePool>,
    Json(cliente): Json<NuevoCliente>,
) -> String {
    sqlx::query(
        "INSERT INTO Cliente
        (DNI, nombre,apellido, email, telefono, fechaNacimiento, estado, Ficha)
        VALUES (?,?,?,?,?,?, ?,?)",
    )
    .bind(cliente.dni)
    .bind(&cliente.nombre)
    .bind(&cliente.apellido)
    .bind(&cliente.email)
    .bind(&cliente.telefono)
    .bind(&cliente.fecha_Nacimiento)
    .bind(&cliente.estado)
    .bind(&cliente.ficha)
    .execute(&pool)
    .await
    .unwrap();
    "cliente creado".to_string()
}
