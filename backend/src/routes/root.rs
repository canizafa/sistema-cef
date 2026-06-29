use axum::{Router, routing::get};

use crate::{
    actividad::route::actividad_router,
    app::state::AppState,
    asistencia::route::asistencia_router,
    auth::route::auth_router,
    clase::route::clase_router,
    estadistica::route::estadistica_router,
    lista_espera::route::lista_espera_router,
    membresia::route::membresia_router,
    notificaciones::route::notificaciones_route,
    pago::route::pago_router,
    reserva::route::reserva_router,
    routes::health_checker,
    sala::route::sala_router,
    usuarios::{
        cliente::route::cliente_router, empleado::route::empleado_router,
        profesor_route::profesor_router,
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
        .nest("/api/notificaciones", notificaciones_route())
        .nest("/api/salas", sala_router())
        .nest("/api/actividades", actividad_router())
        .nest("/api/lista-espera", lista_espera_router())
        .nest("/api/estadisticas", estadistica_router())
}
