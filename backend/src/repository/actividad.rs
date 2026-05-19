use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
//estos structs los genero para que la funcion no me de error
// y pueda probar en bruno
// pero al final deberian ir en routes tambien asi que no andan igual
// podes copiarlo, son los atributos que tienen las tablas
// ES IMPORTANTE RESPETAR LOS ID DE CADA TABLA, ya que eso diferencia un objeto de otro
// te dejo los structs armados
#[derive(Deserialize, Serialize)]
pub struct NuevaActividad {
    pub nombre: String,
    pub descripcion: String,
}

pub async fn crear_actividad(
    State(pool): State<SqlitePool>,
    Json(actividad): Json<NuevaActividad>,
) -> String {
    sqlx::query(
        "INSERT INTO Actividad
        ( nombre, descripcion)
        VALUES (?, ?)",
    )
    .bind(&actividad.nombre)
    .bind(&actividad.descripcion)
    .execute(&pool)
    .await
    .unwrap();
    "actividad creada".to_string()
}
