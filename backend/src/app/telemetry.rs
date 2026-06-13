use tracing_subscriber::{fmt, EnvFilter};

/// Inicializa `tracing` + subscriber con filtro por entorno.
///
/// Usá `RUST_LOG` para ajustar niveles, por ejemplo:
/// `RUST_LOG=info,backend=debug,tower_http=debug,sqlx=warn`
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info,backend=info,tower_http=info,axum=info,sqlx=warn")
    });

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .compact()
        .init();
}
