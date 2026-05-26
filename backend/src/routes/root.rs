use axum::{Router, routing::get};

use crate::{
    app_state::AppState,
    routes::{
        asistencia_router, auth_router, clase_router, cliente_router, empleado_router,
        health_checker, membresia_router, pago_router, profesor_router, reserva_router,
    },
};

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
}
