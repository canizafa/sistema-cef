pub mod actividad_dto;
pub mod asistencia_dto;
pub mod auth_dto;
pub mod clase_dto;
pub mod cliente_dto;
pub mod empleado_dto;
pub mod ficha_medica_dto;
pub mod lista_espera_dto;
pub mod membresia_dto;
pub mod mercado_pago_dto;
pub mod pago_dto;
pub mod profesor_dto;
pub mod reserva_dto;
pub mod sala_dto;

pub use actividad_dto::{ActividadResponse, CreateActividadRequest};
pub use asistencia_dto::{AsistenciaResponse, CreateAsistenciaRequest};
pub use auth_dto::{
    AuthResponse, CreateChangePasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest,
};
pub use clase_dto::{ClaseResponse, CreateClaseRequest};
pub use cliente_dto::{ClienteResponse, CreateClienteRequest};
pub use empleado_dto::{CreateEmpleadoRequest, EmpleadoResponse};
pub use ficha_medica_dto::{CreateFichaMedicaRequest, FichaMedicaResponse};
pub use lista_espera_dto::{
    CreateListaEsperaRequest, ListaEsperaListResponse, ListaEsperaResponse,
};
pub use membresia_dto::{CreateMembresiaRequest, MembresiaResponse};

pub use pago_dto::{CreatePagoRequest, PagoResponse};
pub use profesor_dto::{CreateProfesorRequest, ProfesorResponse};
pub use reserva_dto::{CreateReservaRequest, ReservaResponse};
