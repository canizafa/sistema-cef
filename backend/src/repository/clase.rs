use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::models::clase::{Clase, CrearClase};

pub async fn crear_clase(State(pool): State<SqlitePool>, Json(clase): Json<CrearClase>) -> String {
    sqlx::query(
        "INSERT INTO Clase
        (dia, horario, cupoProfe, cupoMaximo, estado, idActividad, idSala, dniProfesor)
        VALUES (?,?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&clase.dia)
    .bind(&clase.horario)
    .bind(clase.cupo_profe)
    .bind(clase.cupo_maximo)
    .bind(&clase.estado)
    .bind(clase.id_actividad)
    .bind(clase.id_sala)
    .bind(clase.dni_profesor)
    .execute(&pool)
    .await
    .unwrap();
    "clase creada".to_string()
}
pub async fn ver_clases(State(pool): State<SqlitePool>) -> axum::Json<Vec<Clase>> {
    let clases = sqlx::query_as::<_, Clase>("SELECT dia, horario, cupoMaximo FROM Clase") //query_as ejecuta sql y convierte a struct
        .fetch_all(&pool) //trae todas las clases
        .await //CLASE SERIA EXPLICITAMENTE DIA COMISION HORARIO Y CUPO
        .unwrap();
    axum::Json(clases) // cambiar a rutas
}
