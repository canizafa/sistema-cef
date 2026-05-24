use chrono::NaiveDate;

use crate::{domain::Cliente, dtos::CreateListaEsperaRequest};

#[derive(Debug, Clone)]
pub struct ListaEspera {
    id_lista: String,
    tipo: String,
    fecha_ingreso: NaiveDate,
    id_clase: String,
    clientes_en_espera: Vec<Cliente>,
}

impl ListaEspera {
    pub fn get_id_lista(&self) -> &str {
        &self.id_lista
    }
    pub fn get_tipo(&self) -> String {
        self.tipo.clone()
    }
    pub fn get_fecha_ingreso(&self) -> NaiveDate {
        self.fecha_ingreso.clone()
    }
    pub fn get_id_clase(&self) -> &str {
        &self.id_clase
    }
    pub fn get_clientes_en_espera(&self) -> Vec<Cliente> {
        self.clientes_en_espera.clone()
    }
}

impl From<CreateListaEsperaRequest> for ListaEspera {
    fn from(request: CreateListaEsperaRequest) -> Self {
        Self {
            id_lista: uuid::Uuid::new_v4().to_string(),
            tipo: request.tipo,
            fecha_ingreso: request.fecha,
            id_clase: request.id_clase,
            clientes_en_espera: Vec::new(),
        }
    }
}
