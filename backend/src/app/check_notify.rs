use std::{sync::Arc, time::Duration};

use chrono::Local;
use sqlx::SqlitePool;
use tokio::time::interval;

use crate::{notificaciones, usuarios::cliente_service};

pub async fn check_notify(pool: Arc<SqlitePool>) {
    let mut ticker = interval(Duration::from_secs((60 * 5) * 60));
    loop {
        ticker.tick().await;

        let today = Local::now().date_naive();

        match cliente_service::get_all(&pool).await {
            Ok(clientes) => {
                for cliente in clientes {
                    if let Some(fecha) = cliente.get_fecha_notificacion() {
                        if fecha.eq(&today) {
                            let _ = notificaciones::service::notify_date(
                                &cliente.get_mail(),
                                "Su membresía ha vencido, renuevela",
                                "Membresía vencida",
                            )
                            .await;
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error buscando clientes, {}", e),
        }
    }
}
