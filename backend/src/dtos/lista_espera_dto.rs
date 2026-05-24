use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{domain::ListaEspera, dtos::ClienteResponse};

#[derive(Debug, Deserialize)]
pub struct CreateListaEsperaRequest {
    pub dni_cliente: String,
    pub id_clase: String,
    pub tipo: String,
    pub fecha: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct ListaEsperaResponse {
    pub id_espera: String,
    pub clientes_en_espera: Vec<ClienteResponse>,
    pub id_clase: String,
    pub tipo: String,
    pub fecha: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct ListaEsperaListResponse {
    pub lista_espera: Vec<ListaEsperaResponse>,
}

impl From<ListaEspera> for ListaEsperaResponse {
    fn from(lista_espera: ListaEspera) -> Self {
        Self {
            id_espera: lista_espera.get_id_lista(),
            clientes_en_espera: lista_espera
                .get_clientes_en_espera()
                .into_iter()
                .map(|c| c.into())
                .collect(),
            id_clase: lista_espera.get_id_clase(),
            tipo: lista_espera.get_tipo(),
            fecha: lista_espera.get_fecha_ingreso(),
        }
    }
}
impl From<Vec<ListaEspera>> for ListaEsperaListResponse {
    fn from(lista_espera: Vec<ListaEspera>) -> Self {
        Self {
            lista_espera: lista_espera.into_iter().map(|l| l.into()).collect(),
        }
    }
}
