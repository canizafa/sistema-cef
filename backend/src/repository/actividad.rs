use crate::models::actividad::Actividad;
use crate::models::actividad::CrearActividad;
use axum::{Json, extract::State};
use sqlx::SqlitePool;
pub async fn crear_actividad(
    State(pool): State<SqlitePool>,
    Json(actividad): Json<CrearActividad>,
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
pub async fn mostrar_actividades(State(pool): State<SqlitePool>) -> Json<Vec<Actividad>> {
    let actividades = sqlx::query_as::<_, Actividad>(
        "SELECT id, nombre, descripcion
         FROM Actividad",
    )
    .fetch_all(&pool) //TRAE TODAS LAS ACTIVIDADES, tal vez te conviene dejarlo asi, se arma solo el vector sin nada raro
    .await //ACTIVIDAD HACE REFERENCIA A POR EJEMPLO YOGA, PILATES, FUNCIONAL
    .unwrap(); //NO HACE REFERENCIA A LA CLASE CON DIA Y HORARIO
    Json(actividades)
}
