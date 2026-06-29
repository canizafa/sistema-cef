use crate::app::errors::{AppError, DbError};
use crate::auth::password::hash_password;
use crate::clase::dto::CreateClaseRequest;
use crate::clase::estado::EstadoClase;
use chrono::NaiveDate;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn seed_database(pool: &SqlitePool) -> Result<(), DbError> {
    // Hash de passwords
    let duenio_password = hash_password("123456").map_err(|_| DbError::ConnectionError)?;
    let empleado_password = hash_password("123456").map_err(|_| DbError::ConnectionError)?;

    tracing::info!("Llenando la base de datos");

    // SALAS
    sqlx::query("INSERT OR IGNORE INTO sala (id_sala, numero, capacidad_maxima) VALUES (?, ?, ?)")
        .bind("SALA01")
        .bind(1_i64)
        .bind(40_i64)
        .execute(pool)
        .await
        .map_err(DbError::from)?;

    sqlx::query("INSERT OR IGNORE INTO sala (id_sala, numero, capacidad_maxima) VALUES (?, ?, ?)")
        .bind("SALA02")
        .bind(2_i64)
        .bind(25_i64)
        .execute(pool)
        .await
        .map_err(DbError::from)?;

    // PROFESORES
    sqlx::query(
        "INSERT OR IGNORE INTO profesor (dni_profesor, nombre, genero, estado) VALUES (?, ?, ?, ?)",
    )
    .bind(40123456_i64)
    .bind("juan perez")
    .bind("masculino")
    .bind("alta")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    sqlx::query(
        "INSERT OR IGNORE INTO profesor (dni_profesor, nombre, genero, estado) VALUES (?, ?, ?, ?)",
    )
    .bind(38999888_i64)
    .bind("maria gomez")
    .bind("femenino")
    .bind("alta")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // ACTIVIDADES
    sqlx::query(
        "INSERT OR IGNORE INTO actividad (id_actividad, nombre, descripcion) VALUES (?, ?, ?)",
    )
    .bind("ACT001")
    .bind("yoga")
    .bind("clases de yoga para mejorar flexibilidad y relajacion")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    sqlx::query(
        "INSERT OR IGNORE INTO actividad (id_actividad, nombre, descripcion) VALUES (?, ?, ?)",
    )
    .bind("ACT002")
    .bind("crossfit")
    .bind("entrenamiento funcional de alta intensidad")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // EMPLEADOS
    sqlx::query(
        "INSERT OR IGNORE INTO empleado (dni_empleado, nombre_apellido, mail, password, genero, estado, rol) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(30111222_i64)
    .bind("carlos admin")
    .bind("admin@gym.com")
    .bind(duenio_password.clone())
    .bind("masculino")
    .bind("alta")
    .bind("duenio")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    sqlx::query(
        "INSERT OR IGNORE INTO empleado (dni_empleado, nombre_apellido, mail, password, genero, estado, rol) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(33444555_i64)
    .bind("laura empleado")
    .bind("empleado@gym.com")
    .bind(empleado_password)
    .bind("femenino")
    .bind("alta")
    .bind("empleado")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // CLASE YOGA (cupo 1)
    let request = CreateClaseRequest {
        dia: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        horario: "09:00".to_string(),
        cupo_base: 1,
        estado: EstadoClase::Alta,
        id_actividad: "ACT001".to_string(),
        id_sala: "SALA02".to_string(),
        dni_profesor: 40123456,
        descripcion: "Clase de yoga para principiantes".to_string(),
    };

    let _ = crate::clase::service::create(pool, request, "CLASE001").await;

    // CLASE CROSSFIT (cupo 10)
    let request = CreateClaseRequest {
        dia: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        horario: "18:00".to_string(),
        cupo_base: 10,
        estado: EstadoClase::Alta,
        id_actividad: "ACT002".to_string(),
        id_sala: "SALA01".to_string(),
        dni_profesor: 38999888,
        descripcion: "Entrenamiento funcional de alta intensidad".to_string(),
    };

    let _ = crate::clase::service::create(pool, request, "CLASE002").await;

    // CLIENTE
    sqlx::query(
        "INSERT OR IGNORE INTO cliente (
            dni_cliente,
            nombre_completo,
            email,
            telefono,
            fecha_nacimiento
        ) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(40111222_i64)
    .bind("Ana Martinez")
    .bind("ana@gmail.com")
    .bind("2215551234")
    .bind("1995-05-10")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // FICHA MÉDICA 1
    sqlx::query(
        "INSERT OR IGNORE INTO ficha_medica (
            id_ficha,
            enfermedades,
            operaciones_quirurgicas,
            detalles
        ) VALUES (?, ?, ?, ?)",
    )
    .bind("FICHA001")
    .bind(false)
    .bind(false)
    .bind(None::<String>)
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // FICHA MÉDICA 2
    sqlx::query(
        "INSERT OR IGNORE INTO ficha_medica (
            id_ficha,
            enfermedades,
            operaciones_quirurgicas,
            detalles
        ) VALUES (?, ?, ?, ?)",
    )
    .bind("FICHA002")
    .bind(true)
    .bind(false)
    .bind(Some("Asma leve"))
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    let cliente1_password = hash_password("123456").map_err(|_| DbError::ConnectionError)?;
    let cliente2_password = hash_password("123456").map_err(|_| DbError::ConnectionError)?;

    // CLIENTE 1
    sqlx::query(
        "INSERT OR IGNORE INTO cliente (
            dni_cliente,
            nombre_completo,
            email,
            telefono,
            fecha_nacimiento,
            estado,
            password,
            motivo_eliminacion,
            id_ficha
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(40111222_i64)
    .bind("Ana Martinez")
    .bind("ana@gmail.com")
    .bind("2215551234")
    .bind("1995-05-10")
    .bind("alta")
    .bind(cliente1_password)
    .bind(None::<String>)
    .bind("FICHA001")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // CLIENTE 2
    sqlx::query(
        "INSERT OR IGNORE INTO cliente (
            dni_cliente,
            nombre_completo,
            email,
            telefono,
            fecha_nacimiento,
            estado,
            password,
            motivo_eliminacion,
            id_ficha
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(40222333_i64)
    .bind("Pedro Lopez")
    .bind("pedro@gmail.com")
    .bind("2215555678")
    .bind("1990-08-15")
    .bind("alta")
    .bind(cliente2_password)
    .bind(None::<String>)
    .bind("FICHA002")
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    tracing::info!("Base de datos cargada exitosamente");

    Ok(())
}
