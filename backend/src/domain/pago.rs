use chrono::NaiveDate;

use crate::domain::Reserva;

pub struct Pago {
    id_pago: String,
    monto: f64,
    fecha: NaiveDate,
    hora: String,
    sena: bool,
    id_membresia: String,
    reserva_paga: Reserva,
}

impl Pago {
    pub fn get_id_pago(&self) -> &str {
        &self.id_pago
    }

    pub fn get_monto(&self) -> f64 {
        self.monto
    }

    pub fn get_fecha(&self) -> NaiveDate {
        self.fecha
    }

    pub fn get_hora(&self) -> &str {
        &self.hora
    }

    pub fn get_sena(&self) -> bool {
        self.sena
    }

    pub fn get_id_membresia(&self) -> &str {
        &self.id_membresia
    }

    pub fn get_reserva_paga(&self) -> &Reserva {
        &self.reserva_paga
    }
}
