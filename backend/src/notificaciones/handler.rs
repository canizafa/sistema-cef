use crate::{
    app::errors::AppError,
    notificaciones::{self, dto::NotificacionRequest},
};
use axum::Json;

#[axum::debug_handler]
pub async fn notify_handler(Json(request): Json<NotificacionRequest>) -> Result<(), AppError> {
    notificaciones::service::notify(request).await?;
    Ok(())
}
