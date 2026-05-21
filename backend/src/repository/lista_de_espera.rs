use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::models::lista_espera::CrearListaEspera;

pub async fn agregar_lista_espera(
    State(pool): State<SqlitePool>,
    Json(lista): Json<CrearListaEspera>,
) -> String {
    sqlx::query(
        "INSERT INTO ListaEspera
        (idCLiente,dniCliente, idClase, fecha)
        VALUES (?,?, ?, ?)",
    )
    .bind(lista.id_espera)
    .bind(lista.dni_cliente)
    .bind(lista.id_clase)
    .bind(&lista.fecha)
    .execute(&pool)
    .await
    .unwrap();

    "cliente agregado a lista de espera".to_string()
}
