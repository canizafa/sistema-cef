use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::FichaMedica;

pub struct FichaMedicaRepository;

impl FichaMedicaRepository {
    pub async fn create_ficha_medica(State(pool): State<SqlitePool>, ficha_medica: FichaMedica) {}
    pub async fn get_by_dni(State(pool): State<SqlitePool>, dni: i32) {}
    pub async fn update_ficha_medica(
        State(pool): State<SqlitePool>,
        dni: i32,
        ficha_medica: FichaMedica,
    ) {
    }
    pub async fn delete_ficha_medica(State(pool): State<SqlitePool>, dni: i32) {}
}
