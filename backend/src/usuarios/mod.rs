pub use cliente::route as cliente_route;
pub use cliente::service as cliente_service;
pub use empleado::route as empleado_route;
pub use empleado::service as empleado_service;
pub use profesor::route as profesor_route;
pub use profesor::service as profesor_service;

pub mod cliente;
pub mod empleado;
pub mod profesor;

mod estado;
mod genero;
mod rol;
