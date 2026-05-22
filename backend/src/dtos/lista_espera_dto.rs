use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateListaEsperaRequest {
    pub dni_cliente: i32,
    pub id_clase: i32,
    pub fecha: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct ListaEsperaResponse {
    pub id_espera: i32,
    pub dni_cliente: i32,
    pub id_clase: i32,
    pub fecha: NaiveDate,
}
