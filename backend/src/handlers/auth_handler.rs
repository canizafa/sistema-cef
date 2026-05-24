use axum::{Json, extract::State};

use crate::{
    app_state::AppState,
    auth::generar_token,
    domain::Rol,
    dtos::{AuthResponse, LoginRequest},
    errors::ApiError,
    repository::{ClienteRepository, EmpleadoRepository},
};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json(AuthResponse), ApiError> {
    let (dni, email, password_hash, rol) = match body.rol {
        Rol::Cliente => {
            let cliente = ClienteRepository::find_by_email(&state.db, body.email.clone()).await?;
            if let Some(c) = cliente {
                (
                    c.get_dni(),
                    c.get_email(),
                    c.get_password_hash(),
                    Rol::Cliente,
                )
            } else {
                return Err(ApiError::Unauthorized);
            }
        }
        Rol::Profesor => {
            let profesor = EmpleadoRepository::get_by_email(&state.db, body.email.clone()).await?;
            if let Some(p) = profesor {
                (
                    p.get_dni(),
                    p.get_email(),
                    p.get_password_hash(),
                    Rol::Profesor,
                )
            } else {
                return Err(ApiError::Unauthorized);
            }
        }
        Rol::Empleado => {
            let empleado = EmpleadoRepository::get_by_email(&state.db, body.email.clone()).await?;
            if let Some(e) = empleado {
                (
                    e.get_dni(),
                    e.get_email(),
                    e.get_password_hash(),
                    Rol::Empleado,
                )
            } else {
                return Err(ApiError::Unauthorized);
            }
        }
        Rol::Duenio => {
            let duenio = EmpleadoRepository::get_by_email(&state.db, body.email.clone()).await?;
            if let Some(d) = duenio {
                (
                    d.get_dni(),
                    d.get_email(),
                    d.get_password_hash(),
                    Rol::Duenio,
                )
            } else {
                return Err(ApiError::Unauthorized);
            }
        }
        _ => return Err(ApiError::Unauthorized),
    };
    if password_hash != body.password {
        return Err(ApiError::Unauthorized);
    }

    let token = generar_token(&dni, rol.clone());

    Ok(Json(AuthResponse {
        dni,
        email,
        access_token: token,
        rol,
    }))
}
pub async fn logout_handler() {
    todo!()
}
pub async fn register_handler() {
    todo!()
}
pub async fn reset_password_handler() {
    todo!()
}
pub async fn register_empleado_handler() {
    todo!()
}
