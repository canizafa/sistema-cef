use super::*;
use crate::cliente::*;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct ClienteListaEspera {
    id_espera: String,
    dni_cliente: i64,
    fecha_ingreso: NaiveDate,
}

impl ClienteListaEspera {
    pub fn new(id_espera: String, dni_cliente: i64, fecha_ingreso: NaiveDate) -> Self {
        Self {
            id_espera,
            dni_cliente,
            fecha_ingreso,
        }
    }

    pub fn get_id_espera(&self) -> &str {
        &self.id_espera
    }

    pub fn get_dni_cliente(&self) -> i64 {
        self.dni_cliente
    }

    pub fn get_fecha_ingreso(&self) -> NaiveDate {
        self.fecha_ingreso
    }
}
