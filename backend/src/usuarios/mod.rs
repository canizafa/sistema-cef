pub use cliente::route as cliente_route;
pub use cliente::service as cliente_service;
pub use empleado::route as empleado_route;
pub use empleado::service as empleado_service;

pub mod cliente;
pub mod empleado;
mod estado;
mod genero;
mod profesor;
mod rol;
