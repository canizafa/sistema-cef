use crate::{
    app::{errors::AppError, state::AppState},
    notificaciones::{
        self,
        dto::{NotificacionRequest, NotificacionUpdateRequest},
    },
    usuarios::cliente,
};
use axum::{Json, extract::State};

pub async fn notify_handler(Json(request): Json<NotificacionRequest>) -> Result<(), AppError> {
    notificaciones::service::notify(request).await?;
    Ok(())
}

pub async fn update_notify_date_client(
    State(state): State<AppState>,
    Json(request): Json<NotificacionUpdateRequest>,
) -> Result<(), AppError> {
    cliente::service::update_notify_date(&state.db, request.fecha).await?;
    Ok(())
}
