use sqlx::SqlitePool;

use crate::models::cliente::Cliente;

pub async fn crear_cliente(pool: &SqlitePool, cliente: Cliente) -> Result<Cliente, sqlx::Error> {
    sqlx::query(
        "INSERT INTO Cliente
        (DNI, nombre,apellido, email, telefono, fechaNacimiento, estado, Ficha)
        VALUES (?,?,?,?,?,?, ?,?)",
    )
    .bind(cliente.dni)
    .bind(&cliente.nombre)
    .bind(&cliente.email)
    .bind(&cliente.telefono)
    .bind(&cliente.fecha_nacimiento)
    .bind(&cliente.estado)
    .bind(&cliente.ficha)
    .execute(pool)
    .await?;
    let resultado =
        sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE id = last_insert_rowid()")
            .fetch_one(pool)
            .await?;
    Ok(resultado)
}
