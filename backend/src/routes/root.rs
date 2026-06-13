use super::*;
use crate::{
    actividad::actividad_router, app::AppState, asistencia::asistencia_router, auth::auth_router,
    clase::clase_router, cliente::cliente_router, empleado::empleado_router,
    membresia::membresia_router, pago::pago_router, profesor::profesor_router,
    reserva::reserva_router, sala::sala_router,
};
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/health", get(health_checker))
        .nest("/api/reservas", reserva_router())
        .nest("/api/membresias", membresia_router())
        .nest("/api/pagos", pago_router())
        .nest("/api/empleados", empleado_router())
        .nest("/api/clientes", cliente_router())
        .nest("/api/clase", clase_router())
        .nest("/api/auth", auth_router())
        .nest("/api/asistencia", asistencia_router())
        .nest("/api/profesores", profesor_router())
        .nest("/api/salas", sala_router())
        .nest("/api/actividades", actividad_router())
}
