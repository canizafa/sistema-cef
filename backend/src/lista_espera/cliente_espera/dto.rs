use super::domain::ClienteListaEspera;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddClienteListaEsperaRequest {
    pub id_espera: String,
    pub dni_cliente: i64,
    pub fecha_ingreso: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct DeleteClienteListaEsperaRequest {
    pub id_espera: String,
    pub dni_cliente: i64,
}

#[derive(Debug, Serialize)]
pub struct ClienteListaEsperaResponse {
    pub id_espera: String,
    pub dni_cliente: i64,
    pub fecha_ingreso: NaiveDate,
}

impl From<ClienteListaEspera> for ClienteListaEsperaResponse {
    fn from(cliente: ClienteListaEspera) -> Self {
        Self {
            id_espera: cliente.get_id_espera(),
            dni_cliente: cliente.get_dni_cliente(),
            fecha_ingreso: cliente.get_fecha_ingreso(),
        }
    }
}
