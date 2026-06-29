use crate::{
    app::{errors::AppError, state::AppState},
    notificaciones::{
        self,
        dto::{NotificacionRequest, NotificacionUpdateRequest},
    },
    usuarios::cliente,
};
use axum::{Json, extract::State};
use tracing::instrument;

#[instrument(name = "notificaciones.notify", skip(state, request), err)]
pub async fn notify_handler(Json(request): Json<NotificacionRequest>) -> Result<(), AppError> {
    notificaciones::service::notify(request).await?;
    Ok(())
}

#[instrument(name = "notificaciones.update_notify_date_client", skip(state, request), err)]
pub async fn update_notify_date_client(
    State(state): State<AppState>,
    Json(request): Json<NotificacionUpdateRequest>,
) -> Result<(), AppError> {
    cliente::service::update_notify_date(&state.db, &request.email, request.fecha).await?;
    Ok(())
}
