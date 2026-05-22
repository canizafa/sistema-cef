use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Membresia;

pub struct MembresiaRepository;

impl MembresiaRepository {
    pub async fn create_membresia(State(pool): State<SqlitePool>, mem: Membresia) {}
    pub async fn list_membresias(State(pool): State<SqlitePool>) {}
    pub async fn get_membresia(State(pool): State<SqlitePool>, id: i32) {}
    pub async fn update_membresia(State(pool): State<SqlitePool>, id: i32, mem: Membresia) {}
    pub async fn delete_membresia(State(pool): State<SqlitePool>, id: i32) {}
}
