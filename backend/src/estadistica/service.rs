use chrono::NaiveDate;
use sqlx::SqlitePool;

use crate::app::errors::AppError;

use super::{
    domain::{ClaseMasCancelada, ClaseMasConcurrida, Recaudacion},
    repository::EstadisticaRepository,
};

pub async fn obtener_clase_mas_concurrida(
    db: &SqlitePool,
    desde: NaiveDate,
    hasta: NaiveDate,
) -> Result<ClaseMasConcurrida, AppError> {
    if desde > hasta {
        return Err(AppError::Validation(
            "la fecha desde no puede ser mayor que la fecha hasta".into(),
        ));
    }

    Ok(EstadisticaRepository::clase_mas_concurrida(db, desde, hasta).await?)
}

pub async fn obtener_clase_mas_cancelada(
    db: &SqlitePool,
    desde: NaiveDate,
    hasta: NaiveDate,
) -> Result<ClaseMasCancelada, AppError> {
    if desde > hasta {
        return Err(AppError::Validation(
            "la fecha desde no puede ser mayor que la fecha hasta".into(),
        ));
    }

    Ok(EstadisticaRepository::clase_mas_cancelada(db, desde, hasta).await?)
}

pub async fn obtener_recaudacion(
    db: &SqlitePool,
    desde: NaiveDate,
    hasta: NaiveDate,
) -> Result<Recaudacion, AppError> {
    if desde > hasta {
        return Err(AppError::Validation(
            "la fecha desde no puede ser mayor que la fecha hasta".into(),
        ));
    }

    Ok(EstadisticaRepository::recaudacion(db, desde, hasta).await?)
}
