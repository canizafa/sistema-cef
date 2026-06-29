use crate::app::errors::{AppError, FieldError};
use chrono::NaiveDate;
use sqlx::SqlitePool;
use tracing::instrument;

use super::{
    domain::{ClaseMasCancelada, ClaseMasConcurrida, Recaudacion},
    repository::EstadisticaRepository,
};
#[instrument(skip_all, err)]
fn validar_fechas(desde: NaiveDate, hasta: NaiveDate) -> Result<(), AppError> {
    if desde > hasta {
        return Err(AppError::Validation(vec![FieldError {
            field: "fecha_desde".into(),
            message: "la fecha desde no puede ser mayor que la fecha hasta".into(),
        }]));
    }

    Ok(())
}

#[instrument(skip_all, err)]
pub async fn obtener_clase_mas_concurrida(
    db: &SqlitePool,
    desde: NaiveDate,
    hasta: NaiveDate,
) -> Result<ClaseMasConcurrida, AppError> {
    validar_fechas(desde, hasta)?;

    Ok(EstadisticaRepository::clase_mas_concurrida(db, desde, hasta).await?)
}

#[instrument(skip_all, err)]
pub async fn obtener_clase_mas_cancelada(
    db: &SqlitePool,
    desde: NaiveDate,
    hasta: NaiveDate,
) -> Result<ClaseMasCancelada, AppError> {
    validar_fechas(desde, hasta)?;

    Ok(EstadisticaRepository::clase_mas_cancelada(db, desde, hasta).await?)
}

#[instrument(skip_all, err)]
pub async fn obtener_recaudacion(
    db: &SqlitePool,
    desde: NaiveDate,
    hasta: NaiveDate,
) -> Result<Recaudacion, AppError> {
    validar_fechas(desde, hasta)?;

    Ok(EstadisticaRepository::recaudacion(db, desde, hasta).await?)
}
