use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Cliente;

pub struct ClienteRepository;

impl ClienteRepository {
    pub fn create_cliente(State(pool): State<SqlitePool>, cliente: Cliente) {}
    pub fn list_clientes(State(pool): State<SqlitePool>) {}
    pub fn find_by_dni(State(pool): State<SqlitePool>, id: i32) {}
    pub fn find_by_email(State(pool): State<SqlitePool>, email: String) {}
    pub fn update_cliente(State(pool): State<SqlitePool>, cliente: Cliente) {}
    pub fn delete_cliete(State(pool): State<SqlitePool>, cliente: Cliente) {}
}
