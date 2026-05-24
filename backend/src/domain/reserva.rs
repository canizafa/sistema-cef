use chrono::NaiveDate;

use crate::domain::Estado;

pub struct Reserva {
    id_reserva: String,
    estado: Estado,
    tipo: String,
    fecha_reserva: NaiveDate,
    id_clase: String,
}

impl Reserva {
    pub fn get_id(&self) -> &str {
        &self.id_reserva
    }

    pub fn get_estado(&self) -> &Estado {
        &self.estado
    }

    pub fn get_tipo(&self) -> &str {
        &self.tipo
    }

    pub fn get_fecha_reserva(&self) -> NaiveDate {
        self.fecha_reserva
    }

    pub fn get_id_clase(&self) -> &str {
        &self.id_clase
    }
}
