use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
#[derive(Deserialize, Serialize)]
pub struct NuevoProfesor {
    pub dni: i32,
    pub nombre: String,
    pub pellido: String,
    pub genero: String,
    pub estado: bool,
}

pub async fn crear_profesor(
    State(pool): State<SqlitePool>,
    Json(profesor): Json<NuevoProfesor>,
) -> String {
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
    "profe creado".to_string()
}
