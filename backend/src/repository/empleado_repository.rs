use sqlx::SqlitePool;

use crate::domain::Empleado;
use crate::errors::ApiError;

pub struct EmpleadoRepository;

impl EmpleadoRepository {
    pub async fn create_empleado(
        pool: &SqlitePool,
        empleado: &Empleado,
    ) -> Result<Empleado, ApiError> {
        todo!()
    }
    pub async fn get_empleados(pool: &SqlitePool) -> Result<Vec<Empleado>, ApiError> {
        todo!()
    }
    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<Empleado, ApiError> {
        todo!()
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: i64) -> Result<Empleado, ApiError> {
        todo!()
    }
    pub async fn update_empleado(
        pool: &SqlitePool,
        dni: i64,
        empleado: &Empleado,
    ) -> Result<Empleado, ApiError> {
        todo!()
    }
    pub async fn delete_empleado(pool: &SqlitePool, dni: i64) -> Result<Empleado, ApiError> {
        todo!()
    }
}
