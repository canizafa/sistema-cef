use super::domain::ListaEspera;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateListaEsperaRequest {
    pub id_clase: String,
    pub tipo: String,
}

#[derive(Debug, Serialize)]
pub struct ListaEsperaResponse {
    pub id_espera: String,
    pub tipo: String,
    pub id_clase: String,
}

impl From<ListaEspera> for ListaEsperaResponse {
    fn from(lista_espera: ListaEspera) -> Self {
        Self {
            id_espera: lista_espera.get_id_lista().to_string(),
            tipo: lista_espera.get_tipo().to_string(),
            id_clase: lista_espera.get_id_clase().to_string(),
        }
    }
}
