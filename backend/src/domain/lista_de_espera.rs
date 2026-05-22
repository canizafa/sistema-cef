use chrono::NaiveDate;

use crate::domain::Cliente;

pub struct ListaEspera {
    id_lista: i32,
    tipo: String,
    fecha_ingreso: NaiveDate,
    id_clase: i32,
    clientes_en_espera: Vec<Cliente>,
}

impl ListaEspera {
    pub fn get_id_lista(&self) -> i32 {
        self.id_lista
    }
    pub fn get_tipo(&self) -> &str {
        &self.tipo
    }
    pub fn get_fecha_ingreso(&self) -> NaiveDate {
        self.fecha_ingreso
    }
    pub fn get_id_clase(&self) -> i32 {
        self.id_clase
    }
    pub fn get_clientes_en_espera(&self) -> &Vec<Cliente> {
        &self.clientes_en_espera
    }
}
