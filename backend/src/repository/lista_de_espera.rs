use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize, Serialize)]
pub struct NuevaListaEspera {
    pub idLista: i32,
    pub dniCliente: i32,
    pub idClase: i32,
    pub fecha: String,
}

pub async fn agregar_lista_espera(
    State(pool): State<SqlitePool>,
    Json(lista): Json<NuevaListaEspera>,
) -> String {
    sqlx::query(
        "INSERT INTO ListaEspera
        (idCLiente,dniCliente, idClase, fecha)
        VALUES (?,?, ?, ?)",
    )
    .bind(lista.idLista)
    .bind(lista.dniCliente)
    .bind(lista.idClase)
    .bind(&lista.fecha)
    .execute(&pool)
    .await
    .unwrap();

    "cliente agregado a lista de espera".to_string()
}
