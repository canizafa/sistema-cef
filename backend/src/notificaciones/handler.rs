use crate::{
    app::{errors::AppError, state::AppState},
    notificaciones::{
        self,
        dto::{NotificacionRequest, NotificacionUpdateRequest},
    },
};
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "notificaciones.notify", skip(request), err)]
pub async fn notify_handler(Json(request): Json<NotificacionRequest>) -> Result<(), AppError> {
    notificaciones::service::notify(request).await?;
    Ok(())
}

#[instrument(
    name = "notificaciones.update_notify_date_client",
    skip(state, request),
    err
)]
pub async fn update_notify_date_client(
    State(state): State<AppState>,
    Json(request): Json<NotificacionUpdateRequest>,
) -> Result<(), AppError> {
    if request.dias < 0 || request.dias > 90 {
        return Err(AppError::Conflict(
            "Los días deben estar entre 0 y 90".to_string(),
        ));
    }
    let mut dias = state.dias_gracia.lock().unwrap();
    *dias = request.dias;
    Ok(())
}
