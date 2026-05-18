use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
//aca arme dos structs porque uno representa a una clase cuando se crea
// y otro representa cuando un cliente quiere acceder al listado de clases
// es decir, un cliente no podria acceder a los id, al cupo profesor, al estado,etc
// deberia ir un nombre en la clase? ej: clase de yoga, funcional, otro
#[derive(Deserialize, Serialize)]
pub struct NuevaClase {
    pub dia: String,
    pub horario: String,
    pub cupo_Profe: i32,
    pub cupo_Maximo: i32,
    pub estado: bool,
    pub idActividad: i32,
    pub idSala: i32,
    pub dni_Profesor: i32,
}
#[derive(Serialize, FromRow)] //clase que puede ver el cliente fromrow transforma una tupla en struct para poder devolverlo
pub struct Clase {
    pub dia: String,
    pub horario: String,
    pub cupoMaximo: i32,
}

pub async fn crear_clase(State(pool): State<SqlitePool>, Json(clase): Json<NuevaClase>) -> String {
    sqlx::query(
        "INSERT INTO Clase
        (dia, horario, cupoProfe, cupoMaximo, estado, idActividad, idSala, dniProfesor)
        VALUES (?,?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&clase.dia)
    .bind(&clase.horario)
    .bind(clase.cupo_Profe)
    .bind(clase.cupo_Maximo)
    .bind(&clase.estado)
    .bind(clase.idActividad)
    .bind(clase.idSala)
    .bind(clase.dni_Profesor)
    .execute(&pool)
    .await
    .unwrap();
    "clase creada".to_string()
}
pub async fn ver_clases(State(pool): State<SqlitePool>) -> axum::Json<Vec<Clase>> {
    let clases = sqlx::query_as::<_, Clase>("SELECT dia, horario, cupoMaximo FROM Clase") //query_as ejecuta sql y convierte a struct
        .fetch_all(&pool)
        .await
        .unwrap();
    axum::Json(clases)
}
