use chrono::NaiveDate;

use crate::domain::Cliente;

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
    pub fn get_tipo(&self) -> &str {
        &self.tipo
    }
    pub fn get_fecha_ingreso(&self) -> NaiveDate {
        self.fecha_ingreso
    }
    pub fn get_id_clase(&self) -> &str {
        &self.id_clase
    }
    pub fn get_clientes_en_espera(&self) -> &Vec<Cliente> {
        &self.clientes_en_espera
    }
}
