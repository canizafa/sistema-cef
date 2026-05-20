use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::models::profesor::CrearProfesor;

pub async fn crear_profesor(State(pool): State<SqlitePool>, Json(profesor): Json<CrearProfesor>) {
    sqlx::query(
        "INSERT INTO Profesor
        (DNI, nombre,pellido, genero, estado)
        VALUES (?,?,?, ?, ?, ?)",
    )
    .bind(profesor.dni)
    .bind(&profesor.nombre)
    .bind(&profesor.pellido)
    .bind(&profesor.genero)
    .bind(profesor.estado)
    .execute(&pool)
    .await
    .unwrap();
}
