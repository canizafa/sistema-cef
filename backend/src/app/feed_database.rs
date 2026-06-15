use crate::app::errors::DbError;
use crate::auth::password::hash_password;
use sqlx::SqlitePool;

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

    tracing::info!("Base de datos cargada exitosamente");

    Ok(())
}
