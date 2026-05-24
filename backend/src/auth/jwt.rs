use crate::auth::claims::Claims;
use crate::domain::Rol;
use crate::errors::{ApiError, AppError};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub fn generar_token(dni: &str, rol: Rol, secret: &str) -> Result<String, AppError> {
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: dni.to_string(),
        rol,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Api(ApiError::JwtError(e)))
}

pub fn validar_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Api(ApiError::JwtError(e)))
}
