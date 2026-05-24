pub mod clase_repository;
pub mod cliente_repository;
pub mod empleado_repository;
pub mod ficha_medica_repository;
pub mod lista_de_espera_repository;
pub mod membresia_repository;
pub mod pago_repository;
pub mod reserva_repository;

pub use clase_repository::ClaseRepository;
pub use cliente_repository::ClienteRepository;
pub use empleado_repository::EmpleadoRepository;
pub use ficha_medica_repository::FichaMedicaRepository;
pub use lista_de_espera_repository::ListaDeEsperaRepository;
pub use membresia_repository::MembresiaRepository;
pub use pago_repository::PagoRepository;
pub use reserva_repository::ReservaRepository;
