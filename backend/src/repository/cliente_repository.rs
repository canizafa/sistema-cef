use axum::{Json, extract::State};
use sqlx::SqlitePool;

pub struct ClienteRepository;

impl ClienteRepository {
    pub fn create_cliente(State(pool): State<SqlitePool>, cliente: Cliente) {}
}

// pub async fn crear_cliente(State(pool): State<SqlitePool>, Json(cliente): Json<Cliente>) -> String {
//     sqlx::query(
//         "INSERT INTO Cliente
//         (DNI, nombre,apellido, email, telefono, fechaNacimiento, estado, Ficha)
//         VALUES (?,?,?,?,?,?, ?,?)",
//     )
//     .bind(cliente.dni)
//     .bind(&cliente.nombre)
//     .bind(&cliente.email)
//     .bind(&cliente.telefono)
//     .bind(&cliente.fecha_nacimiento)
//     .bind(&cliente.estado)
//     .bind(&cliente.ficha)
//     .execute(&pool)
//     .await
//     .unwrap();
//     "cliente creado".to_string()
// }
