use sqlx::SqlitePool;

use crate::app::DbError;
use crate::auth::password::hash_password;

pub async fn seed_database(pool: &SqlitePool) -> Result<(), DbError> {
    // Hash de passwords
    let duenio_password = hash_password("123456")?;
    let empleado_password = hash_password("123456")?;

    // SALAS
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO sala (
            id_sala,
            numero,
            capacidad_maxima
        )
        VALUES
            ('SALA01', 1, 40),
            ('SALA02', 2, 25);
        "#,
    )
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // PROFESORES
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO profesor (
            dni_profesor,
            nombre,
            genero,
            estado
        )
        VALUES
            (40123456, 'juan perez', 'masculino', 'alta'),
            (38999888, 'maria gomez', 'femenino', 'alta');
        "#,
    )
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // ACTIVIDADES
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO actividad (
            id_actividad,
            nombre,
            descripcion
        )
        VALUES
            (
                'ACT001',
                'yoga',
                'clases de yoga para mejorar flexibilidad y relajacion'
            ),
            (
                'ACT002',
                'crossfit',
                'entrenamiento funcional de alta intensidad'
            );
        "#,
    )
    .execute(pool)
    .await
    .map_err(DbError::from)?;

    // EMPLEADOS
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO empleado (
            dni_empleado,
            nombre_apellido,
            mail,
            password,
            genero,
            estado,
            rol
        )
        VALUES
            (?, ?, ?, ?, ?, ?, ?),
            (?, ?, ?, ?, ?, ?, ?);
        "#,
    )
    .bind(30111222_i64)
    .bind("carlos admin")
    .bind("admin@gym.com")
    .bind(duenio_password)
    .bind("masculino")
    .bind("alta")
    .bind("duenio")
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

    Ok(())
}
