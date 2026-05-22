use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Empleado;

pub struct EmpleadoRepository;

impl EmpleadoRepository {
    pub async fn create_empleado(State(pool): State<SqlitePool>, empleado: Empleado) {}
    pub async fn list_empleados(State(pool): State<SqlitePool>) {}
    pub async fn get_by_email(State(pool): State<SqlitePool>, email: String) {}
    pub async fn get_by_dni(State(pool): State<SqlitePool>, dni: i32) {}
    pub async fn update_empleado(State(pool): State<SqlitePool>, dni: i32, empleado: Empleado) {}
    pub async fn delete_empleado(State(pool): State<SqlitePool>, dni: i32) {}
}
