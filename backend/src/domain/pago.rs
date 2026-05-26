use chrono::{Local, NaiveDate};

use crate::{domain::Reserva, dtos::CreatePagoRequest, errors::ApiError};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Pago {
    id_pago: String,
    monto: f64,
    fecha: NaiveDate,
    hora: String,
    sena: bool,
    id_membresia: Option<String>,
    reserva_paga: Option<String>, //NO SE PUEDE GUARDAR UN OPTION DE OTRA CLASE,
} // adeMAS SE GUARDA SOLO EL ID DE LA RESERVA
//LITERALMENTE NO SE Puede hacer eso en sql
// En SQL no existe como objeto, existe como relacion por ID (foreign key)
impl Pago {
    pub fn new(
        id_pago: String,
        monto: f64,
        fecha: NaiveDate,
        hora: String,
        sena: bool,
        id_membresia: Option<String>,
        reserva_paga: Option<String>,
    ) -> Self {
        Self {
            id_pago,
            monto,
            fecha,
            hora,
            sena,
            id_membresia,
            reserva_paga,
        }
    }

    pub fn get_id_pago(&self) -> &str {
        &self.id_pago
    }

    pub fn get_monto(&self) -> f64 {
        self.monto
    }

    pub fn get_fecha(&self) -> NaiveDate {
        self.fecha
    }

    pub fn get_hora(&self) -> String {
        self.hora.clone()
    }

    pub fn get_sena(&self) -> bool {
        self.sena
    }

    pub fn get_id_membresia(&self) -> Option<&String> {
        self.id_membresia.as_ref()
    }

    pub fn get_reserva_paga(&self) -> Option<&String> {
        self.reserva_paga.as_ref()
    }
    pub fn validate_pago(&self) -> Result<(), ApiError> {
        if self.monto <= 0.0 {
            return Err(ApiError::BadRequest(
                "El monto debe ser mayor a 0".to_string(),
            ));
        }
        Ok(())
    }
}

impl From<CreatePagoRequest> for Pago {
    fn from(request: CreatePagoRequest) -> Self {
        Self {
            id_pago: Uuid::new_v4().to_string(),
            monto: request.monto,
            fecha: request.fecha,
            hora: request.hora,
            sena: request.sena,
            id_membresia: Some(request.id_membresia),
            reserva_paga: Some(request.reserva_paga),
        }
    }
}
