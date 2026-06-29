use crate::reserva::estado::EstadoReserva;
use crate::usuarios::cliente;
use crate::usuarios::cliente::repository::ClienteRepository;
use crate::{
    app::errors::{AppError, FieldError},
    clase, lista_espera,
    reserva::{domain::Reserva, dto::CreateReservaRequest, repository::ReservaRepository},
};
use chrono::{Local, NaiveDateTime, NaiveTime};
use sqlx::SqlitePool;

pub async fn create(db: &SqlitePool, request: CreateReservaRequest) -> Result<Reserva, AppError> {
    //Validar si no existe una reserva para la misma actividad para ese mismo cliente
    let reserva = Reserva::from(request);
    let errors: Vec<FieldError> = reserva
        .validate_reserva()
        .into_iter()
        .map(FieldError::from)
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    let reservas_existentes = ReservaRepository::get_all(db)
        .await
        .map_err(AppError::from)?;
    if reservas_existentes.iter().any(|r| {
        r.get_id_clase() == reserva.get_id_clase()
            && r.get_dni_cliente() == reserva.get_dni_cliente()
    }) {
        return Err(AppError::Conflict(
            "Ya existe una reserva para esta actividad y cliente".to_string(),
        ));
    }
    //Descontar cupo de la clase
    clase::service::aumentar_inscripciones(db, &reserva.get_id_clase()).await?;
    //Guardar la reserva
    ReservaRepository::create(db, &reserva)
        .await
        .map_err(AppError::from)?;
    Ok(reserva)
}

pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<Reserva, AppError> {
    ReservaRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<Reserva>, AppError> {
    ReservaRepository::get_all(db).await.map_err(AppError::from)
}

pub async fn update(
    db: &SqlitePool,
    id: &str,
    request: CreateReservaRequest,
) -> Result<Reserva, AppError> {
    let reserva = Reserva::from(request);
    let errors: Vec<FieldError> = reserva
        .validate_reserva()
        .into_iter()
        .map(FieldError::from)
        .collect();
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    ReservaRepository::update(db, id, &reserva)
        .await
        .map_err(AppError::from)
}
async fn liberar_cupo_y_lista_espera(db: &SqlitePool, id_clase: &str) -> Result<(), AppError> {
    // buscar lista asociada a la clase
    let lista = match lista_espera::service::get_by_clase(db, id_clase).await {
        Ok(lista) => lista,

        // si no existe lista libero el cupo
        Err(_) => {
            clase::service::decrementar_inscripciones(db, &id_clase).await?;
            return Ok(());
        }
    };
    //tomo el proximo cliente
    let cliente = lista_espera::cliente_espera::service::get_next(db, lista.get_id_lista()).await?;
    //si no hay
    match cliente {
        None => {
            clase::service::decrementar_inscripciones(db, &id_clase).await?;
        }
        //si hay le creouna reserva y lo borro de lista espera
        Some(cliente) => {
            let request = CreateReservaRequest {
                fecha: Local::now().date_naive(),
                tipo: "ListaEspera".to_string(),
                estado: EstadoReserva::Pendiente,
                dni_cliente: cliente.get_dni_cliente(),
                id_clase: id_clase.to_string(),
            };

            create(db, request).await?;

            lista_espera::cliente_espera::service::delete(
                db,
                cliente.get_id_espera(),
                cliente.get_dni_cliente(),
            )
            .await?;
        }
    }
    Ok(())
}
pub async fn delete_reserva(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    let reserva = ReservaRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)?;

    let clase = clase::service::get_by_id(db, &reserva.get_id_clase()).await?;

    // calcular si faltan 24 hs
    let hora = NaiveTime::parse_from_str(clase.get_horario(), "%H:%M")
        .map_err(|_| AppError::Conflict("Horario de la clase inválido".to_string()))?;

    let fecha_hora_clase = NaiveDateTime::new(clase.get_dia(), hora);
    let ahora = Local::now().naive_local();

    let tiene_24hs = fecha_hora_clase.signed_duration_since(ahora).num_hours() >= 24;

    registrar_cancelacion(
        db,
        reserva.get_dni_cliente(),
        clase.get_precio(),
        tiene_24hs,
    )
    .await?;
    delete(db, id).await?;
    Ok(())
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    let reserva = ReservaRepository::get_by_id(db, id)
        .await
        .map_err(AppError::from)?;
    let id_clase = reserva.get_id_clase();
    ReservaRepository::delete(db, id)
        .await
        .map_err(AppError::from)?;
    if let Err(error) = liberar_cupo_y_lista_espera(db, &id_clase).await {
        tracing::error!(
            "Error al procesar lista de espera para clase {}: {:?}",
            id_clase,
            error
        );
    }
    Ok(())
}
pub async fn delete_all_by_client(db: &SqlitePool, id: i64) -> Result<(), AppError> {
    let mut reservas = get_all(db).await.map_err(AppError::from)?;
    let mut clases = clase::service::get_all(db).await.map_err(AppError::from)?;

    let reservas_cliente = reservas.iter_mut().filter(|r| r.get_dni_cliente() == id);
    let clases_cliente = clases.iter_mut().filter(|c| c.get_dni_profesor() == id);

    for reserva in reservas_cliente {
        delete(db, reserva.get_id()).await?;
    }
    for clase in clases_cliente {
        clase::service::decrementar_inscripciones(db, clase.get_id()).await?;
    }
    Ok(())
}
async fn registrar_cancelacion(
    db: &SqlitePool,
    dni: i64,
    monto: i64,
    tiene_24hs: bool,
) -> Result<(), AppError> {
    let mut cliente = ClienteRepository::get_by_dni(db, dni)
        .await
        .map_err(AppError::from)?;
    cliente.incrementar_cancelaciones();
    let cancelaciones = cliente.get_contador_cancelaciones();

    if cancelaciones >= 3 {
        cliente.anular_cancelaciones_y_creditos();
        ClienteRepository::update_creditos_y_cancelaciones(
            db,
            cliente.get_dni(),
            cliente.get_creditos(),
            cliente.get_contador_cancelaciones(),
        )
        .await
        .map_err(AppError::from)?;
        return Ok(());
    }
    if !tiene_24hs {
        ClienteRepository::update_creditos_y_cancelaciones(
            db,
            cliente.get_dni(),
            cliente.get_creditos(),
            cliente.get_contador_cancelaciones(),
        )
        .await
        .map_err(AppError::from)?;
        return Ok(());
    }

    //calculo creditos
    let credito = match cancelaciones {
        1 => monto,
        2 => monto / 4, //seria como 25%
        _ => 0,
    };
    cliente.acreditar_creditos(credito);

    ClienteRepository::update_creditos_y_cancelaciones(
        db,
        cliente.get_dni(),
        cliente.get_creditos(),
        cliente.get_contador_cancelaciones(),
    )
    .await
    .map_err(AppError::from)?;
    Ok(())
}
