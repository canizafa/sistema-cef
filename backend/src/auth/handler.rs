use crate::{
    app::{errors::AppError, state::AppState},
    auth::{
        self,
        dto::{AuthResponse, CreateChangePasswordRequest, LoginRequest, ResetPasswordRequest},
    },
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::instrument;

#[instrument(name = "auth.login", skip(state, request), fields(email = %request.email), err)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let auth = auth::service::login(&state.db, request).await?;
    Ok(Json(auth))
}

#[instrument(name = "auth.reset_password", skip(state, request), fields(email = %request.email), err)]
pub async fn reset_password_handler(
    State(state): State<AppState>,
    Json(request): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    auth::service::reset_password(&state.db, &request.email, &state.mailer).await?;
    Ok(StatusCode::OK)
}

#[instrument(name = "auth.change_password", skip(state, body), fields(dni = dni), err)]
pub async fn change_password_handler(
    State(state): State<AppState>,
    Path(dni): Path<i64>,
    Json(body): Json<CreateChangePasswordRequest>,
) -> Result<StatusCode, AppError> {
    auth::service::change_password(&state.db, dni, &body.old_password, &body.new_password).await?;
    Ok(StatusCode::OK)
}
