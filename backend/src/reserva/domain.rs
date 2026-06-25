use crate::reserva::{
    dto::CreateReservaRequest, errors::ReservaDomainError, estado::EstadoReserva,
};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Reserva {
    id_reserva: String,
    estado: EstadoReserva,
    tipo: String,
    fecha_reserva: NaiveDate,
    dni_cliente: i64,
    id_clase: String,
}

impl Reserva {
    pub fn new(
        id_reserva: String,
        estado: EstadoReserva,
        tipo: String,
        fecha_reserva: NaiveDate,
        dni_cliente: i64,
        id_clase: String,
    ) -> Self {
        Self {
            id_reserva,
            estado,
            tipo,
            fecha_reserva,
            dni_cliente,
            id_clase,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id_reserva
    }

    pub fn get_estado(&self) -> EstadoReserva {
        self.estado.clone()
    }

    pub fn get_tipo(&self) -> String {
        self.tipo.clone()
    }

    pub fn get_fecha_reserva(&self) -> NaiveDate {
        self.fecha_reserva.clone()
    }

    pub fn get_dni_cliente(&self) -> i64 {
        self.dni_cliente.clone()
    }

    pub fn get_id_clase(&self) -> String {
        self.id_clase.clone()
    }
    pub fn validate_reserva(&self) -> Vec<ReservaDomainError> {
        let mut vec_errors = Vec::new();
        if self.tipo.is_empty() {
            vec_errors.push(ReservaDomainError::Tipo);
        }
        if self.fecha_reserva == NaiveDate::MIN {
            vec_errors.push(ReservaDomainError::FechaReserva);
        }
        if self.dni_cliente == 0 {
            vec_errors.push(ReservaDomainError::DniCliente);
        }
        if self.id_clase.is_empty() {
            vec_errors.push(ReservaDomainError::IdClase);
        }
        vec_errors
    }
    pub fn cancelar_reserva(&mut self) {
        self.estado = EstadoReserva::Cancelada;
    }
    pub fn confirmar_reserva(&mut self) {
        self.estado = EstadoReserva::Confirmada;
    }
}

impl From<CreateReservaRequest> for Reserva {
    fn from(request: CreateReservaRequest) -> Self {
        Self {
            id_reserva: Uuid::new_v4().to_string(),
            estado: request.estado,
            tipo: request.tipo,
            fecha_reserva: request.fecha,
            dni_cliente: request.dni_cliente,
            id_clase: request.id_clase,
        }
    }
}
